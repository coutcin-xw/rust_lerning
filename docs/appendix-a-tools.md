# 附录 A：工具链

Cargo 之外的开发工具详细介绍。

## rustup —— 工具链管理器

```bash
# 安装和更新
rustup update                     # 更新所有已安装的工具链
rustup update nightly             # 更新 nightly 工具链
rustup default stable             # 设置默认工具链
rustup override set nightly       # 为当前目录设置特定工具链

# 组件管理
rustup component add rustfmt clippy rust-src
rustup component list --installed
rustup component remove rustfmt

# 目标平台（交叉编译）
rustup target add wasm32-unknown-unknown     # WebAssembly
rustup target add x86_64-unknown-linux-musl  # 静态链接的 Linux 二进制
rustup target add aarch64-linux-android       # Android ARM64
rustup target list --installed

# 离线文档
rustup doc --book              # The Rust Book
rustup doc --std               # 标准库文档
rustup doc --reference         # Rust 参考手册
rustup doc --cargo             # Cargo 手册
rustup doc --rustc             # 编译器手册
```

## Cargo 进阶

### 依赖管理

```toml
[dependencies]
# 精确版本
serde = "=1.0.200"

# 版本范围
tokio = ">=1.0, <2.0"

# Git 依赖
my_crate = { git = "https://github.com/user/repo", branch = "main" }

# 本地路径依赖
my_crate = { path = "../my_crate" }

# 可选依赖（条件编译）
redis = { version = "1", optional = true }

# 平台特定依赖
[target.'cfg(windows)'.dependencies]
winapi = "0.3"

[target.'cfg(unix)'.dependencies]
libc = "0.2"
```

### Feature 标志

> 📘 Feature flags 的详细讲解见 [第 17 章：Cargo 进阶](./17-cargo-advanced.md)。以下为速查。

```toml
[features]
default = ["std"]
std = []
async = ["tokio"]
full = ["std", "async"]
```

使用 feature：
```rust
#[cfg(feature = "async")]
async fn process() { }
```

```bash
cargo build --features "async full"
cargo build --no-default-features
```

### 构建配置

```toml
[profile.release]
opt-level = 3         # 优化级别：0（无）~ 3（激进），s/z（优化大小）
lto = true            # 链接时优化（生成更好的代码，但编译慢很多）
codegen-units = 1     # 1：更好的优化；更多：更快的编译
panic = "abort"       # panic 直接 abort（减少二进制大小）
strip = true          # 去除符号表（减小文件大小）

[profile.dev]
opt-level = 1         # 开发时也可以开启一些优化
```

### Cargo 诊断命令

```bash
cargo tree                     # 查看依赖树
cargo tree -e features         # 显示 feature 选择
cargo tree -i syn              # 反向依赖（谁依赖了 syn）
cargo tree --duplicates        # 检查重复依赖

cargo metadata                 # JSON 格式的完整项目信息
cargo audit                    # 检查已知安全漏洞（需 cargo-audit）
cargo outdated                 # 检查过时的依赖（需 cargo-outdated）
cargo deny check               # 许可证和安全审计（需 cargo-deny）
```

## clippy —— 代码检查

```bash
# 运行
cargo clippy                            # 基本 lint
cargo clippy -- -W clippy::pedantic     # 启用 pedantic（最严格）
cargo clippy -- -W clippy::all          # 所有 lint
cargo clippy --fix                      # 自动修复（部分）

# 在代码中控制 lint
#[allow(clippy::too_many_arguments)]
fn many_args(a: i32, b: i32, c: i32, d: i32, e: i32) { }

// Cargo.toml 中全局配置
[lints.clippy]
unwrap_used = "deny"       # 生产代码中禁止 unwrap
expect_used = "deny"
todo = "warn"
cast_possible_truncation = "warn"  # 潜在的截断警告
```

## rustfmt —— 代码格式化

```bash
cargo fmt                          # 格式化所有代码
cargo fmt -- --check               # 仅检查（CI 中常用）
cargo fmt -- --edition 2024        # 指定 Edition

# 配置（rustfmt.toml 或 .rustfmt.toml）
max_width = 100
tab_spaces = 4
edition = "2024"
```

## 推荐第三方工具

| 工具 | 功能 | 安装 |
|------|------|------|
| `cargo-watch` | 文件变化自动执行命令 | `cargo install cargo-watch` |
| `cargo-edit` | 命令行管理依赖 | `cargo install cargo-edit` |
| `cargo-expand` | 展开宏查看生成代码 | `cargo install cargo-expand` |
| `cargo-audit` | 检查依赖安全漏洞 | `cargo install cargo-audit` |
| `cargo-deny` | 许可证合规检查 | `cargo install cargo-deny` |
| `cargo-flamegraph` | 性能火焰图 | `cargo install flamegraph` |
| `cargo-bloat` | 分析二进制大小 | `cargo install cargo-bloat` |
| `cargo-criterion` | 基准测试 | `cargo install cargo-criterion` |

```bash
# 常用命令示例
cargo watch -x check -x test          # 文件变化 → check + test
cargo add serde tokio                 # 添加依赖
cargo rm unused-dep                   # 删除依赖
cargo expand                          # 展开当前文件的宏
cargo audit                           # 安全检查
cargo bloat --release --crates        # 哪些 crate 最占空间
```

## CI/CD 常见配置

```yaml
# .github/workflows/ci.yml
steps:
  - uses: actions/checkout@v4
  - uses: dtolnay/rust-toolchain@stable
  - run: cargo check
  - run: cargo test
  - run: cargo fmt -- --check
  - run: cargo clippy -- -D warnings
  - run: cargo audit  # 可选
```

---

← [第 22 章：unsafe Rust 与 FFI](./22-unsafe-ffi.md) | [返回目录](./README.md) | → [附录 B：学习资源](./appendix-b-resources.md)
