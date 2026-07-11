# 第 18 章：属性与文档

## 学习目标

- 掌握常用属性：`inline`、`non_exhaustive`、`repr`、`must_use`、`deprecated`
- 理解 lint 控制系统（`allow`、`warn`、`deny`、`forbid`）
- 用 `#[derive]` 自动实现常见 trait
- 编写规范的文档注释（`///`、`//!`），用 `cargo doc` 生成文档
- 掌握文档测试的进阶用法

## 常用属性

属性是 `#[...]` 或 `#![...]` 语法，向编译器传递指令。

### 编译优化提示

```rust
#[inline]                  // 提示编译器内联
#[inline(always)]          // 强制内联（小心代码膨胀）
#[cold]                    // 标记冷路径：panic 处理、错误日志
#[track_caller]            // panic 报告调用者的位置
```

### 结构体和枚举语义属性

```rust
#[non_exhaustive]       // 禁止外部穷尽匹配——将来加变体不破坏兼容
pub enum Error {
    NotFound,
    Permission,
    // 下个版本可以加 Timeout，下游代码不会编译失败
}

// 下游使用者必须加通配分支（否则编译错误）：
// match my_lib::Error {
//     NotFound => ...,
//     Permission => ...,
//     _ => ...,       // ← 编译器强制要求
// }

#[repr(C)]              // 按 C ABI 布局（FFI 必须用）
pub struct CCompatible { a: u8, b: u32, c: u16 }

#[repr(transparent)]    // 单字段结构体，ABI 与内部类型完全相同
#[repr(transparent)]
pub struct UserId(u64);

// 其他常用 repr：
// #[repr(u8)] / #[repr(i32)] — 为无数据枚举指定判别值类型
// #[repr(packed)] — 紧凑布局，无填充字节（可能有对齐问题）
// #[repr(align(N))] — 指定对齐要求
```

### must_use — 强制处理返回值

```rust
#[must_use = "此操作可能失败，请检查返回值"]
pub fn connect(addr: &str) -> Result<Connection, Error> { /* ... */ }

connect("localhost:8080");        // ⚠️ Warning: unused return value
let _ = connect("localhost:8080"); // 显式忽略（下划线前缀）
let conn = connect("localhost:8080")?; // ✅ 正确处理
```

`Result` 和 `Future` 默认已标注 `#[must_use]`。你也可以标注类型本身：

```rust
#[must_use = "Connection 不持有后不会生效"]
pub struct Connection { /* ... */ }
```

### 废弃 API

```rust
#[deprecated(since = "2.0.0", note = "请使用 `new_api` 代替")]
pub fn old_api() { }
// 调用时产生编译警告，引导用户迁移
```

### Lint 控制

```rust
// 作用在下一项
#[allow(dead_code)]
#[warn(clippy::all)]

// 作用在 crate/模块 (用 !)
#![allow(dead_code)]
#![deny(missing_docs)]       // 缺少文档 → 编译错误
#![forbid(unsafe_code)]      // 禁止 unsafe——安全敏感项目
// forbid 比 deny 更严格：不允许下游 override
```

**lint 级别从低到高：**
- `allow` — 不报告
- `warn` — 编译警告（默认）
- `deny` — 编译错误
- `forbid` — 编译错误 + 不允许 override

### derive — 自动实现 trait

```rust
// 最常用组合
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct MyStruct {
    field: String,
}

// 可 derive 的：Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
//              Hash, Default
// 需手动实现的：Display(精细), Drop, Deref, Iterator, From(复杂)
```

> 💡 `#[derive]` 生成的是最 naive 的实现（例如 `PartialEq` 会逐字段比较）。当需要定制行为时（如比较时忽略某个字段），应手动实现 trait。

## 文档注释

Rust 的文档是一等公民。`///` 注释由 `cargo doc` 生成 HTML，代码示例由 `cargo test` 执行。

### 项级文档 `///`

```rust
/// 从给定路径加载配置文件。
///
/// # 示例
///
/// ```
/// use mylib::load_config;
/// let config = load_config("config.toml").unwrap();
/// assert_eq!(config.port, 8080);
/// ```
///
/// # Errors
///
/// 以下情况返回 `ConfigError`：
/// - 文件不存在 → `ConfigError::NotFound`
/// - TOML 格式错误 → `ConfigError::Parse`
///
/// # Panics
///
/// 该函数不会 panic。
pub fn load_config(path: impl AsRef<Path>) -> Result<Config, ConfigError> {
    // ...
}
```

**文档注释常用 section：**
- `# Examples` — 可运行示例（`cargo test` 会执行）
- `# Errors` — 何时返回错误
- `# Panics` — 何种情况会 panic
- `# Safety` — unsafe 函数的安全条件（必须写）
- `# See Also` — 相关函数/类型的链接

### 模块级文档 `//!`

```rust
//! # 配置模块
//!
//! 负责加载、验证、提供运行时配置。
//!
//! ## Feature Flags
//!
//! - `hot_reload`: 启用热重载

pub mod config { }
```

- `///` 是**项级文档**（document the following item）
- `//!` 是**模块/crate 级文档**（document the enclosing scope）

### 文档链接和属性

```rust
// 文档内链接
/// 连接到标准库：[`std::fs::File`]
/// 连接本 crate 项：[`Config`]
/// 带自定义文字：[连接配置](crate::config)

// 文档属性
#[doc(hidden)]           // 在文档中隐藏——内部 API
#[doc(alias = "path")]   // 搜索别名——用户搜 "path" 也能找到
pub fn internal_stuff() { }
```

```bash
cargo doc --open                  # 生成文档并在浏览器打开
cargo doc --no-deps               # 只看自己的 crate
cargo doc --document-private-items # 连私有项也生成
cargo test --doc                  # 只运行文档测试
```

### 文档测试进阶

````rust
/// ```rust,should_panic
/// divide(1, 0);  // 应 panic
/// ```
///
/// ```rust,no_run
/// connect("https://example.com");  // 编译但不运行（需要网络）
/// ```
///
/// ```rust,ignore
/// 这段不编译也不运行
/// ```
///
/// ```rust,compile_fail
/// 这段应该编译失败
/// ```
pub fn divide(a: i32, b: i32) -> i32 { a / b }
````

> 📘 `cargo test` 会把文档中的示例代码作为测试运行，这是 Rust 文档"永远不过期"的秘诀——文档示例无法编译时会直接报错。

## 练习

1. 为公开 API 添加 `#[must_use]` 和完整的文档注释（含 `# Examples`、`# Errors`），用 `cargo doc --open` 查看效果
2. 写一个 `#[non_exhaustive]` 枚举，观察到外部 crate 使用时编译器强制要求写 `_ =>` 分支
3. 尝试 `#![deny(missing_docs)]`，为你的 crate 补齐所有文档
4. 写一个带 `#[deprecated]` 标注的旧函数，调用它并观察编译器警告

---

← [第 17 章：Cargo 进阶](./17-cargo-advanced.md) | [返回目录](./README.md) | → [第 19 章：宏](./19-macros.md)
