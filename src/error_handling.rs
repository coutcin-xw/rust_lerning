// ============================================================
// 06_error_handling.rs - 错误处理
// ============================================================
//
// 【核心概念】
// Rust使用类型系统处理错误，而不是异常
// 错误是值，必须显式处理
//
// 【两种错误处理方式】
// 1. Result<T, E> - 可恢复的错误
// 2. panic! - 不可恢复的错误
//
// 【对比其他语言】
// - C: 错误码，容易忽略，不安全
// - C++: 异常，性能开销，难以推理
// - Java: 受检异常，繁琐的try-catch
// - Go: 多返回值error，类似Rust但无?操作符
// - Rust: Result类型 + ?操作符，强制处理，零开销
//
// 【设计哲学】
// - 显式优于隐式
// - 可恢复错误应该传递，不可恢复错误应该终止
// - 错误处理不应该被忽略
// ============================================================

use std::fs::File;
use std::io::{self, Read};

// --- Result枚举 ---
// Result<T, E> 定义在标准库中
// enum Result<T, E> {
//     Ok(T),   // 成功，包含值T
//     Err(E),  // 失败，包含错误E
// }

// --- 基本用法 ---
fn read_file_contents(path: &str) -> Result<String, io::Error> {
    // 【?操作符】
    // Rust的错误处理魔法
    // 如果Result是Err，立即返回
    // 如果是Ok，提取值继续

    let mut f = File::open(path)?; // 打开文件，失败则返回Err
    let mut contents = String::new();
    f.read_to_string(&mut contents)?; // 读取内容，失败则返回Err
    Ok(contents) // 成功返回内容
}

fn result_basics() {
    println!("=== Result基础 ===");

    // 【手动处理Result】
    // 使用match处理所有情况
    let f = File::open("Cargo.toml");
    match f {
        Ok(_file) => println!("文件打开成功"),
        Err(e) => println!("打开失败: {}", e),
    }

    // 【链式调用】
    // Result支持函数式风格
    let result = File::open("Cargo.toml").and_then(|mut f| {
        // and_then: 成功时执行，失败则跳过
        let mut contents = String::new();
        f.read_to_string(&mut contents).map(|_| contents)
        // map: 成功时转换值
    });

    match result {
        Ok(c) => println!("内容长度: {}", c.len()),
        Err(e) => println!("错误: {}", e),
    }

    // 【Result的优势】
    // 1. 必须处理错误，不能忽略
    // 2. 错误信息携带在类型中
    // 3. 错误处理路径清晰可见
}

// --- ?操作符详解 ---
// ? 是Rust最优雅的错误处理方式

fn read_config(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    // ? 可以链式使用
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

// 【?的工作原理】
// 对于 Result<T, E>:
// - Ok(v) => 返回 v
// - Err(e) => 立即从函数返回 Err(e.into())
//
// 对于 Option<T>:
// - Some(v) => 返回 v
// - None => 立即从函数返回 None

// 【?的错误转换】
// ? 会自动转换错误类型
// 需要 From trait 实现
// Err(e)? 中的 e 会被转换成函数返回的E类型

// --- unwrap和expect ---
fn unwrap_usage() {
    println!("=== unwrap和expect ===");

    // 【unwrap】
    // Ok => 返回值
    // Err => panic!
    // 用于原型开发或确定不会失败的情况
    let f = File::open("Cargo.toml").unwrap();
    println!("unwrap: 成功打开文件");

    // 【expect】
    // 类似unwrap，但可以自定义错误信息
    // let f = File::open("notexist.txt")
    //     .expect("应该能打开这个文件");

    // 【何时使用unwrap/expect】
    // 1. 测试代码
    // 2. 原型开发
    // 3. 确定不会失败的情况
    // 4. 程序无法继续的错误（此时panic是合理的）

    // 【生产代码建议】
    // 使用 ? 操作符，让调用者决定如何处理
}

// --- ?和match对比 ---
fn error_propagation() {
    println!("=== 错误传播 ===");

    // 【使用match】
    fn with_match() -> Result<i32, &'static str> {
        // 显式处理错误并返回
        let n = match read_number_from_file() {
            Ok(n) => n,
            Err(e) => return Err(e), // 手动返回错误
        };
        Ok(n)
    }

    // 【使用?（更简洁）】
    fn with_question_mark() -> Result<i32, &'static str> {
        let n = read_number_from_file()?; // ? 自动处理错误
        Ok(n)
    }

    println!("错误传播演示完成");
    // ? 让代码更简洁，同时保持类型安全
}

fn read_number_from_file() -> Result<i32, &'static str> {
    Ok(42)
}

// --- 自定义错误类型 ---
// 当需要统一处理多种错误时

#[derive(Debug)]
enum MyError {
    IoError(io::Error),                  // IO错误
    ParseError(std::num::ParseIntError), // 解析错误
    Custom(String),                      // 自定义错误
}

// 【实现From trait】
// 让?可以自动转换错误类型
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

// 【使用自定义错误类型】
fn custom_error_demo() -> Result<i32, MyError> {
    // ? 会自动调用From::from转换错误
    let s = std::fs::read_to_string("Cargo.toml")?; // io::Error -> MyError
    let n: i32 = s.parse()?; // ParseIntError -> MyError
    Ok(n)
}

// 【自定义错误的场景】
// 1. 库API需要统一的错误类型
// 2. 需要添加额外上下文信息
// 3. 需要实现特定的错误处理逻辑

// --- Option和Result转换 ---
fn option_result_convert() {
    println!("=== Option和Result转换 ===");

    // 【Result -> Option】
    // 丢弃错误信息
    let r: Result<i32, &str> = Ok(5);
    let o: Option<i32> = r.ok(); // Ok(v) => Some(v), Err => None
    println!("Result转Option: {:?}", o);

    let r: Result<i32, &str> = Err("error");
    let o: Option<i32> = r.ok();
    println!("Err转Option: {:?}", o); // None

    // 【Option -> Result】
    // 需要提供错误值
    let o: Option<i32> = Some(5);
    let r: Result<i32, &str> = o.ok_or("None错误");
    println!("Option转Result: {:?}", r);

    let o: Option<i32> = None;
    let r: Result<i32, &str> = o.ok_or("自定义错误");
    println!("None转Result: {:?}", r);

    // 【使用场景】
    // Option -> Result: 当需要区分"无值"和"错误"时
    // Result -> Option: 当只关心有没有值，不关心错误时
}

// --- unwrap_or和unwrap_or_else ---
fn unwrap_methods() {
    println!("=== Result的便捷方法 ===");

    // 【unwrap_or】
    // 提供默认值，立即计算
    let x: Result<i32, &str> = Err("error");
    let n = x.unwrap_or(0);
    println!("unwrap_or: {}", n);

    // 【unwrap_or_else】
    // 提供闭包，延迟计算
    let x: Result<i32, &str> = Err("error");
    let n = x.unwrap_or_else(|e| {
        println!("遇到错误: {}", e);
        42 // 只有在Err时才计算
    });
    println!("unwrap_or_else: {}", n);

    // 【unwrap_or_default】
    // 使用类型的默认值
    let x: Result<i32, &str> = Err("error");
    let n = x.unwrap_or_default(); // i32默认值是0
    println!("unwrap_or_default: {}", n);

    // 【and_then】
    // 链式操作，类似flatMap
    let n: Result<i32, &str> = Ok(5)
        .and_then(|x| Ok(x * 2)) // 成功时转换
        .and_then(|x| Ok(x + 1));
    println!("and_then链式: {:?}", n);

    // 【map】
    // 转换Ok值
    let n: Result<i32, &str> = Ok(5).map(|x| x * 2);
    println!("map: {:?}", n);

    // 【map_err】
    // 转换Err值
    let n: Result<i32, String> = Err("error".to_string()).map_err(|e| format!("Error: {}", e));
    println!("map_err: {:?}", n);

    // 【or】
    // 提供备选Result
    let x: Result<i32, &str> = Err("error");
    let n: Result<i32, &str> = x.or(Ok(10));
    println!("or: {:?}", n);

    // 【方法选择指南】
    // - 需要默认值：unwrap_or / unwrap_or_default
    // - 需要延迟计算：unwrap_or_else
    // - 需要转换值：map
    // - 需要链式操作：and_then
    // - 需要转换错误：map_err
}

// --- panic vs Result ---
fn panic_vs_result() {
    println!("=== panic vs Result ===");

    // 【panic! - 不可恢复的错误】
    // 用于程序bug或无法恢复的情况
    // 例如：数组越界、除零、null解引用

    // panic的例子（已注释）：
    // let v = vec![1, 2, 3];
    // v.get(100).unwrap();  // panic: index out of bounds

    // panic会：
    // 1. 打印错误信息
    // 2. 展开调用栈（或直接中止）
    // 3. 终止程序

    // 【Result - 可恢复的错误】
    // 用于预期的、可处理的错误
    // 例如：文件不存在、网络超时、解析失败

    // 【选择原则】
    // 1. 是程序bug吗？-> panic
    // 2. 调用者能处理吗？-> Result
    // 3. 是预期的情况吗？-> Result
    // 4. 程序能继续运行吗？-> Result

    // 【例子】
    // - 文件不存在 -> Result（用户可能提供其他路径）
    // - 配置格式错误 -> Result（可以显示错误信息）
    // - 内存不足 -> panic（无法继续）
    // - 数组越界 -> panic（程序bug）

    // 【调试断言】
    // debug_assert! 只在debug模式生效
    // 用于性能敏感的代码中的断言
    let x = 5;
    debug_assert!(x > 0, "x必须为正数");

    println!("错误处理原则：可恢复用Result，不可恢复用panic");
}

// --- 错误处理最佳实践 ---
// 【总结】
// 1. 库代码返回Result，让调用者决定
// 2. 应用代码在main中处理Result
// 3. 使用?传播错误，而不是unwrap
// 4. 为错误添加上下文信息
// 5. 定义清晰的错误类型

pub fn run() {
    println!("\n========== 06_error_handling ==========");
    result_basics();
    error_propagation();
    option_result_convert();
    unwrap_methods();
    panic_vs_result();
}
