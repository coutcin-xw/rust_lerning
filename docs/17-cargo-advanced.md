# 第 17 章：Cargo 进阶

## 学习目标

- 使用条件编译 `#[cfg]` 和 feature flags 管理可选功能
- 深入理解 Cargo.toml 中的 feature 定义、传递和合并规则
- 用 Cargo Workspace 组织多 crate 项目
- 掌握 workspace 级别的依赖共享和版本管理

## 回顾：这章在什么位置？

上一章讲了 **Module**——单个 crate 内部的代码组织。这一章往上走一层，讲 **Package 和 Workspace** 级别的工程实践。

```
Workspace ← 本章（多 package 管理）
└── Package ← 本章（条件编译、feature）
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

Feature flag 是 Rust 的**条件编译开关**。它的核心用途：**让你的 crate 提供可选功能，用户只编译他们需要的部分。**

为什么需要 feature flags？
- 库的某些功能依赖较重的第三方 crate（如 `serde`、`tokio`），但并非所有用户都需要
- 同一套代码需要支持多种配置（有/无 TLS、同步/异步）
- 减少编译时间和二进制体积——不需要的代码根本不编译

#### Cargo.toml 配置详解

```toml
[package]
name = "my-lib"
version = "0.1.0"
edition = "2024"

# ─── Feature 定义 ───
[features]
# 默认启用的 feature 集合（用户不指定时生效）
default = ["std"]

# 纯标记 feature：不依赖其他 feature，也不启用额外依赖
# 用于代码中的 #[cfg(feature = "std")] 判断
std = []

# 启用特定可选依赖的 feature
# "dep:tokio" 语法：启用名为 tokio 的可选依赖
async = ["dep:tokio"]

# 一个 feature 可以同时启用多个依赖
# 这里同时启用 serde 和 serde_json 两个可选依赖
serde = ["dep:serde", "dep:serde_json"]

# 纯标记 feature（不关联依赖，仅用于代码中的 cfg 判断）
tls = []

# 组合 feature：一键启用一组功能
# 用户写 features = ["full"] 等价于启用 std + async + serde + tls
full = ["std", "async", "serde", "tls"]

# ─── 依赖声明 ───
[dependencies]
# optional = true：这个依赖默认不编译
# 只有某个 feature 通过 "dep:tokio" 启用它时才编译
tokio = { version = "1", optional = true }

# 即使被 feature 启用，依赖本身也可以有自己的 feature 配置
serde = { version = "1", optional = true, features = ["derive"] }

serde_json = { version = "1", optional = true }

# 非可选的依赖总是编译
anyhow = "1"
```

#### 逐行解读

**`default = ["std"]`**
当用户写 `cargo build`（不加 `--features`）时，默认启用 `std` feature。如果不想任何默认行为，设置 `default = []`，用户需要 `cargo build --no-default-features`。

**`std = []`**
一个"纯标记" feature。`[]` 表示它自己不启用任何额外依赖，只是作为一个标记名存在。代码中通过 `#[cfg(feature = "std")]` 判断是否启用。常见于 `no_std` 库（嵌入式场景，关闭标准库）。

**`async = ["dep:tokio"]`**
`dep:xxx` 是启用可选依赖的语法。当用户启用 `async` feature 时，`tokio` 依赖被激活并编译。注意：`dep:tokio` 中的名字对应 `[dependencies]` 中声明的包名，不是 feature 名。

**`serde = ["dep:serde", "dep:serde_json"]`**
一个 feature 可以同时启用多个可选依赖。当用户只需要序列化功能时，他们不需要引入整个 `full` feature 的其他依赖。

**依赖自身也有 features**
`serde = { version = "1", optional = true, features = ["derive"] }` 中：
- `optional = true`：默认不编译此依赖
- `features = ["derive"]`：**当这个依赖被启用时**，同时启用 serde 自身的 `derive` feature

#### Feature 的传递和合并

当你的 crate 被其他 crate 依赖时：

```toml
# 你的 crate：my-lib
[features]
serde = ["dep:serde"]

# 下游用户：their-app
[dependencies]
my-lib = { version = "1", features = ["serde"] }
# 用户不需要在自己的 Cargo.toml 里声明 serde
# Cargo 会自动合并——把 my-lib 的 serde feature 和 serde 依赖一起启用
```

#### `pkg/feature` — 传递子依赖自身的 feature

`serde/derive` 这种语法的含义：启用 `serde` 这个依赖的 `derive` feature。

**什么时候需要它？** 看一个具体场景：

假设你的 crate `my-lib` 暴露了一个可序列化的结构体：

```rust
// my-lib/src/lib.rs
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]  // ← 这里需要 serde 的 derive feature
pub struct Config {
    pub port: u16,
}
```

这个 `#[derive(Serialize)]` 只有在 `serde` 被启用 **且** `serde` 的 `derive` feature 也启用时才能编译。你的 feature 定义有两种写法：

```toml
[features]
# ❌ 写法 1：只启用 serde，不传递 derive
serde = ["dep:serde"]
# 结果：serde 本身被编译了，但 #[derive(Serialize)] 用不了
# 因为 serde 的 derive feature 没开

# ✅ 写法 2：同时传递 derive feature
serde = ["dep:serde", "serde/derive"]
#         ^^^^^^^^^^^^^  ^^^^^^^^^^^^^^
#         启用 serde 依赖   同时打开 serde 的 derive feature
```

> 💡 `serde/derive` 中的 `/` 是 Cargo feature 语法的命名空间分隔符：`依赖名/feature名`。这与 `dep:serde` 的 `dep:` 前缀是平行的语法——两者都用来引用当前 crate 的依赖树中的项。

**如果不传递会怎样？**

下游用户 `their-app` 依赖你的 `my-lib` 并启用 `serde` feature 时：

```toml
[dependencies]
my-lib = { version = "1", features = ["serde"] }
```

- 你的库 `my-lib` 会启用 `serde` 依赖
- 但 `serde` 的 `derive` feature 没被打开
- `#[derive(Serialize)]` 编译失败——**用户看到的是你的库编译报错**

**总结规则：如果你的 feature 对应的代码中，用到了依赖自身的 feature（如 serde 的 derive、tokio 的 macros 等），就必须在 `[features]` 中显式传递。**

#### 代码中使用

```rust
// 条件函数——不同 feature 对应不同实现
#[cfg(feature = "async")]
pub async fn process(data: &[u8]) -> Result<Output, Error> {
    // 使用 tokio 的异步实现
}

#[cfg(not(feature = "async"))]
pub fn process(data: &[u8]) -> Result<Output, Error> {
    // 同步回退实现
}

// 条件结构体字段
pub struct Client {
    base_url: String,
    timeout: u64,

    #[cfg(feature = "tls")]
    tls_config: TlsConfig,  // 只在 tls feature 启用时存在
}

// 条件导入
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
}
// 没有 serde feature 时，Config 不实现 Serialize/Deserialize

// 条件模块——整个模块只在某个 feature 下编译
#[cfg(feature = "std")]
pub mod sync_impl { /* ... */ }

#[cfg(not(feature = "std"))]
pub mod no_std_impl { /* ... */ }
```

#### 常用 cargo 命令

```bash
cargo build --features "async serde"   # 启用指定 feature
cargo build --all-features             # 启用所有 feature
cargo build --no-default-features      # 不启用任何默认 feature
cargo build --no-default-features --features "std,tls"  # 手工挑选

cargo test --all-features              # 用全部 feature 配置跑测试
cargo doc --all-features               # 生成包含所有 feature 的文档
```

#### Feature Flag 最佳实践

> ⚠️ **feature 应该具有可加性（additive）。** 启用一个 feature 不应该移除功能或改变现有 API 的行为。如果用户写 `--all-features`，所有 feature 的组合应该是合法的。

> ⚠️ **避免 feature 之间互斥。** 不要设计 "启用 A 就不能启用 B" 的 feature。如果确实需要二选一，考虑用编译期错误提示。

> ⚠️ **为每个 feature 编写测试。** CI 中应该用 `--all-features` 跑一遍，也要用 `--no-default-features` 跑一遍，确保各种组合都能编译。

> 💡 **在文档中清楚说明每个 feature 的作用。** 用户看到 `[features]` 列表时不应该需要猜每个 feature 干什么。

#### 实战：带 feature 门控的 Client

```rust
/// HTTP 客户端构造器
///
/// # Features
///
/// - `tls`: 启用 TLS/HTTPS 支持
/// - `gzip`: 启用响应自动解压缩
pub struct ClientBuilder {
    url: String,
    timeout: u64,

    // 这些字段只在对应 feature 启用时存在
    #[cfg(feature = "tls")]
    tls_enabled: bool,

    #[cfg(feature = "gzip")]
    accept_gzip: bool,
}

impl ClientBuilder {
    pub fn new(url: impl Into<String>) -> Self {
        ClientBuilder {
            url: url.into(),
            timeout: 30,
            #[cfg(feature = "tls")]
            tls_enabled: true,   // 默认开启 TLS（如果 feature 存在）
            #[cfg(feature = "gzip")]
            accept_gzip: true,
        }
    }

    // 这个方法只在 tls feature 启用时才存在
    #[cfg(feature = "tls")]
    pub fn tls(mut self, enabled: bool) -> Self {
        self.tls_enabled = enabled;
        self
    }

    // 这个方法也一样
    #[cfg(feature = "gzip")]
    pub fn gzip(mut self, enabled: bool) -> Self {
        self.accept_gzip = enabled;
        self
    }

    pub fn build(self) -> Client {
        Client {
            inner: HttpClient::new(&self.url),
            timeout: self.timeout,
            #[cfg(feature = "tls")]
            tls: self.tls_enabled.then(TlsConfig::default),
            #[cfg(feature = "gzip")]
            gzip: self.accept_gzip,
        }
    }
}
```

```toml
# Cargo.toml 对应的 feature 配置
[features]
default = ["tls"]
tls = ["dep:rustls"]        # TLS 需要 rustls
gzip = ["dep:flate2"]        # Gzip 需要 flate2
full = ["tls", "gzip"]

[dependencies]
rustls = { version = "0.23", optional = true }
flate2 = { version = "1", optional = true }
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

### workspace 继承 — 可共享的属性

根 `Cargo.toml` 的 `[workspace.package]` 可以定义公用的包元数据，子 crate 通过 `key.workspace = true` 继承：

```toml
# 根 Cargo.toml
[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT"
repository = "https://github.com/user/my_project"
rust-version = "1.85"
authors = ["团队 <team@example.com>"]

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

```toml
# core/Cargo.toml — 继承所有共享属性
[package]
name = "core"
version.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true

[dependencies]
serde = { workspace = true }    # 继承共享的 serde 声明
```

**可继承的属性：** `version`、`edition`、`license`、`repository`、`rust-version`、`authors`、`description`、`homepage`、`documentation`、`keywords`、`categories`、`publish`、`exclude`、`include`。

### 成员间依赖与 feature 传递

workspace 成员可以互相依赖，并通过 feature 控制内部行为：

```
my_project/
├── core/                   # 核心库：定义了 add/init 基础功能
│   └── src/lib.rs
├── cli/                    # CLI 工具：依赖 core，使用 core 的公共 API
│   └── src/main.rs
└── web/                    # Web 服务：依赖 core，启用 core 的 "json" feature
    └── src/main.rs
```

```toml
# core/Cargo.toml — 提供可选功能
[features]
default = []
json = ["dep:serde_json"]    # 开启 JSON 支持

[dependencies]
serde_json = { version = "1", optional = true }
```

```rust
// core/src/lib.rs
pub fn init() { println!("core 初始化"); }

#[cfg(feature = "json")]
pub fn to_json<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap()
}
```

```toml
# web/Cargo.toml — 依赖 core，启用 json feature
[dependencies]
core = { path = "../core", features = ["json"] }
tokio = { workspace = true }
```

```toml
# cli/Cargo.toml — 依赖 core，不启用 json feature
[dependencies]
core = { path = "../core" }   # 默认不使用 json
clap = "4"
```

```rust
// cli/src/main.rs — 使用 core 的公共 API
fn main() {
    core::init();
    println!("CLI 工具已启动");
}
```

```rust
// web/src/main.rs — 使用 core + json 功能
#[tokio::main]
async fn main() {
    core::init();
    let data = vec![1, 2, 3];
    println!("JSON: {}", core::to_json(&data));  // ✅ web 启用了 json feature
}
```

```bash
cargo run -p cli                         # 只编译 core + cli（不编译 web）
cargo run -p web                         # 编译 core + web（core 启用 json）
cargo build --workspace                  # 全部编译
```

### 排除成员 + patches

```toml
# 根 Cargo.toml
[workspace]
members = ["core", "cli", "web"]
exclude = ["experiments", "deprecated"]  # 不参与 workspace 的子目录

# 统一替换某个依赖（调试、fix 紧急 bug 时用）
[patch.crates-io]
serde = { git = "https://github.com/serde-rs/serde", branch = "fix-bug" }
```

### workspace 常用命令速查

| 命令 | 作用 |
|------|------|
| `cargo build --workspace` | 编译所有成员 |
| `cargo test -p <name>` | 只测试指定 crate |
| `cargo run -p <name>` | 运行指定 bin crate |
| `cargo check -p <name>` | 只检查指定 crate（快） |
| `cargo doc --workspace --no-deps` | 为所有成员生成文档 |
| `cargo clippy --workspace` | 对所有成员运行 clippy |

## 练习

1. 创建一个 workspace：一个 lib crate + 一个 bin crate
2. 为库添加 `serde` feature flag，条件编译序列化支持
3. 用 `#[cfg(target_os)]` 写跨平台的路径处理函数
4. 尝试 `cargo build --no-default-features` 和 `cargo build --all-features`，观察编译结果差异

---

← [第 16 章：模块系统](./16-modules.md) | [返回目录](./README.md) | → [第 18 章：属性与文档](./18-attributes-docs.md)
