pub fn get_time() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        #[cfg(feature = "local")]
        {
            (ms - STIME) * 1.0
        }
        #[cfg(not(feature = "local"))]
        {
            ms - STIME
        }
    }
}

#[macro_export]
macro_rules! print_red {
    ($x:expr) => {
        print!("\x1b[31m{}\x1b[37m", $x);
    };
}

#[macro_export]
macro_rules! println_red {
    ($x:expr) => {
        println!("\x1b[31m{}\x1b[37m", $x);
    };
}
