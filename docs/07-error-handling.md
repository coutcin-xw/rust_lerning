# 第 7 章：错误处理

## 学习目标

- 理解 Rust 的两种错误处理方式：`Result` 和 `panic`
- 掌握 `?` 操作符传播错误
- 创建自定义错误类型
- 理解何时用 `panic`，何时用 `Result`

## Rust 的错误哲学

Rust 将错误分为两类：

| 类型 | 用途 | 示例 |
|------|------|------|
| **可恢复错误** | `Result<T, E>` | 文件未找到、网络超时 |
| **不可恢复错误** | `panic!` | 数组越界、除零 |

没有异常（try-catch）！错误通过**返回值**处理。

## Result\<T, E\>

```rust
enum Result<T, E> {
    Ok(T),     // 成功，包含返回值
    Err(E),    // 失败，包含错误信息
}
```

基本用法：

```rust
use std::fs::File;
use std::io::ErrorKind;

fn open_file() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => File::create("hello.txt").unwrap(),
            other_error => panic!("无法打开文件: {:?}", other_error),
        },
    };
}
```

## ? 操作符

`?` 是 Rust 错误处理最常用的语法糖：

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;  // 如果 Err，直接返回
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

`?` 做了两件事：
1. 如果是 `Err`，直接 `return Err(...)`
2. 如果是 `Ok`，取出内部值继续

### 链式调用

```rust
fn read_username_short() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

### ? 的自动类型转换

`?` 通过 `From` trait 自动转换错误类型：

```rust
// 自定义错误类型可以自动从 io::Error 转换
fn do_stuff() -> Result<(), MyError> {
    let f = File::open("data.txt")?;  // io::Error → MyError
    Ok(())
}
```

> 💡 `?` 只能在返回 `Result`（或 `Option`）的函数中使用。

## unwrap 和 expect

快速获取值，失败时 panic：

```rust
let f = File::open("hello.txt").unwrap();
// 失败时：thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value'

let f = File::open("hello.txt").expect("无法打开 hello.txt");
// 失败时：thread 'main' panicked at '无法打开 hello.txt: ...'
```

使用原则：

| 方法 | 使用场景 |
|------|---------|
| `unwrap()` | 原型开发、测试、确定不会失败时 |
| `expect("原因")` | 同上，但附带说明信息 |
| `?` | 生产代码，把错误交给调用者处理 |

## 自定义错误类型

```rust
#[derive(Debug)]
enum MyError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    Custom(String),
}

// 实现 From trait，让 ? 自动转换
impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::IoError(err)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(err: std::num::ParseIntError) -> Self {
        MyError::ParseError(err)
    }
}
```

## Option 和 Result 的便捷方法

```rust
let x = Some(5);

// 提供默认值
x.unwrap_or(0);          // None → 0
x.unwrap_or_else(|| 42); // 惰性求值

// 转换
x.map(|n| n * 2);        // Some(10)
x.and_then(|n| Some(n * 2));  // 链式处理

// Option ↔ Result
let x: Option<i32> = Some(5);
let r: Result<i32, &str> = x.ok_or("没有值");  // Option → Result
let o: Option<i32> = r.ok();                    // Result → Option
```

## panic! vs Result

| 场景 | 选择 |
|------|------|
| 调用者可以合理处理 | `Result` |
| 不变式被破坏（bug） | `panic!` |
| 外部不可控因素（网络、文件） | `Result` |
| 测试和原型 | `unwrap` / `expect` |
| 关键安全检查 | `debug_assert!`（仅 debug 模式） |

```rust
// debug_assert! 仅在 debug 模式下检查，release 模式忽略
debug_assert!(x > 0, "x 必须为正数");
```

## 常见陷阱

> ⚠️ **在库代码中使用 `unwrap`。** 库应该返回 `Result`，把错误处理的决定权交给使用者。只在应用代码的顶层使用 `unwrap`。

> ⚠️ **吞掉错误。** `let _ = fallible_fn()` 会忽略错误。如果确实需要忽略，加注释说明原因。

## 练习

1. 写一个读取文件并解析为数字的函数，使用 `?` 传播错误
2. 定义一个自定义错误类型，实现 `From` trait 支持自动转换
3. 写一个函数从环境变量读取配置，变量不存在时返回友好错误

---

← [第 6 章：模式匹配](./06-pattern-match.md) | [返回目录](./README.md) | → [第 8 章：Trait 与泛型](./08-trait-generics.md)
