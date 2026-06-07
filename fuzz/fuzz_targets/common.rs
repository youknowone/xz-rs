#![allow(dead_code)]

use std::io::Read;

use xz::read::XzDecoder;
use xz::stream::{Action, Check, Error, Filters, LzmaOptions, PRESET_EXTREME, Status, Stream};

pub const MEM_LIMIT: u64 = 300 << 20;
const CONTROL_BYTES: usize = 8;
const MAX_ENCODE_INPUT: usize = 128 * 1024;
const UPSTREAM_IN_CHUNK_SIZE: usize = 2047;
const UPSTREAM_OUT_SIZE: usize = 4096;
const FINISH_NO_PROGRESS_LIMIT: usize = 16;

pub fn preset(control: u8) -> u32 {
    match control {
        0 | 1 | 5 => control as u32,
        6 => PRESET_EXTREME,
        7 => 3 | PRESET_EXTREME,
        _ => match control % 6 {
            0 => 2,
            1 => 3,
            2 => 4,
            3 => 6,
            4 => 7,
            _ => 9 | PRESET_EXTREME,
        },
    }
}

pub fn stream_encoder_from_preset(preset: u32) -> Stream {
    let opts = LzmaOptions::new_preset(preset).unwrap();
    let mut filters = Filters::new();
    filters.lzma2(&opts);
    Stream::new_stream_encoder(&filters, Check::Crc64).unwrap()
}

pub fn controls(data: &[u8]) -> &[u8] {
    &data[..data.len().min(CONTROL_BYTES)]
}

pub fn upstream_encode_input(data: &[u8]) -> &[u8] {
    data.get(1..).unwrap_or_default()
}

pub fn upstream_encode_expected(input: &[u8]) -> Vec<u8> {
    // vendor/xz/tests/ossfuzz/fuzz_common.h feeds the first half once, then
    // resumes chunking from the original input pointer.
    let first_chunk = input.len() / 2;
    let mut expected = Vec::with_capacity(first_chunk + input.len());
    expected.extend_from_slice(&input[..first_chunk]);
    expected.extend_from_slice(input);
    expected
}

pub fn encode_input(data: &[u8]) -> &[u8] {
    let start = data.len().min(CONTROL_BYTES);
    let end = data.len().min(start + MAX_ENCODE_INPUT);
    &data[start..end]
}

fn control(controls: &[u8], index: usize) -> u8 {
    controls
        .get(index % controls.len().max(1))
        .copied()
        .unwrap_or(index as u8)
}

fn output_len(control: u8) -> usize {
    match control % 8 {
        0 => 1,
        1 => 2,
        2 => 3,
        _ => 32 + control as usize,
    }
}

fn chunk_len(control: u8, remaining: usize) -> usize {
    if remaining == 0 {
        return 0;
    }

    let len = match control % 6 {
        0 => 1,
        1 => 2,
        2 => 3,
        3 => 7,
        _ => 1 + (control as usize % remaining.min(4096)),
    };
    len.min(remaining)
}

fn process_encoder_action(
    stream: &mut Stream,
    input: &[u8],
    action: Action,
    controls: &[u8],
    control_index: &mut usize,
    encoded: &mut Vec<u8>,
) -> Status {
    let mut input_pos = 0usize;

    for _ in 0..200_000 {
        let in_before = stream.total_in();
        let out_before = stream.total_out();
        let mut output = vec![0; output_len(control(controls, *control_index))];
        *control_index += 1;

        let status = stream
            .process(&input[input_pos..], &mut output, action)
            .unwrap();

        input_pos += (stream.total_in() - in_before) as usize;
        let output_used = (stream.total_out() - out_before) as usize;
        encoded.extend_from_slice(&output[..output_used]);

        if matches!(action, Action::Run) {
            if input_pos == input.len() {
                return status;
            }
        } else if status == Status::StreamEnd {
            return status;
        }
    }

    panic!("encoder action did not converge: {action:?}");
}

fn process_encoder_upstream_action(
    stream: &mut Stream,
    input: &[u8],
    action: Action,
    encoded: &mut Vec<u8>,
) -> Status {
    let mut input_pos = 0usize;
    let mut no_progress = 0usize;

    for _ in 0..200_000 {
        let in_before = stream.total_in();
        let out_before = stream.total_out();
        let mut output = vec![0; UPSTREAM_OUT_SIZE];

        let status = stream
            .process(&input[input_pos..], &mut output, action)
            .unwrap();

        input_pos += (stream.total_in() - in_before) as usize;
        let output_used = (stream.total_out() - out_before) as usize;
        encoded.extend_from_slice(&output[..output_used]);

        if matches!(action, Action::Run) {
            if input_pos == input.len() {
                return status;
            }
        } else if status == Status::StreamEnd {
            return status;
        }

        if stream.total_in() == in_before && stream.total_out() == out_before {
            no_progress += 1;
            if no_progress > 1 {
                return status;
            }
        } else {
            no_progress = 0;
        }
    }

    panic!("upstream-shaped encoder action did not converge: {action:?}");
}

pub fn encode_like_upstream(mut stream: Stream, input: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::new();
    let first_chunk = input.len() / 2;

    if first_chunk > 0 {
        process_encoder_upstream_action(
            &mut stream,
            &input[..first_chunk],
            Action::Run,
            &mut encoded,
        );
    }

    if input.is_empty() {
        process_encoder_upstream_action(&mut stream, &[], Action::Finish, &mut encoded);
        return encoded;
    }

    let mut input_pos = 0usize;
    while input_pos < input.len() {
        let end = (input_pos + UPSTREAM_IN_CHUNK_SIZE).min(input.len());
        let action = if end == input.len() {
            Action::Finish
        } else {
            Action::Run
        };
        let status = process_encoder_upstream_action(
            &mut stream,
            &input[input_pos..end],
            action,
            &mut encoded,
        );
        if status == Status::StreamEnd {
            return encoded;
        }
        input_pos = end;
    }

    encoded
}

pub fn encode_with_actions(
    mut stream: Stream,
    input: &[u8],
    controls: &[u8],
    allow_sync_flush: bool,
) -> Vec<u8> {
    let mut encoded = Vec::new();
    let mut input_pos = 0usize;
    let mut control_index = 0usize;

    while input_pos < input.len() {
        let chunk = chunk_len(control(controls, control_index), input.len() - input_pos);
        control_index += 1;
        process_encoder_action(
            &mut stream,
            &input[input_pos..input_pos + chunk],
            Action::Run,
            controls,
            &mut control_index,
            &mut encoded,
        );
        input_pos += chunk;

        let flush = control(controls, control_index);
        control_index += 1;
        if flush % 4 == 0 {
            let action = if allow_sync_flush {
                match flush % 3 {
                    0 => Action::SyncFlush,
                    1 => Action::FullFlush,
                    _ => Action::FullBarrier,
                }
            } else if flush % 2 == 0 {
                Action::FullFlush
            } else {
                Action::FullBarrier
            };
            process_encoder_action(
                &mut stream,
                &[],
                action,
                controls,
                &mut control_index,
                &mut encoded,
            );
        }
    }

    process_encoder_action(
        &mut stream,
        &[],
        Action::Finish,
        controls,
        &mut control_index,
        &mut encoded,
    );
    encoded
}

pub fn assert_xz_decodes_to(encoded: &[u8], expected: &[u8]) {
    let mut decoder = XzDecoder::new(encoded);
    let mut decoded = Vec::new();
    decoder.read_to_end(&mut decoded).unwrap();
    assert_eq!(decoded, expected);
}

fn process_decoder_upstream_action(stream: &mut Stream, input: &[u8], action: Action) -> bool {
    let mut input_pos = 0usize;
    let mut no_progress = 0usize;

    for _ in 0..200_000 {
        let in_before = stream.total_in();
        let out_before = stream.total_out();
        let mut output = vec![0; UPSTREAM_OUT_SIZE];

        let status = match stream.process(&input[input_pos..], &mut output, action) {
            Ok(status) => status,
            Err(Error::Program) => panic!("decoder returned Program"),
            Err(_) => return true,
        };

        input_pos += (stream.total_in() - in_before) as usize;
        let made_progress = stream.total_in() != in_before || stream.total_out() != out_before;

        if status == Status::StreamEnd {
            return true;
        }

        if input_pos == input.len() && (matches!(action, Action::Run) || !made_progress) {
            return false;
        }

        if made_progress {
            no_progress = 0;
        } else {
            no_progress += 1;
            if no_progress > 1 {
                return false;
            }
        }
    }

    panic!("upstream-shaped decoder action did not converge: {action:?}");
}

pub fn feed_decoder_like_upstream(mut stream: Stream, input: &[u8]) {
    let first_chunk = input.len() / 2;

    if first_chunk > 0
        && process_decoder_upstream_action(&mut stream, &input[..first_chunk], Action::Run)
    {
        return;
    }

    if input.is_empty() {
        process_decoder_upstream_action(&mut stream, &[], Action::Run);
        return;
    }

    let mut input_pos = 0usize;
    while input_pos < input.len() {
        let end = (input_pos + UPSTREAM_IN_CHUNK_SIZE).min(input.len());
        let action = if end == input.len() {
            Action::Finish
        } else {
            Action::Run
        };
        if process_decoder_upstream_action(&mut stream, &input[input_pos..end], action) {
            return;
        }
        input_pos = end;
    }
}

pub fn feed_decoder(mut stream: Stream, input: &[u8], controls: &[u8]) {
    let mut input_pos = 0usize;
    let mut control_index = 0usize;
    let mut force_all_remaining = false;

    while input_pos < input.len() {
        let chunk = if force_all_remaining {
            input.len() - input_pos
        } else {
            chunk_len(control(controls, control_index), input.len() - input_pos)
        };
        control_index += 1;

        let chunk_end = input_pos + chunk;
        let in_before = stream.total_in();
        let out_before = stream.total_out();
        let mut output = vec![0; output_len(control(controls, control_index))];
        control_index += 1;

        let status = match stream.process(&input[input_pos..chunk_end], &mut output, Action::Run) {
            Ok(status) => status,
            Err(Error::Program) => panic!("decoder returned Program"),
            Err(_) => return,
        };

        input_pos += (stream.total_in() - in_before) as usize;
        let made_progress = stream.total_in() != in_before || stream.total_out() != out_before;

        if status == Status::StreamEnd {
            return;
        }

        if made_progress {
            force_all_remaining = false;
        } else if force_all_remaining {
            break;
        } else {
            force_all_remaining = true;
        }
    }

    // Finish gets no further input, so the decoder can only flush what it
    // already holds and must terminate. Loop until it does instead of giving
    // up after a fixed count. Status::MemNeeded is how LZMA_BUF_ERROR
    // surfaces here, which is liblzma reporting that no further progress is
    // possible, so it ends the stream; an Ok that keeps emitting nothing is a
    // state-machine livelock instead and has to fail loudly.
    let mut no_progress = 0usize;
    loop {
        let out_before = stream.total_out();
        let mut output = vec![0; output_len(control(controls, control_index))];
        control_index += 1;
        match stream.process(&[], &mut output, Action::Finish) {
            Ok(Status::StreamEnd | Status::MemNeeded) => return,
            Ok(Status::Ok | Status::GetCheck) => {}
            Err(Error::Program) => panic!("decoder returned Program on finish"),
            Err(_) => return,
        }

        if stream.total_out() != out_before {
            no_progress = 0;
        } else {
            no_progress += 1;
            assert!(
                no_progress <= FINISH_NO_PROGRESS_LIMIT,
                "decoder made no progress in {FINISH_NO_PROGRESS_LIMIT} consecutive finish calls"
            );
        }
    }
}
