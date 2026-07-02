# 第 18 章：测试

## 学习目标

- 编写单元测试和集成测试
- 使用 `cargo test` 运行测试
- 编写文档测试
- 掌握常用的断言宏

## 单元测试

在源文件中使用 `#[cfg(test)]` 模块编写测试：

```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;  // 导入父模块的所有内容

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, -2), -3);
    }
}
```

运行测试：

```bash
cargo test               # 运行所有测试
cargo test test_add      # 运行名称匹配的测试
cargo test -- --nocapture  # 显示被测试函数的输出
cargo test -- --ignored  # 只运行被忽略的测试
```

## 断言宏

```rust
assert!(x > 0);                  // 条件为真
assert_eq!(result, expected);    // 相等（需要 PartialEq）
assert_ne!(result, unexpected);  // 不等

// 带自定义消息的断言
assert_eq!(a, b, "测试失败！a={}, b={}", a, b);
```

### should_panic

验证代码在特定情况下 panic：

```rust
#[test]
#[should_panic(expected = "除数不能为零")]  // 可选：匹配 panic 消息
fn test_divide_by_zero() {
    divide(1, 0);
}
```

### 忽略测试

```rust
#[test]
#[ignore]  // 默认不运行
fn expensive_test() {
    // 耗时很长的测试
}
```

## 集成测试

放在项目根目录的 `tests/` 目录下（与 `src/` 同级）：

```
my_project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    ├── common/
    │   └── mod.rs    # 共享的测试辅助模块
    ├── api_test.rs   # 每个文件是一个独立的测试 crate
    └── db_test.rs
```

```rust
// tests/api_test.rs
use my_project;  // 使用库（像外部 crate 一样）

mod common;  // 引入辅助模块（tests/common/mod.rs）

#[test]
fn test_user_api() {
    common::setup();  // 使用共享的测试辅助
    let result = my_project::get_user(1);
    assert!(result.is_ok());
}
```

```rust
// tests/common/mod.rs
pub fn setup() {
    // 测试环境初始化
}
```

> 💡 集成测试只能测试 `lib.rs` 中的公开 API。把核心逻辑放在 `lib.rs` 中，`main.rs` 保持简单。

## 文档测试

在文档注释中写可运行的测试：

```rust
/// 计算两个数的和。
///
/// # 示例
///
/// ```
/// let result = my_project::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```bash
cargo test --doc  # 运行所有文档测试
```

## 测试最佳实践

> ⚠️ **测试私有函数。** 虽然不能直接测试私有函数（集成测试只能测公开 API），但单元测试在同一文件中可以访问私有函数。

> ⚠️ **每个测试互不依赖。** `cargo test` 默认并行运行测试。如果一个测试依赖另一个的状态，会导致不可预测的结果。

> ⚠️ **测试函数名要有描述性。** 用 `test_add_negative_numbers` 而不是 `test1`。

## 组织测试的建议

| 位置 | 用途 | 访问权限 |
|------|------|---------|
| `#[cfg(test)] mod tests` | 单元测试 | 私有 API 可访问 |
| `tests/` | 集成测试 | 仅公开 API |
| `/// ``` ... ``` ` | 文档测试 | 公开 API + 作为文档示例 |

## 练习

1. 为一个计算器库编写完整的单元测试（加减乘除 + 边界情况）
2. 在 `tests/` 目录下编写集成测试，测试公开 API
3. 为几个关键函数添加 `#[should_panic]` 测试和带文档测试的示例

---

← [第 17 章：标准库集合](./17-collections.md) | [返回目录](./README.md) | → [第 19 章：unsafe Rust 与 FFI](./19-unsafe-ffi.md)
