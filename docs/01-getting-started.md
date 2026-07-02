# 第 1 章：环境搭建

## 学习目标

- 安装 Rust 工具链（版本 ≥ 1.85.0）
- 编写并运行第一个 Rust 程序
- 理解 Cargo 的基本用法
- 配置 VS Code 或其他编辑器的 Rust 支持

## 安装 Rust

### Linux / macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows

从 [rustup.rs](https://rustup.rs) 下载安装器运行。

### 验证安装

```bash
rustc --version
# rustc 1.85.0 (或更高版本)

cargo --version
# cargo 1.85.0 (或更高版本)
```

> ⚠️ 本书基于 **Rust 2024 Edition**，请确保 Rust 版本 ≥ 1.85.0。版本过低可以用 `rustup update` 升级。

### 更新 Rust

```bash
rustup update
```

## rustup 是什么？

`rustup` 是 Rust 的**工具链管理器**，类似 Node.js 的 nvm 或 Python 的 pyenv。它管理：

- `rustc` — Rust 编译器
- `cargo` — 包管理器和构建工具
- `rustup` — 工具链本身
- 文档、标准库源码、clippy、rustfmt 等组件

常用命令：

```bash
rustup update          # 更新工具链
rustup component add rustfmt clippy  # 安装组件
rustup doc --book      # 打开 Rust Book 本地版
```

## Hello, World!

创建第一个项目：

```bash
cargo new hello_rust
cd hello_rust
```

`Cargo.toml` 文件（注意 edition 是 2024）：

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2024"

[dependencies]
```

> 📘 `edition = "2024"` 表示使用 Rust 2024 Edition 语法。新项目应默认使用这个值。

`src/main.rs`：

```rust
fn main() {
    println!("Hello, Rust 2024 Edition!");
}
```

运行：

```bash
cargo run
# 输出：Hello, Rust 2024 Edition!
```

## Cargo 基础

Cargo 是 Rust 的**构建系统和包管理器**，你几乎每天都会用到它。

| 命令 | 功能 |
|------|------|
| `cargo new <name>` | 创建新项目 |
| `cargo build` | 编译项目（debug 模式） |
| `cargo build --release` | 编译项目（release 模式，优化） |
| `cargo run` | 编译并运行 |
| `cargo check` | 快速检查代码能否编译（不生成可执行文件） |
| `cargo test` | 运行测试 |
| `cargo fmt` | 格式化代码 |
| `cargo clippy` | 运行 linter |
| `cargo doc --open` | 生成并打开文档 |

> 💡 **技巧：** 开发时用 `cargo check` 代替 `cargo build`——它只做语法和类型检查，不生成二进制文件，速度快很多。

## 编辑器配置

推荐 **VS Code** + **rust-analyzer** 插件：

1. 安装 VS Code
2. 安装插件 `rust-analyzer`（由 Rust 官方维护）
3. 可选：安装 `Even Better TOML`（Cargo.toml 语法高亮）、`crates`（依赖版本检查）

rust-analyzer 提供：
- 实时代码补全
- 类型提示和文档悬停
- 跳转到定义、查找引用
- 内联错误提示

> 其他编辑器（IntelliJ、Vim、Emacs）也都有良好的 Rust 支持，选择你习惯的即可。

## 项目结构

Cargo 创建的标准项目结构：

```
hello_rust/
├── Cargo.toml          # 项目元数据和依赖
├── Cargo.lock          # 依赖版本锁定（自动生成，不要手动编辑）
├── src/
│   └── main.rs         # 程序入口
└── target/             # 编译输出（自动生成，可加入 .gitignore）
```

## 练习

1. 用 `cargo new` 创建一个项目，修改 `main.rs` 打印自己的名字
2. 试试 `cargo check` 和 `cargo build --release`，感受速度差异
3. 故意在代码中写一个语法错误（如少写分号），看看编译器的错误提示是怎样的

---

← [前言](./00-preface.md) | [返回目录](./README.md) | → [第 2 章：基础语法](./02-basics.md)
