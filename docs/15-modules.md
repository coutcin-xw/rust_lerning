# 第 15 章：模块系统

## 学习目标

- 理解 `mod`、`pub`、`use` 的作用
- 掌握可见性控制（`pub(crate)`, `pub(super)` 等）
- 理解文件模块和目录模块的组织方式
- 掌握 `pub use` 重新导出
- 学会组织标准项目结构

## 模块基础

模块是 Rust 中组织代码的基本单元：

```rust
mod network {
    fn connect() { }        // 私有
    pub fn serve() { }      // 公开
}

mod client {
    fn connect() { }        // 与 network::connect 不冲突
}

fn main() {
    network::serve();       // 只能访问公开的
    // network::connect();  // 编译错误！私有函数
}
```

> 💡 Rust 默认一切**私有**（除了 `pub` 标记的）。这与 Java/C++ 不同。

## 可见性

| 修饰符 | 可见范围 |
|--------|---------|
| （默认，无修饰） | 当前模块及子模块 |
| `pub` | 任何地方 |
| `pub(crate)` | 当前 crate 内 |
| `pub(super)` | 父模块 |
| `pub(in path)` | 指定路径内 |

```rust
pub mod outer {
    pub fn public_fn() { }             // 外部可见
    pub(crate) fn crate_visible() { }  // 本 crate 可见
    pub(super) fn parent_visible() { } // 父模块可见
    fn private_fn() { }                // 仅此模块可见
}
```

### 结构体的可见性

结构体的字段有独立的可见性：

```rust
pub struct User {
    pub username: String,   // 公开字段
    email: String,          // 私有字段
    password_hash: String,  // 私有字段
}

impl User {
    // 提供公共构造函数
    pub fn new(username: String, email: String, password: &str) -> Self {
        User {
            username,
            email,
            password_hash: hash(password),
        }
    }
}
```

## use 的使用

引入路径到当前作用域：

```rust
use std::collections::HashMap;
use std::io::{self, Read, Write};  // 批量导入
use std::fmt::*;                    // 通配符导入（谨慎使用）
use std::fs::File as FsFile;        // as 别名

// 现在可以直接使用
let map = HashMap::new();
let mut buf = String::new();
io::stdin().read_to_string(&mut buf);
```

导入惯例：
- 函数：导入父模块，用 `module::function()` 调用
- 类型/枚举/trait：直接导入
- 多个同名：用 `as` 别名

## 路径

```rust
fn serve_order() { }

mod back_of_house {
    fn fix_order() {
        super::serve_order();      // 访问父模块
        crate::serve_order();      // 从 crate 根开始
    }
}
```

## 文件模块组织

### 方式一：单独的 .rs 文件

```
src/
├── main.rs
├── network.rs          # mod network;
└── network/
    └── server.rs       # mod server; (network/server.rs)
```

`main.rs` 中：
```rust
mod network;  // 自动查找 network.rs 或 network/mod.rs
```

`network.rs` 中：
```rust
pub mod server;  // 自动查找 network/server.rs
pub fn connect() { }
```

### 方式二：mod.rs（传统方式）

```
src/
├── main.rs
└── network/
    ├── mod.rs         # 模块入口
    └── server.rs
```

两种方式都有效，新项目推荐方式一。

## 重新导出（pub use）

隐藏内部结构，提供干净的公开 API：

```rust
// 内部结构
mod db {
    pub mod postgres { pub struct Connection; }
    pub mod mysql { pub struct Connection; }
}

// 重新导出，隐藏内部模块层次
pub use db::postgres::Connection;
```

使用你库的用户只需：
```rust
use mylib::Connection;  // 干净！
```

## 模块设计模式

**类型 + impl + trait 分离**——结构体定义在一个模块，方法实现在另一个，trait 在第三个：

```rust
// 类型定义
pub mod types {
    pub struct User { /* ... */ }
}

// 核心逻辑
pub mod core {
    use super::types::User;
    impl User {
        pub fn process(&self) { /* ... */ }
    }
}

// 序列化（可选特性）
#[cfg(feature = "serde")]
pub mod serialization {
    // serde 相关的 impl
}
```

## 项目结构推荐

### 库项目

```
my_lib/
├── Cargo.toml
└── src/
    ├── lib.rs          # 库根，声明模块 + pub use
    ├── types.rs        # 公开类型
    ├── error.rs        # 错误类型
    ├── core/           # 核心逻辑
    │   ├── mod.rs
    │   └── engine.rs
    └── utils.rs        # 工具函数
```

### 二进制项目

```
my_app/
├── Cargo.toml
└── src/
    ├── main.rs         # 入口，尽量精简
    ├── lib.rs          # 核心逻辑放 lib，方便测试
    ├── cli.rs          # 命令行参数
    ├── config.rs       # 配置
    └── app/            # 应用逻辑
        ├── mod.rs
        └── service.rs
```

> 💡 最佳实践：把核心逻辑放在 `lib.rs` 中，`main.rs` 只负责解析参数和调用库。这样代码更容易测试。

## 练习

1. 创建一个库项目，定义公开的类型和 trait，通过 `pub use` 提供干净的 API
2. 将一段代码从单文件重构为多文件模块结构
3. 在不同模块中使用可见性修饰符，理解每种可见性的效果

---

← [第 14 章：异步编程](./14-async-await.md) | [返回目录](./README.md) | → [第 16 章：宏](./16-macros.md)
