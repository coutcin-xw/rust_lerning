# Rust 2024 Edition 学习项目

基于 Rust 2024 Edition 的系统性学习资源，包含结构化教学文档和配套代码示例。

## 📚 教学文档

**[→ 开始学习](./docs/README.md)** ｜ 20 章 + 3 个附录 ｜ ~7,700 行 ｜ 适合移动端阅读

| 阶段 | 章节 | 主题 |
|------|------|------|
| **基础语法** | 第 1-6 章 | 环境搭建、基础语法、所有权、借用、结构体与枚举、模式匹配 |
| **核心特性** | 第 7-11 章 | 错误处理、Trait 与泛型、生命周期、闭包、迭代器 |
| **高级主题** | 第 12-17 章 | 并发编程、智能指针、异步编程、模块系统、项目工程化、宏 |
| **实战补充** | 第 18-20 章 | 标准库集合、测试、unsafe Rust 与 FFI |
| **附录** | A / B / C | 工具链、学习资源、速查表 |

每章按**概念层 → 机制层 → 代码层 → 实践层 → 示例层**组织，包含：
- 核心概念讲解与跨语言对比
- 内存布局图解和编译原理说明
- 可运行的代码示例
- 常见陷阱与最佳实践
- 4-5 个难度递进练习题

覆盖 Rust 2024 Edition 新特性：let chains、async fn in traits、RPIT 生命周期自动捕获、unsafe extern、async closures 等。

## 💻 配套代码

`src/` 目录包含 15 个带详细注释的 `.rs` 文件，按序号对应各章节：

```
src/
├── 01_basics.rs          # 基础语法
├── 02_ownership.rs       # 所有权系统
├── 03_borrowing.rs       # 借用和引用
├── 04_struct_enum.rs     # 结构体和枚举
├── 05_pattern_match.rs   # 模式匹配
├── 06_error_handling.rs  # 错误处理
├── 07_trait_generics.rs  # Trait 和泛型
├── 08_lifetime.rs        # 生命周期
├── 09_concurrency.rs     # 并发编程
├── 10_closures.rs        # 闭包
├── 11_iterators.rs       # 迭代器
├── 12_smart_pointers.rs  # 智能指针
├── 13_async_await.rs     # 异步编程
├── 14_modules.rs         # 模块系统
└── 15_macros.rs          # 宏
```

## 🚀 运行

```bash
# 运行所有代码示例
cargo run

# 快速检查编译
cargo check

# 运行测试
cargo test
```

## 📖 学习建议

1. **通读文档**：从 [docs/README.md](./docs/README.md) 开始，按章节顺序阅读
2. **手敲代码**：每学完一节，在 `src/` 对应文件中找到代码示例，亲手运行和修改
3. **做练习题**：每章末尾的练习从易到难，逐步建立信心
4. **拥抱编译器**：Rust 的编译错误是业界最友好的——它们是教学材料，不是惩罚

## 🔗 参考资源

- [The Rust Book](https://doc.rust-lang.org/book/) — 官方入门教程
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — 代码示例
- [Rust 标准库文档](https://doc.rust-lang.org/std/) — API 参考
- [Rustlings](https://github.com/rust-lang/rustlings) — 交互式练习题
