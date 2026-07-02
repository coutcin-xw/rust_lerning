# 第 16 章：项目工程化

## 学习目标

- 使用条件编译 `#[cfg]` 和 feature flags 管理可选功能
- 用 Cargo Workspace 组织多 crate 项目
- 掌握常用属性（`must_use`、`non_exhaustive`、`repr`、lint 控制）
- 编写规范的文档注释（`///`、`//!`），用 `cargo doc` 生成文档

## 回顾：这章在什么位置？

上一章讲了 **Module**——单个 crate 内部的代码组织。这一章往上走一层，讲 **Package 和 Workspace** 级别的工程实践。

```
Workspace ← 本章（多 package 管理）
└── Package ← 本章（条件编译、feature、属性配置、文档）
    └── Crate
        └── Module ← 上一章
```

## 条件编译

条件编译让你根据操作系统、CPU 架构、feature、编译模式来选择性包含代码。

### #[cfg] — 编译期条件

`#[cfg]` 作用在**下一个项**上。常用谓词：

```rust
// 操作系统
#[cfg(target_os = "linux")]
fn platform_init() { /* Linux 代码 */ }
#[cfg(target_os = "windows")]
fn platform_init() { /* Windows 代码 */ }
#[cfg(target_os = "macos")]
fn platform_init() { /* macOS 代码 */ }

// CPU 架构
#[cfg(target_arch = "x86_64")]
fn simd_impl() { }
#[cfg(target_arch = "aarch64")]
fn simd_impl() { }

// 编译模式
#[cfg(debug_assertions)]     // debug / cargo test
fn debug_checks() { }
#[cfg(not(debug_assertions))] // release 模式
fn debug_checks() { }

// 测试
#[cfg(test)]
mod tests { }

// 组合条件
#[cfg(all(unix, not(target_os = "macos")))]
#[cfg(any(windows, target_os = "linux"))]
```

### cfg! 宏 — if 条件中的编译期判断

```rust
if cfg!(target_os = "windows") {
    println!("路径分隔符: \\");
} else {
    println!("路径分隔符: /");
}
// cfg! 在编译时展开为 true/false，不可达分支在 release 中被优化掉
```

**#[cfg] vs cfg! 的区别：**

| | `#[cfg(条件)]` | `cfg!(条件)` |
|---|---|---|
| 作用位置 | 项（函数、结构体、模块...） | 表达式内 |
| 不满足时 | 代码**不编译**——可以写平台特有类型 | 编译但不会执行 |
| 典型场景 | 排除不兼容平台的代码 | `if`/`else` 小段条件 |

### #[cfg_attr] — 条件属性

```rust
#[cfg_attr(test, derive(PartialEq))]  // 仅测试时派生
struct InternalData { /* ... */ }

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct Config { /* ... */ }
```

### Feature Flags — 可选功能模块

在 `Cargo.toml` 中：

```toml
[features]
default = ["std"]
std = []
async = ["dep:tokio"]
serde = ["dep:serde", "dep:serde_json"]
tls = []
full = ["std", "async", "serde", "tls"]

[dependencies]
tokio = { version = "1", optional = true }
serde = { version = "1", optional = true, features = ["derive"] }
serde_json = { version = "1", optional = true }
```

在代码中使用：

```rust
// 条件函数实现
#[cfg(feature = "async")]
pub async fn process(data: &[u8]) -> Result<Output, Error> { /* ... */ }

#[cfg(not(feature = "async"))]
pub fn process(data: &[u8]) -> Result<Output, Error> { /* ... */ }

// 条件结构体字段
pub struct Client {
    base_url: String,
    #[cfg(feature = "tls")]
    tls_config: TlsConfig,
}

// 条件导入
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
```

```bash
cargo build --features "async serde"
cargo build --no-default-features
cargo test --all-features
```

### 实战：feature 门控

```rust
pub struct ClientBuilder {
    url: String,
    timeout: u64,
    #[cfg(feature = "tls")]
    tls_enabled: bool,
}

impl ClientBuilder {
    pub fn new(url: impl Into<String>) -> Self {
        ClientBuilder {
            url: url.into(),
            timeout: 30,
            #[cfg(feature = "tls")]
            tls_enabled: false,
        }
    }

    // 仅 tls feature 启用时才存在此方法
    #[cfg(feature = "tls")]
    pub fn tls(mut self, enabled: bool) -> Self {
        self.tls_enabled = enabled;
        self
    }

    pub fn build(self) -> Client { /* ... */ }
}
```

## Cargo Workspace — 多 crate 管理

### 何时拆分 Workspace？

| 信号 | 方案 |
|------|------|
| 多个二进制程序共享核心逻辑 | workspace：lib crate + 多个 bin crate |
| API 层和实现层需隔离 | workspace：api crate + impl crate |
| 某个模块编译越来越慢 | workspace：拆出独立 crate 并行编译 |
| 库对外发布，同时自己有 CLI | workspace：lib crate + cli bin crate |
| 只是代码多了想整理 | ❌ 用 `mod` 组织即可，别过早拆分 |

### 目录结构

```
my_project/
├── Cargo.toml              # workspace 根（只有 [workspace]）
├── Cargo.lock              # 整个 workspace 共享
├── target/                 # 共享编译输出
├── core/                   # 核心库 crate
│   ├── Cargo.toml
│   └── src/lib.rs
├── cli/                    # CLI 二进制 crate
│   ├── Cargo.toml
│   └── src/main.rs
└── web/                    # Web 服务二进制 crate
    ├── Cargo.toml
    └── src/main.rs
```

根 `Cargo.toml`：
```toml
[workspace]
members = ["core", "cli", "web"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

子 crate 引用：
```toml
# cli/Cargo.toml
[package]
name = "my-cli"
version.workspace = true
edition.workspace = true

[dependencies]
core = { path = "../core" }          # 内部依赖
serde = { workspace = true }         # 继承公共版本
anyhow = { workspace = true }
clap = "4"                           # 仅此 crate 独有的依赖
```

```bash
cargo build --workspace          # 编译所有成员
cargo test -p core               # 只测试 core
cargo run -p cli -- --verbose    # 运行 cli，-- 分隔传递参数
```

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

#[repr(C)]              // 按 C ABI 布局（FFI 必须用）
pub struct CCompatible { a: u8, b: u32, c: u16 }

#[repr(transparent)]    // 单字段结构体，ABI 与内部类型完全相同
#[repr(transparent)]
pub struct UserId(u64);
```

### must_use — 强制处理返回值

```rust
#[must_use = "此操作可能失败，请检查返回值"]
pub fn connect(addr: &str) -> Result<Connection, Error> { /* ... */ }

connect("localhost:8080");        // ⚠️ Warning: unused return value
let _ = connect("localhost:8080"); // 显式忽略（下划线前缀）
let conn = connect("localhost:8080")?; // ✅ 正确处理
```

`Result` 和 `Future` 默认已标注 `#[must_use]`。

### 废弃 API

```rust
#[deprecated(since = "2.0.0", note = "请使用 `new_api` 代替")]
pub fn old_api() { }
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
pub fn divide(a: i32, b: i32) -> i32 { a / b }
````

## 练习

1. 创建一个 workspace：一个 lib crate + 一个 bin crate
2. 为库添加 `serde` feature flag，条件编译序列化支持
3. 用 `#[cfg(target_os)]` 写跨平台的路径处理函数
4. 为公开 API 添加 `#[must_use]` 和文档注释，用 `cargo doc --open` 验证
5. 写一个 `#[non_exhaustive]` 枚举，观察外部 crate 使用时编译器强制要求写 `_ =>` 分支

---

← [第 15 章：模块系统](./15-modules.md) | [返回目录](./README.md) | → [第 17 章：宏](./17-macros.md)
