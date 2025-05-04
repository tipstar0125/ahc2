pub fn get_time() -> f64 {
    static mut STIME: Option<std::time::Instant> = None;
    unsafe {
        if STIME.is_none() {
            STIME = Some(std::time::Instant::now());
        }
        let elapsed = STIME.unwrap().elapsed();
        let ms = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
        #[cfg(feature = "local")]
        {
            ms * 3.0
        }
        #[cfg(not(feature = "local"))]
        {
            ms
        }
    }
}
