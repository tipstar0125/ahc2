# 関数型デザインパターン in Rust

Rustで関数型スタイルを活かした代表的なデザインパターンをまとめました。  
副作用の制御、関数合成、データ不変性などを活かして、柔軟かつ堅牢な設計が可能です。

---

## 1. Strategy パターン

### 🧠 概要
アルゴリズムを関数として渡し、動的に振る舞いを切り替える。

### 🦀 Rust実装

```rust
fn sort_strategy<F>(mut data: Vec<i32>, strategy: F) -> Vec<i32>
where
    F: Fn(&i32, &i32) -> std::cmp::Ordering,
{
    data.sort_by(strategy);
    data
}

fn main() {
    let numbers = vec![5, 2, 8, 3];

    let ascending = sort_strategy(numbers.clone(), |a, b| a.cmp(b));
    let descending = sort_strategy(numbers, |a, b| b.cmp(a));

    println!("Asc: {:?}", ascending);
    println!("Desc: {:?}", descending);
}
```

---

## 2. Execute Around Method パターン

### 🧠 概要
前後処理（初期化・クリーンアップ）を囲い込み、本質処理だけ渡す。

### 🦀 Rust実装（ファイル読み込み）

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn with_file<F, R>(path: &str, func: F) -> io::Result<R>
where
    F: FnOnce(BufReader<File>) -> R,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(func(reader))
}

fn main() -> io::Result<()> {
    let line_count = with_file("sample.txt", |reader| {
        reader.lines().count()
    })?;

    println!("Lines: {}", line_count);
    Ok(())
}
```

---

## 3. Template Method パターン

### 🧠 概要
アルゴリズムの構造は固定し、可変部分を関数で外から注入する。

### 🦀 Rust実装

```rust
fn process_template<F1, F2>(prepare: F1, execute: F2)
where
    F1: Fn(),
    F2: Fn(),
{
    prepare();
    println!("Main logic...");
    execute();
}

fn main() {
    process_template(
        || println!("Prepare DB..."),
        || println!("Execute query..."),
    );
}
```

---

## 4. Chain of Responsibility パターン

### 🧠 概要
処理を順番にチェーンして適用する。

### 🦀 Rust実装

```rust
fn apply_chain(mut value: i32, steps: Vec<Box<dyn Fn(i32) -> i32>>) -> i32 {
    for step in steps {
        value = step(value);
    }
    value
}

fn main() {
    let steps: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 2),
        Box::new(|x| x - 3),
    ];

    let result = apply_chain(5, steps);
    println!("Result: {}", result);
}
```

---

## 5. Builder パターン（イミュータブル）

### 🧠 概要
設定をステップごとに積み上げて、最終的に構築。

### 🦀 Rust実装

```rust
#[derive(Debug, Clone)]
struct Config {
    host: String,
    port: u16,
}

impl Config {
    fn new() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
        }
    }

    fn with_host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
}

fn main() {
    let cfg = Config::new()
        .with_host("example.com")
        .with_port(3000);

    println!("{:?}", cfg);
}
```

---

## 6. Visitor パターン（パターンマッチで代用）

### 🧠 概要
構造体に応じた処理を切り替える。関数型では enum + match を使う。

### 🦀 Rust実装

```rust
enum Expr {
    Value(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Value(x) => *x,
        Expr::Add(lhs, rhs) => eval(lhs) + eval(rhs),
        Expr::Mul(lhs, rhs) => eval(lhs) * eval(rhs),
    }
}

fn main() {
    let expr = Expr::Add(
        Box::new(Expr::Value(2)),
        Box::new(Expr::Mul(Box::new(Expr::Value(3)), Box::new(Expr::Value(4)))),
    );

    println!("Result: {}", eval(&expr)); // 2 + (3 * 4) = 14
}
```

---

## 🧠 まとめ表

| パターン                  | 関数型でのRust表現                         |
|---------------------------|--------------------------------------------|
| Strategy                  | クロージャで動的な戦略を渡す               |
| Execute Around Method     | リソース操作をクロージャで抽象化           |
| Template Method           | 可変部分を関数で差し替え可能に             |
| Chain of Responsibility   | 関数のリストを順番に適用                   |
| Builder（イミュータブル） | メソッドチェーンで値を構築                 |
| Visitor                   | enum + match で型に応じた処理を記述         |