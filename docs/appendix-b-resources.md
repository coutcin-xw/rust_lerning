# 附录 B：学习资源

## 官方资源

| 资源 | 说明 | 链接 |
|------|------|------|
| **The Rust Book** | Rust 官方入门教程，必读 | [doc.rust-lang.org/book](https://doc.rust-lang.org/book/) |
| **Rust by Example** | 通过示例学习 Rust | [doc.rust-lang.org/rust-by-example](https://doc.rust-lang.org/rust-by-example/) |
| **Edition Guide** | Edition 迁移指南 | [doc.rust-lang.org/edition-guide](https://doc.rust-lang.org/edition-guide/) |
| **Standard Library Docs** | 标准库 API 文档 | [doc.rust-lang.org/std](https://doc.rust-lang.org/std/) |
| **Rust Reference** | 语言参考手册 | [doc.rust-lang.org/reference](https://doc.rust-lang.org/reference/) |
| **The Cargo Book** | Cargo 使用指南 | [doc.rust-lang.org/cargo](https://doc.rust-lang.org/cargo/) |
| **Rustlings** | 交互式练习题 | [github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings) |
| **Rust Cookbook** | 常用场景代码片段 | [rust-lang-nursery.github.io/rust-cookbook](https://rust-lang-nursery.github.io/rust-cookbook/) |

## 进阶阅读

| 资源 | 主题 |
|------|------|
| **Rust Atomics and Locks** (Mara Bos) | 并发编程深入 |
| **Programming Rust** (Blandy, Orendorff) | 全面深入的 Rust 编程 |
| **Rust for Rustaceans** (Jon Gjengset) | 中级到高级的最佳实践 |
| **Zero To Production** (Luca Palmieri) | 用 Rust 构建生产级 Web 服务 |
| **Effective Rust** | Rust 惯用法（类似 Effective C++） |

## 社区资源

| 资源 | 说明 |
|------|------|
| [users.rust-lang.org](https://users.rust-lang.org/) | 官方用户论坛 |
| [r/rust](https://reddit.com/r/rust) | Reddit Rust 社区 |
| [This Week in Rust](https://this-week-in-rust.org/) | Rust 每周新闻简报 |
| [Rustacean Station](https://rustacean-station.org/) | Rust 播客 |
| [Rust Discord](https://discord.gg/rust-lang) | 即时聊天社区 |

## 常用第三方库

| 类别 | 推荐库 |
|------|--------|
| **Web 框架** | `axum`, `actix-web`, `warp` |
| **异步运行时** | `tokio`, `async-std` |
| **序列化** | `serde`（JSON, YAML, TOML...） |
| **数据库** | `sqlx`, `diesel`, `sea-orm` |
| **CLI** | `clap`, `indicatif`, `tracing` |
| **网络** | `reqwest`, `hyper`, `tonic`（gRPC） |
| **模板** | `askama`, `tera`, `minijinja` |
| **测试** | `criterion`（基准测试）, `proptest`（属性测试） |
| **GUI** | `egui`, `tauri`, `iced` |
| **嵌入式** | `embedded-hal`, ` embassy` |
| **WASM** | `wasm-bindgen`, `yew`, `leptos` |

## 建议的学习路径

1. **入门**：Rust Book + Rustlings → 理解所有权和借用
2. **练习**：写小型 CLI 工具、数据结构实现
3. **进阶**：Rust for Rustaceans + 选择领域深入（Web / 系统 / 嵌入式）
4. **实战**：参与开源项目、阅读优秀 crate 源码（如 `serde`, `tokio`, `clap`）
5. **保持更新**：订阅 This Week in Rust、关注官方博客

---

← [附录 A：工具链](./appendix-a-tools.md) | [返回目录](./README.md) | → [附录 C：速查表](./appendix-c-cheatsheet.md)
