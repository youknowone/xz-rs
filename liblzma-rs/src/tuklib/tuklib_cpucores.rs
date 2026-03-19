pub fn tuklib_cpucores() -> u32 {
    match std::thread::available_parallelism() {
        Ok(n) => n.get() as u32,
        Err(_) => 0,
    }
}
