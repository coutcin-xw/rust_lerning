# 第 15 章：模块系统

## 学习目标

- 使用 `mod` 组织代码为模块树
- 理解 Rust 的私有性规则（默认私有，`pub` 公开）
- 使用 `use` 简化路径引用
- 掌握 `pub use` 重新导出构建干净的公开 API
- 组织标准的库和二进制项目结构

## 模块基础

模块是 Rust 组织代码的单元。默认一切**私有**。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() { }
        fn seat_at_table() { }     // 私有，外部不可见
    }

    mod serving {
        fn take_order() { }
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

> 💡 Rust 的默认私有不只是模块——结构体字段、枚举变体、函数，默认都私有。这与 Java/C++ 默认 public 的理念完全不同。

## 可见性控制

```rust
pub mod outer {
    pub fn public_fn() { }                // 任何地方可见
    pub(crate) fn crate_fn() { }          // 当前 crate 内可见
    pub(super) fn parent_fn() { }         // 父模块可见
    pub(in crate::outer) fn inner_fn() { } // 指定路径内可见
    fn private_fn() { }                   // 仅本模块可见（默认）
}
```

| 修饰符 | 可见范围 | 使用场景 |
|--------|---------|---------|
| 默认（无） | 当前模块及子模块 | 内部实现细节 |
| `pub` | 任何地方 | 公开 API 入口 |
| `pub(crate)` | 当前 crate | 内部共享、不对外暴露 |
| `pub(super)` | 父模块 | 子模块给父模块的特权访问 |
| `pub(in path)` | 指定路径内 | 精细化控制 |

### 结构体字段的可见性

```rust
pub struct User {
    pub username: String,     // 公开
    pub(crate) role: Role,    // crate 内可见
    email: String,            // 私有
    password_hash: String,    // 私有
}

impl User {
    // 公共构造函数——控制如何创建 User
    pub fn new(username: String, email: String, password: &str) -> Self {
        User {
            username,
            email,
            role: Role::default(),
            password_hash: hash_password(password),
        }
    }
}
```

## use —— 导入路径

```rust
use std::collections::HashMap;
use std::io::{self, Read, Write};   // 批量导入
use std::fmt::*;                     // 通配符（谨慎使用）
use std::fs::File as FsFile;         // as 别名（解决命名冲突）

fn main() {
    let mut map = HashMap::new();
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
}
```

### 导入惯例

| 导入目标 | 推荐方式 | 示例 |
|---------|---------|------|
| 函数 | 导入父模块 | `use std::io;` → `io::stdin()` |
| 类型/结构体/枚举 | 导入自身 | `use std::collections::HashMap;` |
| trait | 导入自身（需要它的方法） | `use std::io::Read;` |
| 同名冲突 | `as` 别名 | `use foo::Config as FooConfig;` |

> 💡 为什么函数要导入父模块？因为 `stdin()` 这个名字太泛了。`io::stdin()` 一看就知道是 I/O 操作。

## 文件模块组织

### 模块查找规则

```rust
mod network;  // 编译器查找：
// 1. src/network.rs
// 2. src/network/mod.rs

mod network::server;  // 编译器查找：
// 1. src/network/server.rs
// 2. src/network/server/mod.rs
```

### 推荐的项目结构

**库项目：**
```
my_lib/
├── Cargo.toml
└── src/
    ├── lib.rs              # 库根 → crate 根
    │   pub mod types;      # 声明子模块
    │   pub mod error;
    │   pub mod engine;
    │   // pub use 提供干净 API（见下文）
    │
    ├── types.rs             # 公开类型定义
    ├── error.rs             # 错误类型
    └── engine/
        ├── mod.rs           # engine 模块入口
        ├── parser.rs
        └── executor.rs
```

**二进制项目：**
```
my_app/
├── Cargo.toml
└── src/
    ├── main.rs              # 入口：解析参数 + 调用 lib
    │   fn main() {
    │       let config = my_app::config::parse();
    │       my_app::run(config);
    │   }
    ├── lib.rs               # 核心逻辑（方便测试和复用）
    ├── config.rs
    ├── cli.rs               # CLI 参数解析
    └── app/
        ├── mod.rs
        └── service.rs
```

> 💡 **最佳实践：** 把核心逻辑放 `lib.rs`，`main.rs` 保持精简。这样你的代码既可作为库被测试，也可作为二进制使用。

## pub use —— 重新导出

隐藏内部结构，提供干净的公开 API：

```rust
// 内部结构复杂
pub mod engine {
    pub mod parser {
        pub struct Query { /* ... */ }
    }
    pub mod executor {
        pub struct Executor { /* ... */ }
    }
}

// 通过 pub use 提供扁平的 API
pub use engine::parser::Query;
pub use engine::executor::Executor;

// 用户只需：
// use my_lib::Query;  ← 干净的导入路径
// 而不是：
// use my_lib::engine::parser::Query;  ← 暴露了内部结构
```

## 模块设计模式

### 模式 1：类型 + impl 分离

```rust
// types.rs —— 定义数据
pub struct User {
    pub username: String,
    email: String,
}

// core.rs —— 实现核心逻辑
use super::types::User;
impl User {
    pub fn verify_email(&self) -> bool { /* ... */ }
}

// serialization.rs —— 可选功能
#[cfg(feature = "serde")]
impl Serialize for User { /* ... */ }
```

### 模式 2：Facade 模式

```rust
// lib.rs
mod internal;  // 私有模块
pub use internal::PublicApi;  // 只暴露需要公开的

// internal/mod.rs
pub mod impl_a;  // 实现细节
pub mod impl_b;
pub use impl_a::PublicApi;  // 控制暴露面
```

## 跨语言对比

| 概念 | Rust | Java | Python | C++ |
|------|------|------|--------|-----|
| 组织单元 | `mod` + 文件系统 | `package` + 文件系统 | 文件 = 模块 | `namespace` + `#include` |
| 默认可见性 | 私有 | 包内可见 | 公开 | 公开 |
| 导入 | `use` | `import` | `import` | `using` |
| 重新导出 | `pub use` | 无直接对应 | `__all__` | 无直接对应 |
| 路径 | `crate::` / `self::` / `super::` | `com.example.` | `.` / `..` | `::` |

## 常见陷阱

> ⚠️ **忘记 `pub` 导致外部不可见。** 需要的模块、函数、字段都要加 `pub`。结构体 `pub` 不等于其字段 `pub`。

> ⚠️ **通配符导入 `use xxx::*` 造成命名污染。** 在模块内部使用可以，在公开 API 中避免。

> ⚠️ **循环引用。** `mod a` 中 `use crate::b`，`mod b` 中 `use crate::a`。Rust 不支持循环模块依赖——需要重构。

## 条件编译

条件编译让你根据操作系统、CPU 架构、feature、编译模式来选择性包含或排除代码。这是跨平台项目的基础设施。

### #[cfg] — 编译期条件

`#[cfg]` 作用在**下一个项**上。常用谓词：

```rust
// 操作系统
#[cfg(target_os = "linux")]
fn platform_init() { /* Linux 特定代码 */ }
#[cfg(target_os = "windows")]
fn platform_init() { /* Windows 特定代码 */ }
#[cfg(target_os = "macos")]
fn platform_init() { /* macOS 特定代码 */ }

// CPU 架构
#[cfg(target_arch = "x86_64")]
fn arch_specific() { }
#[cfg(target_arch = "aarch64")]
fn arch_specific() { }

// 编译模式
#[cfg(debug_assertions)]  // debug 模式（含 cargo test）
fn debug_checks() { }
#[cfg(not(debug_assertions))]  // release 模式
fn debug_checks() { }

// 测试模式
#[cfg(test)]
mod tests { /* 只在 cargo test 时编译 */ }

// 组合条件
#[cfg(all(unix, not(target_os = "macos")))]  // Linux/BSD 但不是 macOS
#[cfg(any(windows, target_os = "linux"))]
#[cfg(not(any(target_os = "android", target_os = "ios")))]
```

### cfg! 宏 — 运行时 bool（但不产生运行时开销）

```rust
if cfg!(target_os = "windows") {
    println!("Windows 路径: C:\\...");
} else {
    println!("Unix 路径: /...");
}
// cfg! 在编译时展开为 true/false
// 不可达分支在 release 模式中会被 LLVM 优化掉（死代码消除）
```

**#[cfg] vs cfg! 的区别：**

| | `#[cfg(条件)]` | `cfg!(条件)` |
|---|---|---|
| 作用位置 | 项（函数、结构体、模块...） | 表达式内（if 条件） |
| 时机 | 编译期决定代码是否包含 | 展开为 `true`/`false` |
| 编译 | 不满足就不编译，可能无法通过类型检查 | 两个分支都会编译 |

所以 `#[cfg]` 用于**排除不兼容平台的代码**，`cfg!` 用于**小段的条件逻辑**。

### #[cfg_attr] — 条件属性

在某种条件下才附加属性：

```rust
// 仅在测试时派生 PartialEq
#[cfg_attr(test, derive(PartialEq))]
struct InternalData { /* ... */ }

// 多个条件属性
#[cfg_attr(
    all(feature = "serde", not(target_arch = "wasm32")),
    derive(serde::Serialize, serde::Deserialize)
)]
struct Config { /* ... */ }
```

### Feature Flags — 可选功能模块

在 `Cargo.toml` 中：

```toml
[features]
default = ["std"]
std = []
async = ["dep:tokio"]          # 启用 tokio 依赖
serde = ["dep:serde", "dep:serde_json"]  # 启用一组依赖
tls = []                       # 纯标记 feature
full = ["std", "async", "serde", "tls"]

# 可选依赖（仅 feature 启用时才编译）
[dependencies]
tokio = { version = "1", optional = true }
serde = { version = "1", optional = true, features = ["derive"] }
serde_json = { version = "1", optional = true }
```

在代码中使用：

```rust
// 条件函数实现
#[cfg(feature = "async")]
pub async fn process(data: &[u8]) -> Result<Output, Error> {
    // 异步实现
}

#[cfg(not(feature = "async"))]
pub fn process(data: &[u8]) -> Result<Output, Error> {
    // 同步回退
}

// 条件结构体字段
pub struct Client {
    base_url: String,

    #[cfg(feature = "tls")]
    tls_config: TlsConfig,  // 只在启用 tls feature 时存在
}

// 条件导入
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
```

```bash
cargo build --features "async serde"
cargo build --no-default-features --features "std"
cargo test --all-features
cargo check --features "full"
```

### 真实模式：feature 门控的完整示例

```rust
// lib.rs
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

    #[cfg(feature = "tls")]
    pub fn tls(mut self, enabled: bool) -> Self {
        self.tls_enabled = enabled;
        self
    }

    pub fn build(self) -> Client {
        // 根据 feature 选择初始化逻辑...
    }
}
```

## Cargo Workspace — 管理多个 crate

当项目大到需要拆分为多个子 crate 时（一个库 + 多个二进制、或职责不同的库），Workspace 让它们共享编译缓存和锁文件。

### 何时拆分？

| 信号 | 方案 |
|------|------|
| 有多个二进制程序共享核心逻辑 | workspace：一个 lib crate + 多个 bin crate |
| 库的 API 层和实现层需要隔离 | workspace：api crate + impl crate |
| 某个模块编译时间过长 | workspace：拆出独立 crate 并行编译 |
| 库需要给外部使用，同时自己有 CLI | workspace：lib crate + cli bin crate |
| 只是代码多了想整理 | ❌ 用 `mod` 组织即可，不要过早拆分 |

### 目录结构

```
my_project/
├── Cargo.toml              # workspace 根（只有 [workspace] 段）
├── Cargo.lock              # 整个 workspace 共享
├── target/                 # 共享，所有成员编译到这里
├── core/                   # 核心库 crate
│   ├── Cargo.toml          # name = "my-core"
│   └── src/lib.rs
├── cli/                    # CLI 二进制 crate
│   ├── Cargo.toml          # name = "my-cli"；依赖 my-core
│   └── src/main.rs
└── web/                    # Web 服务二进制 crate
    ├── Cargo.toml          # 依赖 my-core + workspace 版本的 tokio
    └── src/main.rs
```

根 `Cargo.toml`：
```toml
[workspace]
members = ["core", "cli", "web"]
resolver = "2"

# 集中管理版本号
[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT"

# 集中管理公共依赖版本（避免不同 crate 用不同版本）
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```

子 crate 引用：
```toml
# cli/Cargo.toml
[package]
name = "my-cli"
version.workspace = true    # 继承 workspace.package.version
edition.workspace = true

[dependencies]
core = { path = "../core" }
serde = { workspace = true }  # 使用 workspace 定义的 serde 版本
anyhow = { workspace = true }
clap = "4"                    # 仅此 crate 用的依赖，直接写
```

```bash
cargo build --workspace          # 编译所有成员
cargo test -p core               # 只测试 core
cargo run -p cli                 # 只运行 cli
cargo run -p cli -- --verbose    # 传递参数给 cli（-- 分隔）
```

## 常用属性（Attributes）

### 编译优化提示

```rust
#[inline]                  // 提示编译器内联（跨 crate 也有效）
#[inline(always)]          // 强制内联（小心代码膨胀）
#[cold]                    // 标记冷路径：panic 处理、错误日志等
#[track_caller]            // panic 时报告调用者位置（而非此函数位置）

#[inline]
fn small_frequently_called(x: i32) -> i32 { x * 2 + 1 }

#[cold]
fn log_and_panic(msg: &str) -> ! {
    eprintln!("致命错误: {}", msg);
    panic!("{}", msg);
}
```

### 枚举和结构体的语义属性

```rust
#[non_exhaustive]       // 外部 crate 无法穷尽匹配——将来加变体不破坏兼容
pub enum Error {
    NotFound,
    Permission,
    // 下个版本可以加 Timeout，不会破坏下游代码
}

#[repr(C)]              // 按 C 语言 ABI 排列字段（FFI 传给 C 代码）
pub struct CCompatible {
    a: u8,
    b: u32,
    c: u16,
    // C 的布局：a(offset 0), padding 3 bytes, b(offset 4), c(offset 8), padding 2 bytes
    // 总大小 12 字节（而不是 Rust 默认可能重排的）
}

#[repr(transparent)]    // 单字段结构体，ABI 完全等于内部类型
#[repr(transparent)]
pub struct UserId(u64);
// UserId 和 u64 有相同的内存布局——可以安全地 transmute
```

### must_use — 强制处理返回值

```rust
#[must_use = "此操作可能失败，请检查返回值"]
pub fn connect(addr: &str) -> Result<Connection, Error> {
    // ...
}

// 调用者忽略返回值 → 编译警告
connect("localhost:8080");       // ⚠️ warning: unused return value
let _ = connect("localhost:8080"); // 显式忽略（用 _ 前缀）
let conn = connect("localhost:8080")?; // ✅ 正确处理
```

`Result` 和 `Future` 已经默认标注了 `#[must_use]`，所以 Rust 默认就会警告未处理的 Result。

### deprecated — 废弃 API

```rust
#[deprecated(since = "2.0.0", note = "请使用 `new_api()` 代替")]
pub fn old_api() { }

// 使用时：
old_api();  // ⚠️ warning: use of deprecated function `old_api`: 请使用 `new_api()` 代替
```

### Lint 控制属性

```rust
// 作用于下一项
#[allow(dead_code)]            // 允许未使用代码（常见于开发中）
#[allow(unused_variables)]     // 允许未使用变量
#[deny(missing_docs)]          // 缺少文档 → 编译错误
#[warn(clippy::all)]           // 启用 clippy lint

// 作用于整个模块/crate（用 ! 语法）
#![allow(dead_code)]           // crate 级别允许
#![deny(unsafe_code)]          // 禁止 unsafe——安全敏感项目
#![forbid(unsafe_code)]        // 禁止且不允许 override（比 deny 更严格）
```

### derive — 自动实现 trait

```rust
// 最常用的 derive 组合
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MyStruct {
    field: String,
}

// 不是所有 trait 都能 derive
// 可以 derive 的：Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
//                Hash, Default, Display(部分), From(部分)
// 需要手动实现的：Display(精细), Drop, Deref, Iterator, From(复杂)
```

## 文档注释

Rust 的文档系统是**一等公民**——`///` 注释由 `cargo doc` 生成 HTML，其中的代码示例由 `cargo test` 执行。

### 项级文档 `///`

```rust
/// 从给定路径加载配置文件。
///
/// 配置文件格式为 TOML。如果文件不存在或格式错误，返回错误。
///
/// # 示例
///
/// ```
/// use mylib::load_config;
///
/// let config = load_config("config.toml").unwrap();
/// assert_eq!(config.port, 8080);
/// ```
///
/// # Errors
///
/// 以下情况返回 `ConfigError`：
/// - 文件不存在 → `ConfigError::NotFound`
/// - TOML 格式错误 → `ConfigError::Parse(..)`
///
/// # Panics
///
/// 该函数不会 panic。
pub fn load_config(path: impl AsRef<Path>) -> Result<Config, ConfigError> {
    // ...
}
```

**文档注释中常用的 Markdown section：**
- `# Examples` — 可运行的代码示例（`cargo test` 会执行）
- `# Errors` — 什么时候返回错误
- `# Panics` — 什么情况下会 panic
- `# Safety` — unsafe 函数的调用安全条件（必须写）
- `# See Also` — 相关函数/类型的链接

### 模块级文档 `//!`

```rust
//! # 配置模块
//!
//! 本模块负责加载、验证、和提供运行时配置。
//!
//! ## 基本用法
//!
//! ```
//! let config = mylib::config::load("app.toml")?;
//! ```
//!
//! ## Feature Flags
//!
//! - `hot_reload`: 启用配置文件热重载

pub mod config {
    // ...
}
```

### 文档链接

```rust
/// 连接到标准库类型：[`std::fs::File`]，[`String`]
/// 连接到本 crate 的项：[`Config`]，[`load`]
/// 带自定义文字：[连接到配置模块](crate::config)
/// 省略号链接：[`Config::load`][]
///
/// [`Config::load`]: Config::load
```

### 文档级别的属性

```rust
#[doc(hidden)]          // 在文档中隐藏——内部 API 不想让用户看到
#[doc(alias = "path")]  // 添加搜索别名——用户搜 "path" 也能找到此函数
#[doc(hidden)]
pub fn internal_stuff() { }

#[doc(alias = "connect")]
pub fn establish_connection() { }  // 搜 connect 也能找到
```

```bash
cargo doc --open           # 生成并在浏览器打开
cargo doc --no-deps        # 只生成自己的 crate（不看依赖）
cargo doc --document-private-items  # 连私有项也生成文档
cargo test --doc           # 只运行文档测试
```

### 文档测试的高级用法

````rust
/// ```rust,should_panic
/// # use mylib::divide;
/// divide(1, 0);  // 这段代码应该 panic
/// ```
///
/// ```rust,no_run
/// # use mylib::connect;
/// connect("https://example.com");  // 不在测试时运行（需要网络）
/// ```
///
/// ```rust,ignore
/// // 这段代码被忽略——编译都不过
/// not_even_valid_rust();
/// ```
pub fn divide(a: i32, b: i32) -> i32 { a / b }
````

## 练习

1. 创建一个 workspace，包含一个库 crate 和一个二进制 crate
2. 为库添加 `serde` feature flag，条件编译序列化支持
3. 用 `#[cfg(target_os)]` 写跨平台的路径处理函数
4. 为你的公开 API 添加 `#[must_use]` 和文档注释，用 `cargo doc --open` 查看效果

---

← [第 14 章：异步编程](./14-async-await.md) | [返回目录](./README.md) | → [第 16 章：宏](./16-macros.md)
