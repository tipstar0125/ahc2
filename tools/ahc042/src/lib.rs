#![allow(non_snake_case, unused_macros)]

use proconio::{input, marker::Chars};
use rand::prelude::*;
use std::ops::RangeBounds;
use svg::node::element::{Definitions, Group, Image, Rectangle, Style, Text, Title, Use};

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

#[derive(Clone, Debug)]
pub struct Input {
    cs: Vec<Vec<char>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.cs.len())?;
        for cs in &self.cs {
            writeln!(f, "{}", cs.iter().collect::<String>())?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        n: usize,
        cs: [Chars; n],
    }
    let cs = cs.iter().map(|cs| cs.to_vec()).collect();
    Input { cs }
}

pub fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr, R: RangeBounds<T>>(
    token: Option<&str>,
    range: R,
) -> Result<T, String> {
    if let Some(v) = token {
        if let Ok(v) = v.parse::<T>() {
            if !range.contains(&v) {
                Err(format!("Out of range: {}", v))
            } else {
                Ok(v)
            }
        } else {
            Err(format!("Parse error: {}", v))
        }
    } else {
        Err("Unexpected EOF".to_owned())
    }
}

pub struct Output {
    pub out: Vec<(char, usize)>,
}

pub fn parse_output(input: &Input, f: &str) -> Result<Output, String> {
    let mut out = vec![];
    let mut f = f.split_whitespace().peekable();
    while f.peek().is_some() {
        let dir = read(f.next(), 'A'..='Z')?;
        let p = read(f.next(), 0..input.cs.len())?;
        out.push((dir, p));
        if out.len() > 4 * input.cs.len() * input.cs.len() {
            return Err("Too many operations".to_owned());
        }
    }
    Ok(Output { out })
}

pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    loop {
        let n = 20;
        let mut cs = mat!['.'; n; n];
        for _ in 0..n * 2 {
            let (i, j) = loop {
                let i = rng.gen_range(0..n as i32) as usize;
                let j = rng.gen_range(0..n as i32) as usize;
                if cs[i][j] == '.' {
                    break (i, j);
                }
            };
            cs[i][j] = 'o';
        }
        let mut cand = vec![];
        for i in 0..n {
            for j in 0..n {
                if cs[i][j] == '.' {
                    if (0..i).all(|i| cs[i][j] != 'o')
                        || (i + 1..n).all(|i| cs[i][j] != 'o')
                        || (0..j).all(|j| cs[i][j] != 'o')
                        || (j + 1..n).all(|j| cs[i][j] != 'o')
                    {
                        cand.push((i, j));
                    }
                }
            }
        }
        if cand.len() < n * 2 {
            continue;
        }
        cand.shuffle(&mut rng);
        for &(i, j) in cand.iter().take(n * 2) {
            cs[i][j] = 'x';
        }
        return Input { cs };
    }
}

pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    let (mut score, err, _) = compute_score_details(input, &out.out);
    if err.len() > 0 {
        score = 0;
    }
    (score, err)
}

pub fn compute_score_details(
    input: &Input,
    out: &[(char, usize)],
) -> (i64, String, Vec<Vec<char>>) {
    let n = input.cs.len();
    let mut cs = input.cs.clone();
    for &(d, p) in out {
        match d {
            'L' => {
                let i = p;
                for j in 0..n - 1 {
                    cs[i][j] = cs[i][j + 1];
                }
                cs[i][n - 1] = '.';
            }
            'R' => {
                let i = p;
                for j in (1..n).rev() {
                    cs[i][j] = cs[i][j - 1];
                }
                cs[i][0] = '.';
            }
            'U' => {
                let j = p;
                for i in 0..n - 1 {
                    cs[i][j] = cs[i + 1][j];
                }
                cs[n - 1][j] = '.';
            }
            'D' => {
                let j = p;
                for i in (1..n).rev() {
                    cs[i][j] = cs[i - 1][j];
                }
                cs[0][j] = '.';
            }
            _ => {
                return (0, format!("Invalid direction: {}", d), cs);
            }
        }
    }
    let T = out.len();
    let mut X = 0;
    let mut Y = 2 * n;
    for i in 0..n {
        for j in 0..n {
            if cs[i][j] == 'x' {
                X += 1;
            }
            if cs[i][j] == 'o' {
                Y -= 1;
            }
        }
    }
    let score = if X == 0 && Y == 0 {
        8 * n * n - T
    } else {
        4 * n * n - n * (X + Y)
    };
    (score as i64, String::new(), cs)
}

/// 0 <= val <= 1
pub fn color(mut val: f64) -> String {
    val.setmin(1.0);
    val.setmax(0.0);
    let (r, g, b) = if val < 0.5 {
        let x = val * 2.0;
        (
            30. * (1.0 - x) + 144. * x,
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
        )
    } else {
        let x = val * 2.0 - 1.0;
        (
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
            30. * (1.0 - x) + 70. * x,
        )
    };
    format!(
        "#{:02x}{:02x}{:02x}",
        r.round() as i32,
        g.round() as i32,
        b.round() as i32
    )
}

pub fn rect(x: usize, y: usize, w: usize, h: usize, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}

pub fn group(title: String) -> Group {
    Group::new().add(Title::new(title))
}

pub fn vis_default(input: &Input, out: &Output) -> (i64, String, String) {
    let (mut score, err, svg) = vis(input, &out.out, false);
    if err.len() > 0 {
        score = 0;
    }
    (score, err, svg)
}

const IMAGES: [&'static str; 2] = [
    // https://dotown.maeda-design-room.net/1093/
    "data:image/png;charset=utf-8;base64,iVBORw0KGgoAAAANSUhEUgAAApQAAALQCAYAAAAuBuBGAAAOd0lEQVR4nO3YsY5dRx3A4XvRbZZmG1YKTdK4cBMqIhpbtFEegBZcIEFHm2rvreiCkEBCAmFFsojEM6TzdhG9ARe2K0tbbROXh4InSH7JnbNnvu8FZuY/e8/+NDsAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD4hvazDWxZlhXsAvg2jqfTbD/gId/o4/X1iGVhU/b7uRLrByvYAwAA95igBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkB+NjS46n05DTHK+vx6x7Oi1DFh5nP9Nhj787jrnfu+OQZXeXy1T3++Tx1ZB1D68+HrIu2+aFEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASA7Gx5Ycr6/dJ3AvPH1+O9VFPXl8NWTdw6uPh6w7Gy+UAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAcjA+tuR4Oi0zXejrL/+0gl2c1VT3++TLH61gF+d0NdX97na7/YhFnz6/HbEsG+eFEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASA7GBwB8354+v51qxn/bP1vBLs7HCyUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJDsZxvfsiwr2MX5HE+nqQ787C9/XsEugG/j8YO5vs+zefr8dqrm2O/nSiwvlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQHIwPuCburm8GDKzR3fv3NUZjLrfT3dfD1kX6LxQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAyWG28R1Pp2UF2zib33zx90lO+n/P1rAJAJiMF0oAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACA5GB9bcnN5sR9xnEd374ZM8ebyYhmy8CA3lxczHRfg3vBCCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIcJx7dfwR7OaZnnqOO8fPPfMXP+/Kt1DGDrfvnRkAO+/fAnM0159/vbH65gF2c15P/Rey9eDznsk8dXQ9blPLxQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAycH44DuxHzTGxfWdwedfbf6IAIUXSgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAINnPNr5lWVawi/N5+/CDuQ4MACvw43+/maqxvFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAcjC+zdvPPoCNW2YfANxjs32ffa82zAslAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQHIwP7q9Hd++mur2XL25XsAv4brz92fsmyWZ4oQQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAAJKD8W3bey9eT3Xez3710xXs4nxe/vFfsxyVCTx4eDXVNd9cjln3zT9/PWTd93/x1/2QhTkLL5QAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEByMD7gvnjw8MpdncHLF7dTrTvKqL/nf6zh8GyOF0oAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACDZzza+ZVlWsAu+L28ffuCCt23IN+vR3buJRjzOzeWF3++2Dfn9/ucPnw4Z6s8/+e2QdUfxQgkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAACJoAQAIBGUAAAkghIAgERQAgCQCEoAABJBCQBAIigBAEgEJQAAiaAEACARlAAAJIISAIBEUAIAkAhKAAASQQkAQCIoAQBIBCUAAImgBAAgEZQAACSCEgCARFACAJAISgAAEkEJAEAiKAEASAQlAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABwPrvd7n/ev3+4ZeU4igAAAABJRU5ErkJggg==",
    // https://dotown.maeda-design-room.net/3513/
    "data:image/png;charset=utf-8;base64,iVBORw0KGgoAAAANSUhEUgAAA8AAAALQBAMAAACtWQSHAAAAJ1BMVEUAAAD///8WiokjGBVuTV9xTVWRYDSaZ6DCchPknSPo0mLqVRT/9uNxzgU8AAAAAnRSTlMAAHaTzTgAAASWSURBVHja7d1RUcNAEIDhwpyBaAkSQAJIAAmthGIBCSAhtRALZyEWcLB92Lm5a/J9rztJ0/y9vu7pBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAOzZUzyehnzoa7M7XxLX/iau/Uhcu4XTZ7/xfRNYYARGYARGYARGYIERGIERGIERGIEFRmAERmAERmAERmCBERiBERiBERiBBUZgBEZgBEZgBBYYgREYgREYgREYgQVGYARGYARGYAQWGIERGIERGIG5b9DdhdchX1a82XBJ3LmG069wanehv2gERmAERmAERmAEFhiBERiBERiBEVhgBEZgBEZgBEZggb0CgREYgREYgREYgQVGYARGYARGYAQWGIERGIERGIERGIEFRmAERmAERmAEFhiBERiBERiBCZV4fE7c+vtgrzLeMFidYARGYAQWGIERGIERGIERWGAERmAERmAERmAEFhiBERiBERiBEVhgBEZgBEZgBEZggREYgREYgREYgRFYYARGYARGYARGYIERGIERGIERGIERWGAERmD6uLO7MN4/mNls+J64dg2nc2Lay5K49sUJ9heNwAiMwAiMwAiMwAIjMAIjMAIjMAILjMAIjMAIjMAILDACIzACIzACIzACC4zACIzACIzACCwwAiMwAiMwAiMwAguMwAiMwAiMwAgsMAIjMAIjMAITKb0++C+ctts/mNl7uDjBCIzACIzACCwwAiMwAiMwAiOwwAiMwAiMwAiMwAIjMAIjMAIjMAIjsMDsQRnzseZmd16dYARGYARGYARGYIERGIERGIERGIERWGAERmAERmAERmCBERiBERiBERiBj6484kNv4XQa8pnfnGAERmAEFhiBERiBERiBEVhgBEZgBEZgBEZgBBYYgREYgREYgRFYYARGYARGYARGYIERGIERGIERGIERWGAERmAERmAERmCBERiBERiBERiBEVhgBEZg+kjtLoy3BH42e+jMdsKamMZ+nGAERmAERmCBERiBERiBERiBBUZgBEZgBEZgBEZggREYgREYgREYgQVGYARGYARGYAQWGIERGIERGIERGIEFRmAERmAERmAEFhiBERiBERiBEVhgBEZgRpTaXbiF0zF3+d0S3+gcTqfEtDrBCIzAAiMwAiMwAiMwAguMwAiMwAiMwAiMwAIjMAIjMAIjMAILjMAIjMAIjMAIjMACIzACIzACIzACC4zACIzACIzACCwwAiMwAiMwAiMwAguMwAiMwLRSjvaFX8NpvG/xkvjczQlGYARGYIERGIERGIERGIEFRmAERmAERmAERmCBERiBERiBERiBBUZgBEZgBEZgBBYYgREYgREYgREYgQVGYARGYARGYAQWGIERGIERGIERGIEFRmAEpo9uuwvjHYI1nN4SnzsnpqsTjMAIjMAIjMACIzACIzACIzACC4zACIzACIzACCwwAiMwAiMwAiMwAguMwAiMwAiMwAgsMAIjMAIjMAIjsMBegcAIjMAIjMAIjMACIzACIzACIzACC4zAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAIzlH8HCI23db016AAAAAElFTkSuQmCC",
];

pub fn vis(input: &Input, out: &[(char, usize)], manual: bool) -> (i64, String, String) {
    let n = input.cs.len();
    let D = 600 / n;
    let W = D * n + if manual { 100 } else { 0 };
    let H = D * n + if manual { 100 } else { 0 };
    let (score, err, cs) = compute_score_details(input, &out);
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, W + 10, H + 10))
        .set("width", W + 10)
        .set("height", H + 10)
        .set("style", "background-color:white");
    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central;}}"
    )));
    doc = doc.add(
        Definitions::new()
            .add(
                Image::new()
                    .set("id", "oni")
                    .set("width", D)
                    .set("height", D)
                    .set("image-rendering", "pixelated")
                    .set("href", IMAGES[0]),
            )
            .add(
                Image::new()
                    .set("id", "fuku")
                    .set("width", D)
                    .set("height", D)
                    .set("image-rendering", "pixelated")
                    .set("href", IMAGES[1]),
            ),
    );
    let last_i = if out.len() > 0 && matches!(out[out.len() - 1].0, 'L' | 'R') {
        out[out.len() - 1].1
    } else {
        !0
    };
    let last_j = if out.len() > 0 && matches!(out[out.len() - 1].0, 'U' | 'D') {
        out[out.len() - 1].1
    } else {
        !0
    };
    for i in 0..n {
        for j in 0..n {
            let mut g = group(format!(
                "({}, {}){}",
                i,
                j,
                if cs[i][j] == 'o' {
                    "\nFukunokami"
                } else if cs[i][j] == 'x' {
                    "\nOni"
                } else {
                    ""
                }
            ))
            .add(
                rect(
                    j * D,
                    i * D,
                    D,
                    D,
                    if last_i == i || last_j == j {
                        "#f0d0d0"
                    } else {
                        "#f0f0f0"
                    },
                )
                .set("stroke", "black")
                .set("stroke-width", 1),
            );
            if cs[i][j] == 'o' {
                g = g.add(
                    Use::new()
                        .set("href", "#fuku")
                        .set("x", j * D)
                        .set("y", i * D),
                );
            } else if cs[i][j] == 'x' {
                g = g.add(
                    Use::new()
                        .set("href", "#oni")
                        .set("x", j * D)
                        .set("y", i * D),
                );
            }
            doc = doc.add(g);
        }
    }
    if manual {
        for i in 0..n {
            doc = doc.add(
                Text::new("◀")
                    .set("x", n * D + D / 2)
                    .set("y", i * D + D / 2)
                    .set("font-size", D / 2)
                    .set("fill", "gray")
                    .set("onclick", format!("manual_update('L', {})", i)),
            );
            doc = doc.add(
                Text::new("▶")
                    .set("x", n * D + D / 2 + D * 2 / 3)
                    .set("y", i * D + D / 2)
                    .set("font-size", D / 2)
                    .set("fill", "gray")
                    .set("onclick", format!("manual_update('R', {})", i)),
            );
            doc = doc.add(
                Text::new("▲")
                    .set("x", i * D + D / 2)
                    .set("y", n * D + D / 2)
                    .set("font-size", D / 2)
                    .set("fill", "gray")
                    .set("onclick", format!("manual_update('U', {})", i)),
            );
            doc = doc.add(
                Text::new("▼")
                    .set("x", i * D + D / 2)
                    .set("y", n * D + D / 2 + D * 2 / 3)
                    .set("font-size", D / 2)
                    .set("fill", "gray")
                    .set("onclick", format!("manual_update('D', {})", i)),
            );
        }
    }
    (score, err, doc.to_string())
}
