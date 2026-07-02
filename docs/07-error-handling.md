# 第 7 章：错误处理

## 学习目标

- 区分可恢复错误（`Result`）和不可恢复错误（`panic`）
- 熟练使用 `?` 操作符传播错误
- 实现 `From` trait 构建自定义错误类型
- 掌握 `Option` 和 `Result` 的便捷组合方法
- 建立"何时 panic、何时 Result"的判断力

## Rust 的错误哲学

Rust 没有异常（try-catch）。错误通过**返回值**处理，而且是类型系统的一部分：

```
            错误处理
           ↙        ↘
    可恢复             不可恢复
   Result<T, E>       panic!
   (文件找不到)        (数组越界)
   (网络超时)          (除零)
   (格式错误)          (不可达代码)
```

这种二分法让你在写代码时**被迫思考**每个可能失败的操作应该怎么处理。编译器的检查确保你不会"忘记处理错误"。

## Result\<T, E\>

```rust
enum Result<T, E> {
    Ok(T),    // 成功：包含返回值
    Err(E),   // 失败：包含错误信息
}
```

### 使用 match 处理

```rust
use std::fs::File;
use std::io::ErrorKind;

fn open_or_create() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("创建文件失败: {:?}", e),
            },
            other_error => panic!("打开文件失败: {:?}", other_error),
        },
    };
}
```

但每次都这样写太冗长了。Rust 提供了更简洁的方式。

### unwrap 和 expect

快速获取值，失败时自动 panic：

```rust
let f1 = File::open("hello.txt").unwrap();
// Err 时: thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value'

let f2 = File::open("hello.txt").expect("无法打开 hello.txt");
// Err 时: thread 'main' panicked at '无法打开 hello.txt: ...'
```

`expect` 比 `unwrap` 好——它附带了你写的说明信息，调试时更快定位问题。

使用原则：

| 场景 | 用什么 |
|------|--------|
| 原型开发、测试 | `unwrap()` — 快速，失败了立即知道 |
| 确定不会失败 | `expect("为什么不会失败")` — 留下文档 |
| 需要自定义错误处理 | `match` — 精细控制 |
| 需要传播错误给调用者 | `?` — Rust 最常用的方式 |

## `?` 操作符 —— 传播错误

`?` 是 Rust 错误处理的核心：

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;   // ①
    let mut s = String::new();
    f.read_to_string(&mut s)?;               // ②
    Ok(s)
}
```

`?` 展开后等价于：

```rust
let mut f = match File::open("hello.txt") {
    Ok(file) => file,
    Err(e) => return Err(e.into()),  // 返回错误，并且自动转换成调用者期望的类型
};
```

### `?` 的链式使用

```rust
fn read_username_short() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

这在功能式风格中很常见——每一步都可能失败，`?` 让错误传播几乎不可见。

### `?` 在 `Option` 上

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
// next() 返回 Option<&str>, ? 在 None 时直接返回 None
```

### `?` 的自动类型转换

```rust
// ? 通过 From trait 自动转换错误类型
fn do_things() -> Result<(), MyError> {
    let f = File::open("data.txt")?;  // io::Error 自动转成 MyError
    let n = "42".parse::<i32>()?;    // ParseIntError 自动转成 MyError
    Ok(())
}
```

只要你的自定义错误类型实现了 `From<io::Error>` 和 `From<ParseIntError>`，`?` 就能自动转换。

## 自定义错误类型

```rust
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Config(String),
}

// 让 ? 能自动转换 io::Error
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

// 让 ? 能自动转换 ParseIntError
impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::Parse(err)
    }
}

// 让 AppError 可以打印
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO 错误: {}", e),
            AppError::Parse(e) => write!(f, "解析错误: {}", e),
            AppError::Config(msg) => write!(f, "配置错误: {}", msg),
        }
    }
}

// 使用
fn load_data() -> Result<String, AppError> {
    let content = std::fs::read_to_string("config.toml")?;  // io::Error → AppError
    let value: i32 = content.trim().parse()?;                // ParseIntError → AppError
    Ok(format!("读取值: {}", value))
}
```

在真实项目中，推荐使用 `thiserror` crate 来自动生成这些样板：

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),       // #[from] 自动实现 From

    #[error("解析错误: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("配置错误: {0}")]
    Config(String),
}
```

## Option 和 Result 互转

```rust
// Option → Result
let opt: Option<i32> = Some(5);
let res: Result<i32, &str> = opt.ok_or("没有值");

// Result → Option
let res: Result<i32, &str> = Ok(5);
let opt: Option<i32> = res.ok();        // Err 变 None

// 过滤
let opt = Some(10);
let filtered = opt.filter(|n| n > &5);  // Some(10)
let filtered = opt.filter(|n| n > &100); // None
```

## panic! vs Result —— 选择原则

| 场景 | 选择 | 理由 |
|------|------|------|
| 文件可能不存在 | `Result` | 外部环境不可控，调用者应处理 |
| 网络请求失败 | `Result` | 同上 |
| 用户输入格式错误 | `Result` | 可以给用户友好提示 |
| 数组越界 | `panic!` | 这是代码 bug，应该立即暴露 |
| 不变式被破坏 | `panic!` | "这不可能发生"——如果真的发生了，程序不应继续 |
| 测试代码 | `unwrap` / `expect` | 测试失败应该立即停止 |
| 库代码 | `Result` | 让库的使用者决定怎么处理 |
| `Vec::new()` OOM | `panic!`（系统行为） | 这种极端情况一般无法恢复 |

### debug_assert! —— 仅调试模式检查

```rust
fn sqrt(x: f64) -> f64 {
    debug_assert!(x >= 0.0, "负数没有实数平方根");  // debug 模式检查
    x.sqrt()
}
```

`debug_assert!` 在 release 模式中被移除（不为 0 开销），仅在开发和测试时提供安全网。

## 错误处理的多层策略

```
最内层（库）      → 返回 Result，不用 unwrap
中间层（逻辑）    → 用 ? 传播，或用 match 精细处理
最外层（入口）    → 处理错误：记录日志、返回用户友好信息、重试等
```

```rust
// 内层库
fn read_config() -> Result<Config, ConfigError> { /* ? */ }

// 中间层
fn app_start() -> Result<(), AppError> {
    let config = read_config()?;
    let data = load_data(&config)?;
    process(data)?;
    Ok(())
}

// 最外层
fn main() {
    if let Err(e) = app_start() {
        eprintln!("应用启动失败: {}", e);
        std::process::exit(1);
    }
}
```

## 跨语言对比

| 方式 | Rust | Go | Java | Python |
|------|------|----|------|--------|
| 错误传递 | `Result` + `?` | `if err != nil` | `throw` / `try-catch` | `raise` / `try-except` |
| 空值处理 | `Option<T>` | `nil` | `null` / `Optional<T>` | `None` |
| 程序崩溃 | `panic!`（恢复用 catch_unwind） | `panic` | unchecked exception | 未处理异常 |
| 编译器强制检查 | ✅（match 穷尽性） | ❌ | ❌（checked exception 除外） | ❌ |

## 练习

1. 写一个函数，从文件读取内容，解析为数字，返回 `Result<i32, AppError>`。使用自定义错误类型和 `?` 操作符
2. 实现 `From` trait 让 `io::Error` 和 `ParseIntError` 能自动转换为你的错误类型
3. 写一个 `read_config` 函数：从环境变量读取文件路径 → 打开文件 → 解析为 TOML → 提取某个键的值。使用 `?` 链式调用
4. 区分一个场景：哪里该用 `Result`，哪里该用 `panic!`？为你的决策写注释

---

← [第 6 章：模式匹配](./06-pattern-match.md) | [返回目录](./README.md) | → [第 8 章：Trait 与泛型](./08-trait-generics.md)
