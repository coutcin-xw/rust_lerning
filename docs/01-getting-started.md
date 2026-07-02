# 第 1 章：环境搭建

## 学习目标

- 在所有主流平台上安装 Rust 工具链（版本 ≥ 1.85.0）
- 理解 rustup、rustc、cargo 各自的作用
- 创建、编译、运行第一个 Rust 项目
- 掌握 Cargo 的核心命令和项目结构
- 配置编辑器的 Rust 开发支持

## 安装 Rust

Rust 的安装通过 `rustup`——一个**工具链管理器**，类似 Node.js 的 nvm 或 Python 的 pyenv，但更强大。

### Linux / macOS

打开终端，运行：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装过程会询问几个选项，默认选择即可（直接按 Enter）。安装完成后，**重启终端**或运行：

```bash
source "$HOME/.cargo/env"
```

### Windows

1. 访问 [rustup.rs](https://rustup.rs) 下载 `rustup-init.exe`
2. 运行安装器，默认选项即可
3. 如果提示需要 Visual Studio C++ Build Tools，按提示安装

### 验证安装

```bash
rustc --version
# 输出示例：rustc 1.85.0 (4d91de4c4 2025-02-20)

cargo --version
# 输出示例：cargo 1.85.0
```

> ⚠️ 本书基于 **Rust 2024 Edition**，请确保版本 ≥ 1.85.0。版本过低？运行 `rustup update`。

### 更新 Rust

```bash
rustup update                    # 更新所有工具链
rustup update stable             # 只更新 stable
rustup self update               # 更新 rustup 自身
```

Rust 每 6 周发布一个新版本（stable channel）。保持更新是推荐做法，因为编译器本身也在不断改进。

## rustup 详解

rustup 不只是安装器——它是你管理 Rust 生态的入口：

```bash
# 安装额外组件
rustup component add rustfmt   # 代码格式化
rustup component add clippy    # 代码检查（linter）
rustup component add rust-src  # 标准库源码（rust-analyzer 需要）

# 安装其他工具链
rustup install nightly         # 每夜构建版（最新特性）
rustup install beta            # 即将发布的 beta 版

# 查看信息
rustup show                    # 当前工具链信息
rustup toolchain list          # 已安装的工具链列表
rustup default stable          # 设置默认工具链

# 交叉编译目标
rustup target add wasm32-unknown-unknown  # 编译到 WebAssembly
rustup target add x86_64-unknown-linux-musl  # 静态链接的 Linux 二进制

# 离线文档（非常有用！）
rustup doc --book              # Rust Book
rustup doc --std               # 标准库 API 文档
rustup doc --reference         # 语言参考
```

> 💡 `rustup doc --std` 会打开标准库的离线文档。这是你编程时最常用的参考资料——在浏览器里搜索任何类型或函数，秒开。

## Hello, World!

### 创建第一个项目

```bash
cargo new hello_rust
cd hello_rust
```

Cargo 为你生成了一个完整的项目结构：

```
hello_rust/
├── Cargo.toml          # 项目元数据（配置文件）
├── .gitignore          # 自动生成，忽略 target/ 目录
└── src/
    └── main.rs         # 程序入口：fn main()
```

### Cargo.toml 详解

打开 `Cargo.toml`：

```toml
[package]
name = "hello_rust"          # 项目名称（用于发布到 crates.io）
version = "0.1.0"            # 语义化版本
edition = "2024"             # 使用 Rust 2024 Edition 语法

[dependencies]
# 第三方依赖写在这里
# serde = "1"
```

> 📘 `edition = "2024"` 是本书的要求。新项目默认也是 2024（从 Rust 1.85.0 开始），无需手动修改。

每个字段的含义：

| 字段 | 说明 |
|------|------|
| `name` | 项目名称。编译后的二进制文件会叫这个名字 |
| `version` | 语义版本号（SemVer）。发布库时必须管理好 |
| `edition` | Rust 语法版次。**2024 是目前最新的** |
| `[dependencies]` | 项目依赖。添加库时在这一栏写 |

### main.rs

```rust
fn main() {
    println!("Hello, Rust 2024 Edition!");
}
```

逐行解释：

| 代码 | 解释 |
|------|------|
| `fn main() {` | `fn` 声明函数。`main` 是程序的**入口函数**——程序从这里开始执行 |
| `println!("...")` | 自带 `!` 说明它是**宏**——不是普通函数，它在编译期展开为更复杂的代码 |
| `;` | 语句结束符。Rust 的分号表示"这是一个语句，丢弃返回值" |

### 运行

```bash
cargo run
```

输出：
```
   Compiling hello_rust v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
     Running `target/debug/hello_rust`
Hello, Rust 2024 Edition!
```

发生了什么？
1. `Compiling` —— rustc 编译了你的代码
2. `Finished` —— 编译完成，生成了二进制文件
3. `Running` —— 执行了 `target/debug/hello_rust`

如果你只想编译不运行：

```bash
cargo build                     # 编译（debug 模式，未优化）
cargo build --release           # 编译（release 模式，优化）
ls target/debug/hello_rust      # 编译产物在这里
```

## Cargo 命令详解

Cargo 是你使用 Rust 时最频繁接触的工具：

| 命令 | 做什么 | 什么时候用 |
|------|--------|-----------|
| `cargo new <name>` | 创建新项目 | 开始一个新项目 |
| `cargo init` | 在当前目录初始化项目 | 已有目录想变成 Rust 项目 |
| `cargo build` | 编译（debug） | 检查代码、生成可执行文件 |
| `cargo build --release` | 编译（release，优化） | 发布/性能测试 |
| `cargo run` | 编译 + 运行 | 日常开发最常用 |
| `cargo check` | 仅检查语法和类型（不生成二进制） | 快速验证想法，比 build 快得多 |
| `cargo test` | 运行测试 | 开发流程 |
| `cargo fmt` | 格式化代码 | 保持代码风格一致 |
| `cargo clippy` | 运行 linter | 发现不地道的代码写法 |
| `cargo doc --open` | 生成文档并打开浏览器 | 查看 API 文档 |
| `cargo clean` | 删除 target/ 目录 | 释放磁盘空间 |
| `cargo update` | 更新依赖版本 | 维护项目 |

> 💡 **技巧：** 开发时用 `cargo check` 代替 `cargo build`。它只做类型检查，不生成机器码，速度快 2-5 倍。你可以在编辑器保存文件后自动运行它来获得即时反馈。

### Debug 和 Release 模式的区别

```bash
cargo build                # Debug：编译快、运行慢、有调试信息
cargo build --release      # Release：编译慢、运行快、优化全开
```

| | Debug | Release |
|---|---|---|
| 编译速度 | 快 | 慢（可能 5-10x） |
| 运行速度 | 慢 | 快（可能 10-100x） |
| 文件大小 | 大（包含调试信息） | 小（strip 后） |
| 优化级别 | 无 | `-C opt-level=3` |
| 调试 | ✅ 可以 | ❌ 困难 |

日常开发用 debug，发布用 release。这是 Rust 惯例。

## 编辑器配置

一个好的 Rust 开发环境 = 编辑器 + **rust-analyzer**。

### VS Code（推荐）

1. 安装 [VS Code](https://code.visualstudio.com/)
2. 安装 `rust-analyzer` 插件（Rust 官方维护）
3. 安装 `Even Better TOML`（Cargo.toml 语法高亮）
4. 安装 `crates`（显示依赖最新版本）

安装后打开任意 `.rs` 文件，你会获得：

| 功能 | 快捷键（VS Code） | 说明 |
|------|------------------|------|
| 类型提示 | 鼠标悬停 | 显示变量/函数的完整类型 |
| 代码补全 | Ctrl+Space | 智能感知 |
| 跳转到定义 | F12 | 跳到函数/类型的定义处 |
| 查找引用 | Shift+F12 | 谁在使用这个符号 |
| 内联错误 | 自动显示 | 编译错误实时标注 |
| 重命名 | F2 | 安全地重命名符号 |
| 代码格式化 | Shift+Alt+F | 自动格式化（调用 rustfmt） |
| 展开宏 | Ctrl+Shift+P → "Expand Macro" | 看宏展开后的代码 |

### 其他编辑器

| 编辑器 | 必备插件/配置 |
|--------|-------------|
| **IntelliJ / CLion** | JetBrains 官方 `Rust` 插件（自带 rust-analyzer） |
| **Vim / Neovim** | `rust-analyzer` LSP + `coc-rust-analyzer` 或 `nvim-lspconfig` |
| **Emacs** | `rustic` 或 `rust-mode` + `lsp-mode` + `eglot` |
| **Helix** | 内置 LSP 支持，开箱即用 |

### 确认 rust-analyzer 正常

打开 `src/main.rs`，在 `fn main() {` 的下一行输入 `let x = 5;`。你应该看到：
- `x` 后面出现灰字提示 `: i32`（类型推断）
- 鼠标悬停在 `println!` 上能看到宏的文档

如果没有这些效果，检查：
1. `rust-analyzer` 插件是否安装和启用
2. `rustup component add rust-src` 是否运行过
3. 项目根目录是否有 `Cargo.toml`

## Cargo 项目结构深入

```
hello_rust/
├── Cargo.toml              # 📄 项目配置文件（人写的）
├── Cargo.lock              # 📄 依赖版本锁定文件（机器生成的，不要手动编辑）
├── .gitignore              # 📄 自动生成，忽略 target/
├── src/
│   └── main.rs             # 📄 程序入口（二进制 crate）
├── tests/                  # 📁 集成测试（可选）
│   └── integration_test.rs
├── examples/               # 📁 示例代码（可选）
│   └── example_usage.rs
├── benches/                # 📁 基准测试（可选）
│   └── benchmark.rs
└── target/                 # 📁 编译输出（不要提交到 Git）
    ├── debug/              # debug 模式产物
    │   └── hello_rust      # 可执行文件
    └── release/            # release 模式产物
```

> ⚠️ `target/` 目录可以很大（几百 MB 到几 GB）。用 `cargo clean` 清理。它已经被 `.gitignore` 排除，不要手动提交。

## 常见问题排查

### 安装失败

```bash
# 检查网络连接
curl -I https://sh.rustup.rs

# 国内网络慢？尝试中科大镜像
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### cargo build 报错 "linker 'cc' not found"

Linux 需要安装 C 编译器：
```bash
# Ubuntu/Debian
sudo apt install build-essential

# Fedora/CentOS
sudo dnf install gcc

# Arch
sudo pacman -S base-devel
```

macOS 需要 Xcode Command Line Tools：
```bash
xcode-select --install
```

### Windows 报错 "link.exe not found"

安装 [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)，选择 "C++ build tools" 工作负载。

## 练习

1. 用 `cargo new my_first_project` 创建项目，修改 `main.rs` 打印自己的名字和当前时间（提示：`std::time::SystemTime::now()` 可能暂时有点难，先打印名字即可）
2. 试试以下命令，观察输出差异：
   - `cargo check`
   - `cargo build`
   - `cargo build --release`
   - 比较 `target/debug/` 和 `target/release/` 下编译产物的大小
3. 在 VS Code 中打开项目，测试 rust-analyzer 功能：鼠标悬停看类型、Ctrl+Click 跳转
4. 用 `cargo doc --open` 打开标准库文档，搜索 `String` 类型，浏览它的方法列表

---

← [前言](./00-preface.md) | [返回目录](./README.md) | → [第 2 章：基础语法](./02-basics.md)
