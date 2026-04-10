use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use liblzma::read;
use liblzma::stream;
use liblzma::write;

#[cfg(feature = "xz")]
const BACKEND_NAME: &str = "xz";
#[cfg(feature = "xz-sys")]
const BACKEND_NAME: &str = "xz-sys";
#[cfg(feature = "liblzma-sys")]
const BACKEND_NAME: &str = "liblzma-sys";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    All,
    Good,
    Bad,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    Both,
    Read,
    Write,
}

#[derive(Debug)]
struct Config {
    corpus_dir: PathBuf,
    mode: Mode,
    phase: Phase,
    name_pattern: Option<String>,
    iters: usize,
    warmup: usize,
}

#[derive(Debug)]
struct Case {
    name: String,
    data: Vec<u8>,
    is_bad: bool,
    decoded: Option<Vec<u8>>,
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
    let cases = load_cases(&config).unwrap_or_else(|err| {
        eprintln!("failed to load corpus: {err}");
        std::process::exit(1);
    });
    let total_bytes: usize = cases.iter().map(|case| case.data.len()).sum();

    println!(
        "config: backend={} mode={:?} phase={:?} cases={} bytes={} iters={} warmup={}",
        BACKEND_NAME,
        config.mode,
        config.phase,
        cases.len(),
        total_bytes,
        config.iters,
        config.warmup
    );
    if let Some(pattern) = &config.name_pattern {
        println!("name_pattern: {pattern}");
    }
    if !cases.is_empty() {
        let case_names = cases
            .iter()
            .map(|case| case.name.as_str())
            .collect::<Vec<_>>()
            .join(",");
        println!("cases: {case_names}");
    }

    let measurement = measure(total_bytes, config.iters, config.warmup, || {
        run_cases(black_box(&cases), config.phase)
    });
    print_measurement(&measurement, config.iters);
}

impl Config {
    fn parse<I>(mut args: I) -> Result<Self, String>
    where
        I: Iterator<Item = String>,
    {
        let mut config = Self {
            corpus_dir: PathBuf::from("liblzma-sys/xz/tests/files"),
            mode: Mode::All,
            phase: Phase::Both,
            name_pattern: None,
            iters: 200,
            warmup: 20,
        };

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--corpus-dir" => {
                    config.corpus_dir = PathBuf::from(next_arg(&mut args, "--corpus-dir")?);
                }
                "--mode" => {
                    config.mode = parse_mode(next_arg(&mut args, "--mode")?)?;
                }
                "--phase" => {
                    config.phase = parse_phase(next_arg(&mut args, "--phase")?)?;
                }
                "--name-pattern" => {
                    config.name_pattern = Some(next_arg(&mut args, "--name-pattern")?);
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
        "  cargo run --example standard_files_probe --release -- [options]\n\
         \n\
         To use the C backend instead:\n\
           cargo run --example standard_files_probe --release --no-default-features --features liblzma-sys -- [options]\n\n",
    );
    message.push_str("Options:\n");
    message.push_str("  --corpus-dir <path>   XZ test corpus directory\n");
    message.push_str("  --mode <all|good|bad>\n");
    message.push_str("  --phase <both|read|write>\n");
    message.push_str("  --name-pattern <substring>\n");
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

fn parse_usize(value: String, flag: &str) -> Result<usize, String> {
    value
        .parse()
        .map_err(|_| format!("failed to parse `{flag}` as usize: `{value}`"))
}

fn parse_mode(value: String) -> Result<Mode, String> {
    match value.as_str() {
        "all" => Ok(Mode::All),
        "good" => Ok(Mode::Good),
        "bad" => Ok(Mode::Bad),
        _ => Err(format!("unsupported mode `{value}`")),
    }
}

fn parse_phase(value: String) -> Result<Phase, String> {
    match value.as_str() {
        "both" => Ok(Phase::Both),
        "read" => Ok(Phase::Read),
        "write" => Ok(Phase::Write),
        _ => Err(format!("unsupported phase `{value}`")),
    }
}

fn load_cases(config: &Config) -> Result<Vec<Case>, std::io::Error> {
    let mut cases = Vec::new();
    for entry in fs::read_dir(&config.corpus_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("xz") {
            continue;
        }

        let filename = entry.file_name().into_string().unwrap();
        if filename.contains("unsupported-check") {
            continue;
        }
        if let Some(pattern) = &config.name_pattern {
            if !filename.contains(pattern) {
                continue;
            }
        }

        let is_bad = filename.starts_with("bad") || filename.starts_with("unsupported");
        if (config.mode == Mode::Good && is_bad) || (config.mode == Mode::Bad && !is_bad) {
            continue;
        }

        let mut data = Vec::new();
        File::open(&path)?.read_to_end(&mut data)?;
        let decoded = if !is_bad && config.phase == Phase::Write {
            Some(decode_good(&data))
        } else {
            None
        };
        cases.push(Case {
            name: filename,
            data,
            is_bad,
            decoded,
        });
    }

    cases.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(cases)
}

fn run_cases(cases: &[Case], phase: Phase) -> u64 {
    let mut digest = 0u64;
    for case in cases {
        digest = digest.rotate_left(7) ^ run_case(case, phase);
    }
    digest
}

fn run_case(case: &Case, phase: Phase) -> u64 {
    if case.is_bad {
        run_bad(&case.data)
    } else {
        run_good(case, phase)
    }
}

fn decode_good(data: &[u8]) -> Vec<u8> {
    let mut ret = Vec::new();
    read::XzDecoder::new_multi_decoder(data)
        .read_to_end(&mut ret)
        .unwrap();
    ret
}

fn run_good(case: &Case, phase: Phase) -> u64 {
    match phase {
        Phase::Both => run_good_both(&case.data),
        Phase::Read => {
            let ret = decode_good(&case.data);
            fold_bytes(ret.len(), &ret)
        }
        Phase::Write => run_good_write(&case.data, case.decoded.as_deref().unwrap()),
    }
}

fn run_good_both(data: &[u8]) -> u64 {
    let ret = decode_good(data);
    run_good_write(data, &ret)
}

fn run_good_write(data: &[u8], decoded_seed: &[u8]) -> u64 {
    let ret = decoded_seed.to_vec();
    let mut w = write::XzDecoder::new_multi_decoder(ret);
    w.write_all(data).unwrap();
    let ret = w.finish().unwrap();
    fold_bytes(data.len() + ret.len(), &ret)
}

fn run_bad(data: &[u8]) -> u64 {
    let mut ret = Vec::new();
    let stream = stream::Stream::new_stream_decoder(u64::MAX, stream::CONCATENATED).unwrap();
    let result = read::XzDecoder::new_stream(data, stream).read_to_end(&mut ret);
    assert!(result.is_err(), "{result:?}");
    let mut w = write::XzDecoder::new(ret);
    assert!(w.write_all(data).is_err() || w.finish().is_err());
    fold_bytes(data.len(), data)
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
