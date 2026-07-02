# 第 19 章：测试

## 学习目标

- 编写单元测试（`#[test]` + `#[cfg(test)]`）
- 编写集成测试（`tests/` 目录）
- 编写文档测试（`///` 中的可运行示例）
- 使用 `cargo test` 控制测试的运行和过滤

## 为什么测试在 Rust 中很重要

Rust 编译器帮你消除了大量 bug，但不能验证**业务逻辑**。而且 Rust 的测试工具一流行：测试和源码写在同一文件中，`cargo test` 开箱即用，无需安装第三方框架。

## 单元测试

放在与源码**同一文件**中，用 `#[cfg(test)]` 条件编译：

```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]          // 只在 cargo test 时编译
mod tests {
    use super::*;     // 导入父模块的所有内容

    #[test]
    fn test_add_basic() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, -2), -3);
        assert_eq!(add(-1, 5), 4);
    }

    #[test]
    fn test_divide_ok() {
        assert_eq!(divide(10, 2), Ok(5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(1, 0).is_err());
    }

    #[test]
    #[should_panic(expected = "除数不能为零")]
    fn test_divide_panic() {
        divide(1, 0).unwrap();  // 这里会 panic
    }

    #[test]
    #[ignore]  // 默认不运行，cargo test -- --ignored 才运行
    fn expensive_test() {
        // 耗时很长的测试...
    }
}
```

### 断言宏

| 宏 | 用途 |
|----|------|
| `assert!(condition)` | 断言条件为 true |
| `assert_eq!(a, b)` | 断言 `a == b`（需要 `PartialEq + Debug`） |
| `assert_ne!(a, b)` | 断言 `a != b` |
| `assert!(cond, "msg: {}", extra)` | 带自定义错误消息 |

### 运行测试

```bash
cargo test                          # 运行所有测试
cargo test test_add                 # 运行名称包含 "test_add" 的测试
cargo test tests::                  # 运行 tests 模块的所有测试
cargo test -- --test-threads=1      # 单线程运行（顺序执行）
cargo test -- --nocapture           # 显示 println! 输出
cargo test -- --ignored             # 只运行被忽略的测试
```

## 集成测试

放在项目根目录的 `tests/` 目录下。每个 `.rs` 文件是一个独立的测试 crate：

```
my_project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    ├── common/
    │   └── mod.rs      # 共享辅助模块（不会被当作测试文件）
    ├── api_test.rs      # 会被当作独立的测试 crate
    └── db_test.rs
```

```rust
// tests/api_test.rs
use my_project;  // 像外部用户一样使用你的库

mod common;      // 引入共享辅助模块

#[test]
fn test_get_user() {
    common::setup();
    let user = my_project::get_user(1);
    assert!(user.is_ok());
    assert_eq!(user.unwrap().id, 1);
}
```

```rust
// tests/common/mod.rs
pub fn setup() {
    // 初始化测试环境（如设置环境变量、创建临时目录等）
}
```

> 💡 集成测试只能访问 `lib.rs` 中的**公开 API**——这是有意为之。它迫使你设计好库的公开接口。

## 文档测试

在文档注释中写可运行的示例代码：

```rust
/// 计算两个数的和。
///
/// # 示例
///
/// ```
/// use my_project::add;
///
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// # 注意
/// 溢出时在 debug 模式会 panic。
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```bash
cargo test --doc  # 只运行文档测试
```

文档测试的价值是双重的——它既是测试（确保示例代码正确），也是文档（用户从文档中复制代码就能跑）。

## 测试组织的最佳实践

| 测试类型 | 位置 | 可访问的内容 | 目的 |
|---------|------|------------|------|
| 单元测试 | `#[cfg(test)] mod tests` | 私有 + 公开 | 验证内部逻辑 |
| 集成测试 | `tests/` | 仅公开 API | 验证整体行为 |
| 文档测试 | `/// ``` ... ``` ` | 公开 API | 确保示例正确 |

## 练习

1. 为一个计算器写完整测试：加法、减法、乘法、除法（含除零边界条件）
2. 用 `#[should_panic]` 测试 panic 行为
3. 在 `tests/` 目录下写集成测试，模拟外部用户的方式使用你的库
4. 为一个公开函数添加文档测试，确保 `cargo test --doc` 通过

---

← [第 18 章：标准库集合](./18-collections.md) | [返回目录](./README.md) | → [第 20 章：unsafe Rust 与 FFI](./20-unsafe-ffi.md)
