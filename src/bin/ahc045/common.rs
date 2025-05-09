pub fn get_time() -> f64 {
    static mut STIME: Option<std::time::Instant> = None;
    unsafe {
        #[allow(static_mut_refs)]
        if STIME.is_none() {
            STIME = Some(std::time::Instant::now());
        }
        let elapsed = STIME.unwrap().elapsed();
        let ms = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
        #[cfg(feature = "local")]
        {
            ms * 1.3
        }
        #[cfg(not(feature = "local"))]
        {
            ms
        }
    }
}

pub fn eprint_red(message: &str) {
    eprintln!("\x1b[31m{}\x1b[0m", message);
}

pub fn eprint_green(message: &str) {
    eprintln!("\x1b[32m{}\x1b[0m", message);
}

pub fn eprint_yellow(message: &str) {
    eprintln!("\x1b[33m{}\x1b[0m", message);
}

pub fn eprint_blue(message: &str) {
    eprintln!("\x1b[34m{}\x1b[0m", message);
}
