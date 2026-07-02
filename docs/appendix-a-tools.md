# 附录 A：工具链

Cargo 以外的 Rust 工具链组件介绍。

## rustup

```bash
# 安装/更新
rustup update
rustup update nightly          # 安装 nightly 工具链
rustup default stable          # 设置默认工具链

# 组件管理
rustup component add rustfmt clippy
rustup component list --installed

# 目标平台
rustup target add wasm32-unknown-unknown  # WebAssembly
rustup target add x86_64-unknown-linux-musl  # 静态链接

# 文档
rustup doc --book              # 打开 Rust Book
rustup doc --std               # 打开标准库文档
```

## Cargo 进阶

```bash
# 项目分析
cargo tree                     # 依赖树
cargo tree -e features         # 显示 feature
cargo tree -i <crate>          # 谁依赖了某个 crate

# 构建控制
cargo build --timings          # 编译耗时分析
cargo build --target <target>  # 交叉编译
cargo build -Z build-std       # 编译自定义标准库（nightly）

# 发布
cargo publish                  # 发布到 crates.io
cargo package                  # 打包检查
cargo login <token>            # crates.io 认证
cargo yank --vers <ver> <crate>  # 撤回版本

# 工作空间
cargo metadata                 # JSON 格式的项目信息
```

### Cargo.toml 常用配置

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2024"
description = "A sample application"
license = "MIT"
repository = "https://github.com/user/repo"

[dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", optional = true }

[dev-dependencies]
criterion = "0.5"

[build-dependencies]
cc = "1"

[features]
default = ["std"]
std = []
async = ["tokio"]

[profile.release]
opt-level = 3        # 优化级别
lto = true           # 链接时优化
codegen-units = 1    # 更好的优化（但编译更慢）
```

## clippy

Rust 的官方 linter：

```bash
cargo clippy                    # 运行所有 lint
cargo clippy -- -W clippy::pedantic  # 启用最严格的 lint
cargo clippy --fix              # 自动修复部分问题

# 常用 lint 属性
#[allow(clippy::too_many_arguments)]
fn complex(a: i32, b: i32, c: i32, d: i32, e: i32) { }
```

### 推荐的 clippy 配置

在 `Cargo.toml` 中：
```toml
[lints.clippy]
unwrap_used = "deny"       # 禁止 unwrap
expect_used = "deny"       # 禁止 expect
todo = "warn"              # 提醒完成 TODO
```

## rustfmt

```bash
cargo fmt                      # 格式化所有代码
cargo fmt -- --check           # 检查但不修改
cargo fmt -- --edition 2024    # 使用 2024 风格版本
```

## 常用第三方工具

| 工具 | 用途 | 安装 |
|------|------|------|
| `cargo-watch` | 文件变化时自动执行命令 | `cargo install cargo-watch` |
| `cargo-edit` | 命令行管理依赖 | `cargo install cargo-edit` |
| `cargo-audit` | 检查依赖安全漏洞 | `cargo install cargo-audit` |
| `cargo-deny` | 许可证和依赖审计 | `cargo install cargo-deny` |
| `criterion` | 基准测试框架 | 添加到 `dev-dependencies` |
| `cargo-expand` | 展开宏和泛型代码 | `cargo install cargo-expand` |
| `cargo-flamegraph` | 性能火焰图 | `cargo install flamegraph` |

```bash
# cargo-watch：文件变化自动运行
cargo watch -x check -x test

# cargo-edit：添加/删除依赖
cargo add serde tokio
cargo rm unused-dep

# cargo-audit：安全审计
cargo audit

# cargo-expand：展开宏
cargo expand            # 展开 main.rs

# cargo-flamegraph：性能分析
cargo flamegraph --bin my-app
```

## IDE 和编辑器支持

| 编辑器 | 必备插件 |
|--------|---------|
| VS Code | `rust-analyzer`, `Even Better TOML`, `crates` |
| IntelliJ/CLion | `Rust` 插件（JetBrains 官方） |
| Vim/Neovim | `rust-analyzer` (LSP), `coc-rust-analyzer` 或 `nvim-lspconfig` |
| Emacs | `rustic` 或 `rust-mode` + `lsp-mode` |

---

← [第 19 章：unsafe Rust 与 FFI](./19-unsafe-ffi.md) | [返回目录](./README.md) | → [附录 B：学习资源](./appendix-b-resources.md)
