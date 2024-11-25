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

/// あるマスを削除した際に残りが連結か
/// 012
/// 3.5
/// 678
pub fn connect9() -> Vec<bool> {
    let mut ok = vec![false; 1 << 9];
    for mask in 0..1 << 9 {
        let mut k = 0;
        for i in 0..4 {
            if mask >> (i * 2 + 1) & 1 != 0 {
                k += 1;
            }
        }
        for (a, b, c) in [(0, 1, 3), (1, 2, 5), (3, 6, 7), (5, 7, 8)] {
            if mask >> a & 1 != 0 && mask >> b & 1 != 0 && mask >> c & 1 != 0 {
                k -= 1;
            }
        }
        ok[mask] = k == 1;
    }
    ok
}

pub fn get_mask9(bs: &Vec<Vec<bool>>, i: usize, j: usize) -> usize {
    let mut k = 0;
    let mut mask = 0;
    for di in [!0, 0, 1] {
        let i2 = i + di;
        for dj in [!0, 0, 1] {
            let j2 = j + dj;
            if i2 < bs.len() && j2 < bs[i2].len() && bs[i2][j2] {
                mask |= 1 << k;
            }
            k += 1;
        }
    }
    mask
}