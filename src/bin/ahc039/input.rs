use proconio::input;

pub fn read_input() -> Input {
    input! {}

    Input {}
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
    }
    Input {}
}

#[derive(Debug)]
pub struct Input {}
