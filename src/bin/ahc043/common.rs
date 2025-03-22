pub fn get_time() -> f64 {
    static mut STIME: Option<std::time::Instant> = None;
    unsafe {
        let now = std::time::Instant::now();
        if STIME.is_none() {
            STIME = Some(now);
        }
        now.duration_since(STIME.unwrap()).as_secs_f64()
    }
}
