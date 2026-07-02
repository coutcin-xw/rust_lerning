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

## 练习

1. 创建一个库项目，定义多个模块，通过 `pub use` 提供简洁的公开 API
2. 把一段单文件代码重构为多模块结构（`main.rs` + `lib.rs` + 子模块）
3. 用 `pub(crate)` 实现"内部共享但不对外暴露"的工具函数

---

← [第 14 章：异步编程](./14-async-await.md) | [返回目录](./README.md) | → [第 16 章：宏](./16-macros.md)
