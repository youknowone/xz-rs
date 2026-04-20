use std::env;
use std::io::{Read, Write};
use std::time::{Duration, Instant};

use xz::read;
use xz::write;

#[cfg(feature = "xz-core")]
const BACKEND_NAME: &str = "xz-core";
#[cfg(feature = "xz-sys")]
const BACKEND_NAME: &str = "xz-sys";
#[cfg(feature = "liblzma-sys")]
const BACKEND_NAME: &str = "liblzma-sys";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Read,
    Write,
    Both,
}

#[derive(Debug)]
struct Config {
    mode: Mode,
    cases: usize,
    max_size: usize,
    seed: u64,
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
    let cases = build_cases(config.cases, config.max_size, config.seed);
    let total_bytes: usize = cases.iter().map(Vec::len).sum();

    println!(
        "config: backend={} mode={:?} cases={} max_size={} bytes={} seed={} iters={} warmup={}",
        BACKEND_NAME,
        config.mode,
        cases.len(),
        config.max_size,
        total_bytes,
        config.seed,
        config.iters,
        config.warmup
    );

    let bytes_per_iter = total_bytes
        * match config.mode {
            Mode::Read | Mode::Write => 1,
            Mode::Both => 2,
        };
    let measurement = measure(bytes_per_iter, config.iters, config.warmup, || {
        run_cases(black_box(&cases), config.mode)
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
            cases: 128,
            max_size: 4096,
            seed: 0xC0DEC0FFEE,
            iters: 200,
            warmup: 20,
        };

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--mode" => {
                    config.mode = parse_mode(next_arg(&mut args, "--mode")?)?;
                }
                "--cases" => {
                    config.cases = parse_usize(next_arg(&mut args, "--cases")?, "--cases")?;
                }
                "--max-size" => {
                    config.max_size =
                        parse_usize(next_arg(&mut args, "--max-size")?, "--max-size")?;
                }
                "--seed" => {
                    config.seed = parse_u64(next_arg(&mut args, "--seed")?, "--seed")?;
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

        if config.cases == 0 {
            return Err("`--cases` must be greater than zero".to_owned());
        }
        if config.max_size == 0 {
            return Err("`--max-size` must be greater than zero".to_owned());
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
        "  cargo run --example qc_probe --release -- [options]\n\
         \n\
         To use the C backend instead:\n\
           cargo run --example qc_probe --release --no-default-features --features liblzma-sys -- [options]\n\n",
    );
    message.push_str("Options:\n");
    message.push_str("  --mode <read|write|both>\n");
    message.push_str("  --cases <n>\n");
    message.push_str("  --max-size <bytes>\n");
    message.push_str("  --seed <u64>\n");
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
        "read" => Ok(Mode::Read),
        "write" => Ok(Mode::Write),
        "both" => Ok(Mode::Both),
        _ => Err(format!("unsupported mode `{value}`")),
    }
}

fn parse_usize(value: String, flag: &str) -> Result<usize, String> {
    value
        .parse()
        .map_err(|_| format!("failed to parse `{flag}` as usize: `{value}`"))
}

fn parse_u64(value: String, flag: &str) -> Result<u64, String> {
    value
        .parse()
        .map_err(|_| format!("failed to parse `{flag}` as u64: `{value}`"))
}

fn build_cases(count: usize, max_size: usize, seed: u64) -> Vec<Vec<u8>> {
    let mut rng = seed;
    let mut cases = Vec::with_capacity(count);
    for i in 0..count {
        rng = next_u64(rng);
        let len = 1 + ((rng as usize ^ i.wrapping_mul(131)) % max_size);
        let mut case = Vec::with_capacity(len);
        for _ in 0..len {
            rng = next_u64(rng);
            case.push((rng >> 56) as u8);
        }
        cases.push(case);
    }
    cases
}

fn next_u64(mut x: u64) -> u64 {
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    x.wrapping_mul(0x2545F4914F6CDD1D)
}

fn run_cases(cases: &[Vec<u8>], mode: Mode) -> u64 {
    let mut digest = 0u64;
    for case in cases {
        digest = digest.rotate_left(7)
            ^ match mode {
                Mode::Read => run_read_case(case),
                Mode::Write => run_write_case(case),
                Mode::Both => run_read_case(case) ^ run_write_case(case),
            };
    }
    digest
}

fn run_read_case(input: &[u8]) -> u64 {
    let reader = read::XzEncoder::new(input, 6);
    let mut decoder = read::XzDecoder::new(reader);
    let mut output = Vec::with_capacity(input.len());
    decoder.read_to_end(&mut output).unwrap();
    fold_bytes(output.len(), &output)
}

fn run_write_case(input: &[u8]) -> u64 {
    let inner = write::XzDecoder::new(Vec::with_capacity(input.len()));
    let mut encoder = write::XzEncoder::new(inner, 6);
    encoder.write_all(input).unwrap();
    let output = encoder.finish().unwrap().finish().unwrap();
    fold_bytes(output.len(), &output)
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
