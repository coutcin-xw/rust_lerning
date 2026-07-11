# 第 21 章：测试

## 学习目标

- 编写单元测试、集成测试、文档测试
- 掌握断言宏和 `should_panic`
- 理解 `cargo test` 的运行机制（并行、过滤、输出控制）
- 使用测试夹具（fixtures）和辅助函数组织测试
- 了解属性测试和 mock 的基本思路

## 概念层：Rust 的测试哲学

### 测试和源码在一起

Rust 不强制你把测试放在单独的目录中。单元测试直接写在源码文件里，用 `#[cfg(test)]` 条件编译：

```rust
// src/lib.rs — 源码和测试在同一个文件
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]  // 这个模块只在 cargo test 时编译
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

这种做法有好处：
- 测试紧邻源码，开发者不会"忘记写测试"
- 测试可以访问私有函数（`use super::*` 导入父模块所有内容）
- 阅读测试就是阅读该模块的用法示例

### 测试在 Rust 中的角色

Rust 编译器已经消灭了空指针、数据竞争、悬垂引用等 bug。但它无法验证**业务逻辑**——"这个函数在给定输入时返回期望的输出吗？" 这就是测试的职责。

## 机制层：cargo test 怎么工作

### 编译机制

`cargo test` 实际上做了两件事：

1. 用 `--test` flag 编译你的代码，这会**设置 `cfg(test)` 为 true**——所有 `#[cfg(test)]` 模块被包含进来
2. 生成一个**测试 harness 可执行文件**，内部调用所有 `#[test]` 函数

每个 `#[test]` 函数在**独立的线程**中运行。如果某个测试 panic 了（断言失败 = panic），harness 捕获 panic 并报告该测试失败，然后继续运行其他测试。

### 并行执行

```bash
cargo test -- --test-threads=1   # 单线程顺序执行
cargo test -- --test-threads=4   # 4 个测试并行
# 默认：CPU 核心数
```

> ⚠️ 因为测试并行运行，**测试之间不能共享可变状态**。一个测试修改全局状态会影响其他测试。如果需要串行，用 `--test-threads=1`。

### 输出控制

```bash
cargo test                          # 默认：通过的不显示输出，失败才显示
cargo test -- --nocapture           # 显示所有 println! 输出
cargo test -- --show-output         # 同上（1.64+）
cargo test -- --format=json         # JSON 格式输出（CI 用）
```

## 代码层：断言和测试写法

### 断言宏

```rust
#[test]
fn test_assertions() {
    // 基本断言
    assert!(1 + 1 == 2);
    assert_eq!(2 + 2, 4);
    assert_ne!(2 + 2, 5);

    // 带自定义消息（消息只在断言失败时求值）
    let name = "Alice";
    assert!(name.len() > 0, "名字不能为空，但得到 '{}'", name);

    // 比较浮点数（不精确相等）
    let result: f64 = 0.1 + 0.2;
    assert!((result - 0.3).abs() < f64::EPSILON);
    // 或者
    assert!((result - 0.3).abs() < 1e-10, "浮点误差过大: {}", result);
}

#[test]
fn test_result() -> Result<(), String> {
    // 测试函数可以返回 Result——失败时传播 Err
    let value = some_fallible_op()?;
    assert_eq!(value, 42);
    Ok(())
}

#[test]
#[should_panic(expected = "除数不能为零")]
fn test_divide_by_zero() {
    divide(1, 0);  // panic 被捕获，检查消息包含 "除数不能为零"
}

#[test]
#[ignore = "耗时太长，手动运行时才执行"]
fn expensive_test() {
    // 用 cargo test -- --ignored 专门跑
}
```

### 测试夹具（Fixtures）—— 复用初始化和清理

```rust
// 简单的 helper 函数
fn setup_test_user() -> User {
    User {
        id: 1,
        name: String::from("test_user"),
        email: String::from("test@example.com"),
    }
}

#[test]
fn test_user_display() {
    let user = setup_test_user();
    assert_eq!(format!("{}", user), "test_user <test@example.com>");
}

#[test]
fn test_user_email_validation() {
    let mut user = setup_test_user();
    user.email = String::from("invalid");
    assert!(!user.has_valid_email());
}

// 需要清理的夹具（如临时文件）
fn with_temp_dir<F: FnOnce(&std::path::Path)>(test: F) {
    let dir = std::env::temp_dir().join(format!("test-{}", uuid()));
    std::fs::create_dir_all(&dir).unwrap();
    test(&dir);
    std::fs::remove_dir_all(&dir).unwrap();  // 清理
}

#[test]
fn test_file_operations() {
    with_temp_dir(|dir| {
        let file_path = dir.join("test.txt");
        std::fs::write(&file_path, "hello").unwrap();
        assert_eq!(std::fs::read_to_string(&file_path).unwrap(), "hello");
    });
    // 离开 with_temp_dir 时文件自动被删除
}
```

### 测试私有函数

```rust
// lib.rs
fn internal_helper(x: i32) -> i32 { x * 2 + 1 }  // 私有函数

#[cfg(test)]
mod tests {
    use super::*;  // 可以访问 internal_helper

    #[test]
    fn test_internal() {
        assert_eq!(internal_helper(3), 7);
    }
}
```

## 集成测试

集成测试放在 `tests/` 目录，每个文件是独立的 crate，只能访问公开 API：

```
my_project/
└── tests/
    ├── common/
    │   └── mod.rs           # 辅助模块（不会被 cargo test 当作测试文件）
    ├── api_test.rs           # 独立的测试二进制
    └── db_test.rs
```

```rust
// tests/api_test.rs
use my_project;            // 像外部用户一样导入
mod common;                // 导入辅助模块

#[test]
fn test_create_and_get_user() {
    common::init_logger();  // 共享的测试初始化

    let user = my_project::create_user("alice", "alice@example.com").unwrap();
    assert_eq!(user.name, "alice");

    let fetched = my_project::get_user(user.id).unwrap();
    assert_eq!(fetched, user);
}
```

## 实践层：测试模式与策略

### AAA 模式（Arrange-Act-Assert）

```rust
#[test]
fn test_transfer_money() {
    // Arrange —— 准备
    let mut alice = Account::new(100);
    let mut bob = Account::new(0);

    // Act —— 执行
    let result = alice.transfer(30, &mut bob);

    // Assert —— 验证
    assert!(result.is_ok());
    assert_eq!(alice.balance(), 70);
    assert_eq!(bob.balance(), 30);
}
```

### 测试组织：每个模块对应一个测试模块

```rust
// src/config.rs
pub struct Config { /* ... */ }
impl Config { pub fn load(path: &str) -> Result<Self> { /* ... */ } }

#[cfg(test)]
mod tests {
    use super::*;
    // 所有 config 相关的测试放在这里
}

// src/service.rs
pub struct Service { /* ... */ }

#[cfg(test)]
mod tests {
    use super::*;
    // 所有 service 相关的测试放在这里
}
```

### 属性测试（Property-based Testing）简介

传统测试是"写输入，验证输出"。属性测试是"描述属性，随机生成输入验证"。

```rust
// 需要 proptest crate: cargo add --dev proptest
#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn doubling_doesnt_change_sign(x: i32) {
            let doubled = x * 2;
            // 属性：翻倍不会改变符号
            prop_assert_eq!(doubled >= 0, x >= 0);
        }

        #[test]
        fn roundtrip_json(v in any::<Vec<String>>()) {
            let json = serde_json::to_string(&v).unwrap();
            let back: Vec<String> = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(v, back);
        }
    }
}
```

### Mock 的基本思路

Rust 没有反射，传统 mock 框架受限制。常用替代方案：

| 方式 | 适用场景 | 示例 |
|------|---------|------|
| **Trait 对象替换** | 接口明确 | 定义 `trait Database`，测试用 `FakeDb`，生产用 `RealDb` |
| **条件编译替换** | 全局依赖 | `#[cfg(test)]` 使用 fake 实现 |
| **mockall crate** | 需要精细化控制调用 | `#[automock]` 自动生成 mock |
| **直接传假数据** | 简单场景 | 传一个假的 `&Path` 或 `&str` |
| **集成测试** | 真实外部服务 | 启动测试数据库、HTTP mock server |

```rust
// Trait 对象替换 —— Rust 中最常见的 mock 方式
trait EmailSender {
    fn send(&self, to: &str, body: &str) -> Result<(), EmailError>;
}

struct RealSender;
impl EmailSender for RealSender { /* 真实发邮件 */ }

struct FakeSender {
    sent: Mutex<Vec<(String, String)>>,
}
impl EmailSender for FakeSender {
    fn send(&self, to: &str, body: &str) -> Result<(), EmailError> {
        self.sent.lock().unwrap().push((to.to_string(), body.to_string()));
        Ok(())
    }
}

// 测试中用 FakeSender
#[test]
fn test_welcome_email() {
    let fake = FakeSender { sent: Mutex::new(vec![]) };
    let service = UserService::new(&fake);
    service.send_welcome("alice@example.com").unwrap();
    assert_eq!(fake.sent.lock().unwrap().len(), 1);
}
```

## 命令层：cargo test 完整用法

```bash
cargo test                          # 运行所有测试（单元+集成+文档）
cargo test --lib                    # 只运行库的单元测试
cargo test --bin my_app             # 只运行特定二进制的测试
cargo test --test api_test          # 只运行特定集成测试文件
cargo test --doc                    # 只运行文档测试
cargo test test_add                 # 运行名称包含 "test_add" 的测试
cargo test tests::service::         # 运行特定模块的测试
cargo test -- --test-threads=1      # 单线程（需要顺序时）
cargo test -- --nocapture           # 显示 println! 输出
cargo test -- --ignored             # 只运行 #[ignore] 的测试
cargo test -- --include-ignored     # 包含 #[ignore] 的测试
cargo test -- --format=json         # JSON 格式（CI 用）
cargo test --features "full"        # 启用特定 feature 时测试
cargo test --all-features           # 全部 feature 启用
cargo test --no-default-features    # 不启用默认 feature

# 测试覆盖率（需要安装 cargo-tarpaulin）
cargo tarpaulin --out Html
```

## 示例层：为一个模块写完整测试套件

假设你在写一个 `Config` 解析器：

```rust
// src/config.rs
#[derive(Debug, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    IoError(String),
    ParseError(String),
    MissingField(&'static str),
}

impl Config {
    /// 从文件加载配置。
    /// 文件格式：每行 `key = value`，空行和 # 注释行忽略。
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::IoError(e.to_string()))?;
        Self::parse(&content)
    }

    fn parse(input: &str) -> Result<Self, ConfigError> {
        let mut host = None;
        let mut port = None;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (key, value) = line.split_once('=')
                .ok_or(ConfigError::ParseError(format!("无效行: {}", line)))?;
            match key.trim() {
                "host" => host = Some(value.trim().to_string()),
                "port" => port = Some(value.trim().parse()
                    .map_err(|e| ConfigError::ParseError(e.to_string()))?),
                _ => {}
            }
        }

        Ok(Config {
            host: host.ok_or(ConfigError::MissingField("host"))?,
            port: port.ok_or(ConfigError::MissingField("port"))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------- 测试夹具 ----------
    fn make_temp_config(content: &str) -> (tempfile::TempDir, String) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.txt");
        std::fs::write(&path, content).unwrap();
        (dir, path.to_str().unwrap().to_string())
    }
    // (TempDir 离开作用域时自动删除目录)

    // ---------- 单元测试：parse 函数 ----------
    #[test]
    fn parse_basic() {
        let config = Config::parse("host = localhost\nport = 8080\n").unwrap();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn parse_ignores_comments_and_blanks() {
        let input = "\n# 数据库配置\nhost = db.example.com\n\nport = 5432\n";
        let config = Config::parse(input).unwrap();
        assert_eq!(config.host, "db.example.com");
    }

    #[test]
    fn parse_missing_field_returns_error() {
        let err = Config::parse("host = localhost\n").unwrap_err();
        assert!(matches!(err, ConfigError::MissingField("port")));
    }

    #[test]
    fn parse_invalid_port_returns_error() {
        let err = Config::parse("host = localhost\nport = not_a_number\n").unwrap_err();
        assert!(matches!(err, ConfigError::ParseError(_)));
    }

    // ---------- 集成风格的测试：from_file ----------
    #[test]
    fn from_file_loads_config() {
        let (_dir, path) = make_temp_config("host = testserver\nport = 3000\n");
        let config = Config::from_file(&path).unwrap();
        assert_eq!(config.host, "testserver");
        assert_eq!(config.port, 3000);
        // _dir 在此离开作用域 → 临时文件自动删除
    }

    #[test]
    fn from_file_not_found() {
        let result = Config::from_file("/nonexistent/path.txt");
        assert!(matches!(result, Err(ConfigError::IoError(_))));
    }
}
```

这个示例展示了：
- 私有函数的单元测试（`parse` 是私有的，通过 `use super::*` 访问）
- 公开 API 的集成风格测试（`from_file`）
- 自定义错误的 `matches!` 断言
- 临时文件的自动清理机制
- AAA 模式

### rstest — 基于 fixture 的参数化测试

`rstest` 是 Rust 社区最流行的测试增强库之一，提供了 fixture（测试夹具）和参数化测试的支持。相比手写 helper 函数，它减少了样板代码；相比 proptest 的随机输入，它适合"多组已知输入输出验证同一逻辑"的场景。

首先在 `Cargo.toml` 中添加依赖：

```toml
[dev-dependencies]
rstest = "0.19"
```

用 `#[fixture]` 创建可复用的测试上下文。当测试函数参数名匹配某个 fixture 名时，rstest 会自动注入：

```rust
use rstest::rstest;

#[fixture]
fn user() -> User {
    User { id: 1, name: "alice".into(), active: true }
}

#[rstest]
fn test_user_is_active(user: User) {
    assert!(user.active);
}
```

`#[case]` 让同一个测试函数跑多组数据，每组生成独立的子测试——其中一组失败不影响其他组的定位：

```rust
#[rstest]
#[case(1, 2, 3)]
#[case(0, 0, 0)]
#[case(-1, 1, 0)]
fn test_add(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
    assert_eq!(a + b, expected);
}
```

fixture 和 case 可以组合使用，例如为每个测试用例注入同样的数据库连接夹具，打造出"参数化 + 共享上下文"的灵活测试编排。

### criterion — 基准性能测试

`criterion` 是 Rust 事实上的基准测试标准库，替代了 nightly 专属且功能有限的内置 `#[bench]`。它采用统计分析消除系统噪声，自动检测异常值，对比历史记录发现性能回归，还能生成 HTML 可视化报告。

在 `Cargo.toml` 中配置：

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false
```

在 `benches/my_benchmark.rs` 中编写基准测试：

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n { 0 => 0, 1 => 1, _ => fibonacci(n - 1) + fibonacci(n - 2) }
}

fn bench_fib(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, bench_fib);
criterion_main!(benches);
```

运行 `cargo bench` 即可执行。输出包含每次迭代的平均时间、标准差、相对上次运行的变化百分比。`black_box` 告诉编译器"这段代码的结果外部可见"，防止优化器把被测函数直接消除掉。

开启 `html_reports` feature 后，criterion 会在 `target/criterion/` 下生成详细的 HTML 报告，包括抖动分布图、性能变化趋势等，非常适合在 CI 中跟踪性能曲线的长期变化。

## 练习

1. 为一个计算器库写完整的单元测试（加减乘除 + 除零 + 溢出）
2. 在 `tests/` 目录下写集成测试，模拟外部用户的方式
3. 为关键公开函数添加文档测试，用 `cargo test --doc` 验证
4. 写一个 `#[should_panic]` 测试验证特定 panic 消息
5. 用 trait 对象替换写一个可测试的 `NotificationService`（实际发邮件/测试用虚假实现）

---

← [第 20 章：标准库集合](./20-collections.md) | [返回目录](./README.md) | → [第 22 章：unsafe Rust 与 FFI](./22-unsafe-ffi.md)
