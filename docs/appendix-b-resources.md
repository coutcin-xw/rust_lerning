# 附录 B：学习资源

## 官方资源（必读）

| 资源 | 说明 |
|------|------|
| [The Rust Book](https://doc.rust-lang.org/book/) | 官方入门教程。**如果你只读一个外部资源，就读这个。** |
| [Rust by Example](https://doc.rust-lang.org/rust-by-example/) | 通过可运行的示例学 Rust |
| [Rust Reference](https://doc.rust-lang.org/reference/) | 语言参考手册——精确到每个语法细节 |
| [Edition Guide](https://doc.rust-lang.org/edition-guide/) | Edition 间迁移指南 |
| [The Cargo Book](https://doc.rust-lang.org/cargo/) | Cargo 完全指南 |
| [Rustlings](https://github.com/rust-lang/rustlings) | 交互式小练习，clone 下来在终端里做 |
| [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/) | 常用场景代码片段：文件操作、网络、日期时间等 |
| [std API 文档](https://doc.rust-lang.org/std/) | 标准库的完整 API 参考 |
| [The Rustonomicon](https://doc.rust-lang.org/nomicon/) | 深入 unsafe Rust 的黑暗魔法 |
| [The Unstable Book](https://doc.rust-lang.org/nightly/unstable-book/) | Nightly 特性的文档 |

## 系统学习推荐

### 书籍

| 书名 | 作者 | 适用阶段 |
|------|------|---------|
| **Programming Rust** (2nd ed.) | Blandy, Orendorff, Tindall | 初学者到中级 |
| **Rust for Rustaceans** | Jon Gjengset | 中级到高级 |
| **Zero to Production in Rust** | Luca Palmieri | 用 Rust 构建 Web 服务（实战） |
| **Rust Atomics and Locks** | Mara Bos | 并发编程深入 |
| **Effective Rust** | David Drysdale | Rust 惯用法和最佳实践 |
| **Command-Line Rust** | Ken Youens-Clark | CLI 工具实战 |
| **Rust Brain Teasers** | Herbert Wolverson | 加深理解的有趣题目 |

### 视频课程

| 课程 | 讲师 | 平台 |
|------|------|------|
| Rust Programming | Jon Gjengset | YouTube (Crust of Rust 系列) |
| Rust Development | Brooks Builds | YouTube |
| Ultimate Rust Crash Course | Nathan Stocks | Udemy (免费) |

## 练习平台

| 平台 | 说明 |
|------|------|
| [Rustlings](https://github.com/rust-lang/rustlings) | 官方练习题——最好的 Rust 上手练习 |
| [Exercism Rust Track](https://exercism.org/tracks/rust) | 由简到难的编程题，有 mentor review |
| [Rust Quiz](https://dtolnay.github.io/rust-quiz/) | 测试你对 Rust 微妙行为的理解 |
| [Advent of Code](https://adventofcode.com/) | 每年 12 月 25 道题——用 Rust 解很合适 |
| [CodeCrafters](https://codecrafters.io/) | 从零构建 Git、Redis、Docker 等 |

## 社区和保持更新

| 渠道 | 链接 |
|------|------|
| **官方论坛** | [users.rust-lang.org](https://users.rust-lang.org/) |
| **Reddit** | [r/rust](https://reddit.com/r/rust) |
| **Discord** | [discord.gg/rust-lang](https://discord.gg/rust-lang) |
| **This Week in Rust** | [this-week-in-rust.org](https://this-week-in-rust.org/) — 每周新闻简报，必订 |
| **Rust Blog** | [blog.rust-lang.org](https://blog.rust-lang.org/) — 官方发布公告 |
| **Rustacean Station** | [rustacean-station.org](https://rustacean-station.org/) — 播客 |
| **Rust Internals** | [internals.rust-lang.org](https://internals.rust-lang.org/) — 语言开发讨论 |

## 常用第三方库速查

| 领域 | 推荐库 |
|------|--------|
| **Web 框架** | `axum` (推荐), `actix-web`, `warp` |
| **异步运行时** | `tokio` (事实标准) |
| **序列化** | `serde` + `serde_json`（事实标准） |
| **数据库** | `sqlx` (async), `diesel` (sync ORM), `sea-orm` |
| **CLI 参数** | `clap` (推荐), `bpaf` |
| **日志/追踪** | `tracing` (结构化日志，推荐), `log` |
| **HTTP 客户端** | `reqwest` |
| **模板引擎** | `askama` (编译期), `minijinja` |
| **错误处理** | `thiserror` (库), `anyhow` (应用) |
| **测试** | `criterion` (基准), `proptest` (属性测试), `mockall` (mock) |
| **随机数** | `rand` |
| **正则表达式** | `regex` |
| **时间日期** | `chrono` / `time` |
| **命令行 UI** | `indicatif` (进度条), `dialoguer` (交互提示), `ratatui` (TUI) |

## 学习路径推荐

```
初学者：
  1. Rust Book 前 10 章 + Rustlings 全部做完
  2. 用 Rust 写几个 CLI 工具（替代你的 bash 脚本）
  3. Advent of Code 做 25-50 题

中级：
  4. Rust for Rustaceans（第 3-6 章为核心）
  5. 用 axum + sqlx 写一个 REST API
  6. 阅读常用 crate 源码（serde, clap, reqwest）

高级：
  7. Rust Atomics and Locks
  8. 阅读 The Rustonomicon
  9. 为开源 Rust 项目贡献代码
  10. 写一个 crate 发布到 crates.io
```

---

← [附录 A：工具链](./appendix-a-tools.md) | [返回目录](./README.md) | → [附录 C：速查表](./appendix-c-cheatsheet.md)
