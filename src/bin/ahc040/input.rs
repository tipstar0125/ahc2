use proconio::input_interactive;

pub fn read_input() -> Input {
    input_interactive! {
        N: usize, T: usize, sigma: i32,
        wh2: [(i32, i32); N],
    }

    Input { N, T, sigma, wh2 }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub T: usize,
    pub sigma: i32,
    pub wh2: Vec<(i32, i32)>,
}
