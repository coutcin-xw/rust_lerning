// ============================================================
// 06_error_handling.rs - 错误处理
// Rust使用Result<T, E>和panic处理错误
// 对比: Java异常 -> Rust Result
// 对比: C错误码 -> Rust Result（更类型安全）
// ============================================================

use std::fs::File;
use std::io::{self, Read};

// --- Result枚举 ---
// enum Result<T, E> { Ok(T), Err(E) }

// --- 基本用法 ---
fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?; // ?操作符：返回Err则提前返回
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

fn result_basics() {
    // 手动处理
    let f = File::open("Cargo.toml");
    match f {
        Ok(file) => println!("文件打开成功"),
        Err(e) => println!("打开失败: {}", e),
    }

    // 使用?操作符
    // let content = read_file_contents("Cargo.toml").expect("读取失败");
    // println!("{}", content);

    // 链式调用
    let result = File::open("Cargo.toml").and_then(|mut f| {
        let mut contents = String::new();
        f.read_to_string(&mut contents).map(|_| contents)
    });

    match result {
        Ok(c) => println!("内容长度: {}", c.len()),
        Err(e) => println!("错误: {}", e),
    }
}

// --- ?操作符 ---
// ? 可以用在返回Result的函数中
// - Ok(v) => 返回v
// - Err(e) => 从函数返回Err
fn read_config(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    // 多种错误可以用?链式传播
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

// --- unwrap和expect ---
fn unwrap_usage() {
    // OK: unwrap直接获取值，失败则panic
    let f = File::open("Cargo.toml").unwrap();
    println!("unwrap: 成功");

    // expect类似但可以自定义消息
    // let f = File::open("notexist.txt").expect("文件打开失败");
}

// --- ?和match对比 ---
fn error_propagation() {
    // 使用match
    fn with_match() -> Result<i32, &'static str> {
        let n = match read_number_from_file() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        Ok(n)
    }

    // 使用?（更简洁）
    fn with_question_mark() -> Result<i32, &'static str> {
        let n = read_number_from_file()?;
        Ok(n)
    }

    println!("错误传播演示完成");
}

fn read_number_from_file() -> Result<i32, &'static str> {
    Ok(42)
}

// --- 自定义错误类型 ---
#[derive(Debug)]
enum MyError {
    IoError(io::Error),
    ParseError(std::num::ParseIntError),
    Custom(String),
}

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> MyError {
        MyError::IoError(err)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(err: std::num::ParseIntError) -> MyError {
        MyError::ParseError(err)
    }
}

fn custom_error_demo() -> Result<i32, MyError> {
    // ?会自动转换错误类型（需要实现From trait）
    let s = std::fs::read_to_string("Cargo.toml")?;
    let n: i32 = s.parse()?;
    Ok(n)
}

// --- Option和Result转换 ---
fn option_result_convert() {
    // Result -> Option
    let r: Result<i32, &str> = Ok(5);
    let o: Option<i32> = r.ok();
    println!("Result转Option: {:?}", o);

    // Option -> Result
    let o: Option<i32> = Some(5);
    let r: Result<i32, &str> = o.ok_or("None错误");
    println!("Option转Result: {:?}", r);
}

// --- unwrap_or和unwrap_or_else ---
fn unwrap_methods() {
    let x: Result<i32, &str> = Err("error");

    // unwrap_or: 提供默认值
    let n = x.unwrap_or(0);
    println!("unwrap_or: {}", n);

    // unwrap_or_else: 延迟计算
    let n = x.unwrap_or_else(|_| {
        println!("计算默认值...");
        42
    });
    println!("unwrap_or_else: {}", n);

    // and_then: 链式操作
    let n: Result<i32, &str> = Ok(5).and_then(|x| Ok(x * 2)).and_then(|x| Ok(x + 1));
    println!("and_then链式: {:?}", n);

    // map: 转换Ok值
    let n: Result<i32, &str> = Ok(5).map(|x| x * 2);
    println!("map: {:?}", n);

    // map_err: 转换错误
    let n: Result<i32, String> = Err("error".to_string()).map_err(|e| format!("Error: {}", e));
    println!("map_err: {:?}", n);
}

// --- panic vs Result ---
fn panic_vs_result() {
    // panic!用于不可恢复的错误
    // let v = vec![1, 2, 3];
    // v.get(100).unwrap();  // panic

    // Result用于可恢复的错误
    // 选择用哪个的原则：
    // - 假设调用者能修复错误 -> Result
    // - 假设是程序bug -> panic

    // 调试时使用
    // assert!, debug_assert!

    // 区分：unwrap用于测试/原型，Result用于生产代码
}

pub fn run() {
    println!("\n========== 06_error_handling ==========");
    result_basics();
    error_propagation();
    option_result_convert();
    unwrap_methods();
    panic_vs_result();
}
