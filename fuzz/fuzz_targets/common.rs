#![allow(dead_code)]

use std::io::Read;

use xz::read::XzDecoder;
use xz::stream::{Action, Error, Status, Stream, PRESET_EXTREME};

pub const MEM_LIMIT: u64 = 300 << 20;
const CONTROL_BYTES: usize = 8;
const MAX_ENCODE_INPUT: usize = 128 * 1024;

pub fn preset(control: u8) -> u32 {
    match control % 5 {
        0 => 0,
        1 => 1,
        2 => 5,
        3 => PRESET_EXTREME,
        _ => 3 | PRESET_EXTREME,
    }
}

pub fn controls(data: &[u8]) -> &[u8] {
    &data[..data.len().min(CONTROL_BYTES)]
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

    for _ in 0..1024 {
        let mut output = vec![0; output_len(control(controls, control_index))];
        control_index += 1;
        match stream.process(&[], &mut output, Action::Finish) {
            Ok(Status::StreamEnd) => return,
            Ok(Status::Ok | Status::MemNeeded | Status::GetCheck) => {}
            Err(Error::Program) => panic!("decoder returned Program on finish"),
            Err(_) => return,
        }
    }
}
