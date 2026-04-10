use std::env;
use std::io::Read;
use std::time::{Duration, Instant};

use liblzma::bufread;

#[cfg(feature = "xz")]
const BACKEND_NAME: &str = "xz";
#[cfg(feature = "xz-sys")]
const BACKEND_NAME: &str = "xz-sys";
#[cfg(feature = "liblzma-sys")]
const BACKEND_NAME: &str = "liblzma-sys";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Encode,
    Decode,
    Both,
}

#[derive(Debug)]
struct Config {
    mode: Mode,
    input_size: usize,
    trailing_size: usize,
    level: u32,
    iters: usize,
    warmup: usize,
}

#[derive(Debug)]
struct Measurement {
    elapsed: Duration,
    throughput_mib_s: f64,
    digest: u64,
}

#[inline(always)]
fn black_box<T>(value: T) -> T {
    let value = std::mem::ManuallyDrop::new(value);
    unsafe { std::ptr::read_volatile(&*value) }
}

fn main() {
    let config = Config::parse(env::args().skip(1)).unwrap_or_else(|message| {
        eprintln!("{message}");
        std::process::exit(2);
    });
    let input = build_input(config.input_size);
    let trailing = build_trailing(config.trailing_size);
    let decode_case = build_decode_case(&input, &trailing, config.level);

    println!(
        "config: backend={} mode={:?} input_size={} trailing_size={} level={} iters={} warmup={}",
        BACKEND_NAME,
        config.mode,
        config.input_size,
        config.trailing_size,
        config.level,
        config.iters,
        config.warmup
    );

    let bytes_per_iter = match config.mode {
        Mode::Encode => input.len(),
        Mode::Decode => input.len() + trailing.len(),
        Mode::Both => input.len() * 2 + trailing.len(),
    };
    let measurement = measure(bytes_per_iter, config.iters, config.warmup, || {
        run_case(
            black_box(&input),
            black_box(&trailing),
            black_box(&decode_case),
            config.mode,
            config.level,
        )
    });
    print_measurement(&measurement, config.iters);
}

impl Config {
    fn parse<I>(mut args: I) -> Result<Self, String>
    where
        I: Iterator<Item = String>,
    {
        let mut config = Self {
            mode: Mode::Both,
            input_size: 1024,
            trailing_size: 123,
            level: 6,
            iters: 1000,
            warmup: 100,
        };

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--mode" => {
                    config.mode = parse_mode(next_arg(&mut args, "--mode")?)?;
                }
                "--input-size" => {
                    config.input_size =
                        parse_usize(next_arg(&mut args, "--input-size")?, "--input-size")?;
                }
                "--trailing-size" => {
                    config.trailing_size =
                        parse_usize(next_arg(&mut args, "--trailing-size")?, "--trailing-size")?;
                }
                "--level" => {
                    config.level = parse_u32(next_arg(&mut args, "--level")?, "--level")?;
                }
                "--iters" => {
                    config.iters = parse_usize(next_arg(&mut args, "--iters")?, "--iters")?;
                }
                "--warmup" => {
                    config.warmup = parse_usize(next_arg(&mut args, "--warmup")?, "--warmup")?;
                }
                "--help" | "-h" => return Err(usage()),
                unknown => return Err(format!("unknown argument `{unknown}`\n\n{}", usage())),
            }
        }

        if config.input_size == 0 {
            return Err("`--input-size` must be greater than zero".to_owned());
        }
        if config.iters == 0 {
            return Err("`--iters` must be greater than zero".to_owned());
        }

        Ok(config)
    }
}

fn usage() -> String {
    let mut message = String::new();
    message.push_str("Usage:\n");
    message.push_str(
        "  cargo run --example bufread_trailing_probe --release -- [options]\n\
         \n\
         To use the C backend instead:\n\
           cargo run --example bufread_trailing_probe --release --no-default-features --features liblzma-sys -- [options]\n\n",
    );
    message.push_str("Options:\n");
    message.push_str("  --mode <encode|decode|both>\n");
    message.push_str("  --input-size <bytes>\n");
    message.push_str("  --trailing-size <bytes>\n");
    message.push_str("  --level <preset>\n");
    message.push_str("  --iters <n>\n");
    message.push_str("  --warmup <n>\n");
    message
}

fn next_arg<I>(args: &mut I, flag: &str) -> Result<String, String>
where
    I: Iterator<Item = String>,
{
    args.next()
        .ok_or_else(|| format!("missing value for `{flag}`\n\n{}", usage()))
}

fn parse_mode(value: String) -> Result<Mode, String> {
    match value.as_str() {
        "encode" => Ok(Mode::Encode),
        "decode" => Ok(Mode::Decode),
        "both" => Ok(Mode::Both),
        _ => Err(format!("unsupported mode `{value}`")),
    }
}

fn parse_usize(value: String, flag: &str) -> Result<usize, String> {
    value
        .parse()
        .map_err(|_| format!("failed to parse `{flag}` as usize: `{value}`"))
}

fn parse_u32(value: String, flag: &str) -> Result<u32, String> {
    value
        .parse()
        .map_err(|_| format!("failed to parse `{flag}` as u32: `{value}`"))
}

fn build_input(len: usize) -> Vec<u8> {
    (0..len).map(|num| num as u8).collect()
}

fn build_trailing(len: usize) -> Vec<u8> {
    (0..len).map(|num| ((25 + num) % 256) as u8).collect()
}

fn build_decode_case(input: &[u8], trailing: &[u8], level: u32) -> Vec<u8> {
    let mut encoder = bufread::XzEncoder::new(input, level);
    let mut decoder_input = Vec::new();
    encoder.read_to_end(&mut decoder_input).unwrap();
    decoder_input.extend_from_slice(trailing);
    decoder_input
}

fn run_case(input: &[u8], trailing: &[u8], decode_case: &[u8], mode: Mode, level: u32) -> u64 {
    match mode {
        Mode::Encode => run_encode_case(input, level),
        Mode::Decode => run_decode_case(input, trailing, decode_case),
        Mode::Both => {
            let encoded = run_encode_to_vec(input, level);
            let mut decode_input = Vec::with_capacity(encoded.len() + trailing.len());
            decode_input.extend_from_slice(&encoded);
            decode_input.extend_from_slice(trailing);
            fold_bytes(encoded.len(), &encoded) ^ run_decode_case(input, trailing, &decode_input)
        }
    }
}

fn run_encode_case(input: &[u8], level: u32) -> u64 {
    let encoded = run_encode_to_vec(input, level);
    fold_bytes(encoded.len(), &encoded)
}

fn run_encode_to_vec(input: &[u8], level: u32) -> Vec<u8> {
    let mut encoder = bufread::XzEncoder::new(input, level);
    let mut encoded = Vec::new();
    encoder.read_to_end(&mut encoded).unwrap();
    encoded
}

fn run_decode_case(input: &[u8], trailing: &[u8], decode_case: &[u8]) -> u64 {
    let mut decoder_reader = decode_case;
    let mut decoder = bufread::XzDecoder::new(&mut decoder_reader);
    let mut decompressed = vec![0u8; input.len()];
    let read = decoder.read(&mut decompressed).unwrap();
    assert_eq!(read, input.len());
    assert_eq!(decompressed, input);
    assert_eq!(decoder.total_out(), input.len() as u64);

    let mut remaining = Vec::new();
    let trailing_read = decoder_reader.read_to_end(&mut remaining).unwrap();
    assert_eq!(trailing_read, trailing.len());
    assert_eq!(remaining, trailing);

    fold_bytes(input.len() + remaining.len(), &decompressed)
        ^ fold_bytes(remaining.len(), &remaining)
}

fn measure<F>(bytes_per_iter: usize, iters: usize, warmup: usize, mut work: F) -> Measurement
where
    F: FnMut() -> u64,
{
    let mut digest = 0u64;
    for _ in 0..warmup {
        digest ^= black_box(work());
    }

    let start = Instant::now();
    for _ in 0..iters {
        digest ^= black_box(work());
    }
    let elapsed = start.elapsed();
    let mib = (bytes_per_iter as f64 * iters as f64) / (1024.0 * 1024.0);

    Measurement {
        elapsed,
        throughput_mib_s: mib / elapsed.as_secs_f64(),
        digest,
    }
}

fn print_measurement(measurement: &Measurement, iters: usize) {
    println!(
        "{}: total={:.3?} ns_per_iter={:.0} throughput_mib_s={:.2} digest={:#x}",
        BACKEND_NAME,
        measurement.elapsed,
        measurement.elapsed.as_nanos() as f64 / iters as f64,
        measurement.throughput_mib_s,
        measurement.digest
    );
}

fn fold_bytes(len: usize, data: &[u8]) -> u64 {
    let mut acc = len as u64;
    for &byte in data.iter().take(32) {
        acc = acc.rotate_left(5) ^ u64::from(byte);
    }
    acc
}
