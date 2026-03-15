# Rust 学习项目

通过代码实践学习 Rust 核心语法和思想。

## 学习路线图

### 第一阶段：基础语法

| 序号 | 文件 | 主题 | 核心内容 | 重要程度 |
|-----|------|------|---------|---------|
| 01 | `01_basics.rs` | 基础语法 | 变量、数据类型、函数、控制流 | ⭐⭐⭐ |
| 02 | `02_ownership.rs` | 所有权系统 | 所有权规则、移动、Copy、Drop | ⭐⭐⭐⭐⭐ |
| 03 | `03_borrowing.rs` | 借用和引用 | 不可变引用、可变引用、切片、借用规则 | ⭐⭐⭐⭐⭐ |
| 04 | `04_struct_enum.rs` | 结构体和枚举 | struct定义、方法、enum、Option<T> | ⭐⭐⭐⭐ |
| 05 | `05_pattern_match.rs` | 模式匹配 | match、if let、解构、匹配守卫 | ⭐⭐⭐⭐ |

### 第二阶段：核心特性

| 序号 | 文件 | 主题 | 核心内容 | 重要程度 |
|-----|------|------|---------|---------|
| 06 | `06_error_handling.rs` | 错误处理 | Result<T,E>、?操作符、自定义错误 | ⭐⭐⭐⭐ |
| 07 | `07_trait_generics.rs` | Trait和泛型 | trait定义、实现、泛型、trait bound | ⭐⭐⭐⭐⭐ |
| 08 | `08_lifetime.rs` | 生命周期 | 生命周期标注、省略规则、'static | ⭐⭐⭐⭐⭐ |
| 09 | `09_closures.rs` | 闭包 | 闭包语法、捕获变量、Fn/FnMut/FnOnce | ⭐⭐⭐⭐ |
| 10 | `10_iterators.rs` | 迭代器 | Iterator trait、适配器、消费者 | ⭐⭐⭐⭐ |

### 第三阶段：高级特性

| 序号 | 文件 | 主题 | 核心内容 | 重要程度 |
|-----|------|------|---------|---------|
| 11 | `11_concurrency.rs` | 并发编程 | 线程、消息传递、Mutex、Arc、Send/Sync | ⭐⭐⭐⭐ |
| 12 | `12_smart_pointers.rs` | 智能指针 | Box, Rc, Arc, Cell, RefCell, Cow | ⭐⭐⭐⭐ |
| 13 | `13_async_await.rs` | 异步编程 | async/await, Future, 运行时 | ⭐⭐⭐⭐⭐ |
| 14 | `14_modules.rs` | 模块系统 | mod, pub, use, 路径, 可见性 | ⭐⭐⭐⭐ |
| 15 | `15_macros.rs` | 宏 | 声明式宏、过程宏、内置宏 | ⭐⭐⭐ |

## 各章节详解

### 01_basics.rs - 基础语法
- 变量和可变性（let, mut, const）
- 基本数据类型（整数、浮点、布尔、字符、元组、数组）
- 函数定义和表达式
- 控制流（if, loop, while, for）

### 02_ownership.rs - 所有权系统 ⭐
- 所有权三大规则
- 移动语义 vs Copy语义
- 函数中的所有权转移
- Drop trait自动释放
- Clone深拷贝

### 03_borrowing.rs - 借用和引用 ⭐
- 不可变引用 &T
- 可变引用 &mut T
- 借用规则（防止数据竞争）
- 切片类型 &str
- 悬垂引用防护

### 04_struct_enum.rs - 结构体和枚举
- 结构体定义和方法
- 元组结构体和单元结构体
- 枚举定义和携带数据
- Option<T> 替代 null
- match 处理枚举

### 05_pattern_match.rs - 模式匹配
- match 表达式
- 多条件匹配和范围匹配
- 解构元组、结构体、枚举
- 匹配守卫和@绑定
- if let / while let

### 06_error_handling.rs - 错误处理
- Result<T, E> 枚举
- ? 操作符传播错误
- unwrap / expect
- 自定义错误类型
- panic vs Result 选择

### 07_trait_generics.rs - Trait和泛型 ⭐
- Trait 定义和实现
- Trait 作为参数和返回值
- 泛型函数和结构体
- Trait bound 和 where 子句
- Trait 对象（动态分发）

### 08_lifetime.rs - 生命周期 ⭐
- 生命周期概念和标注语法
- 函数中的生命周期
- 结构体中的生命周期
- 生命周期省略规则
- 'static 静态生命周期

### 09_closures.rs - 闭包
- 闭包语法 |args| { body }
- 捕获环境变量
- move 关键字
- Fn / FnMut / FnOnce traits
- 闭包作为参数和返回值
- 与迭代器配合使用

### 10_iterators.rs - 迭代器
- Iterator trait
- iter() / iter_mut() / into_iter()
- 适配器：map, filter, take, skip...
- 消费者：collect, fold, sum...
- 零成本抽象
- 自定义迭代器

### 11_concurrency.rs - 并发编程
- 创建线程 thread::spawn
- move 闭包
- 消息传递 mpsc::channel
- 共享状态 Mutex
- 原子引用计数 Arc
- Send 和 Sync traits

### 12_smart_pointers.rs - 智能指针
- Box<T> 堆分配
- Rc<T> 引用计数（单线程）
- Arc<T> 原子引用计数（多线程）
- Cell<T> 内部可变性（Copy类型）
- RefCell<T> 内部可变性（运行时检查）
- Deref 和 Drop traits
- Cow<T> 写时克隆

### 13_async_await.rs - 异步编程 ⭐
- async/await 语法
- Future trait
- 异步运行时（tokio, async-std）
- join! / select! 并发模式
- 异步错误处理
- 异步流 Stream
- 异步网络编程

### 14_modules.rs - 模块系统
- 模块定义 mod
- 可见性 pub / pub(crate) / pub(super)
- use 导入和 as 别名
- 路径：self / super / crate
- 文件模块和目录模块
- 重新导出 pub use
- 项目结构最佳实践

### 15_macros.rs - 宏
- 声明式宏 macro_rules!
- 宏参数类型（ident, expr, ty, tt...）
- 重复模式 $(...),*
- 卫生性（Hygiene）
- 内置宏（vec!, println!, format!...）
- 过程宏简介（派生、属性、函数式）

## 运行

```bash
# 运行所有示例
cargo run

# 运行测试
cargo test

# 检查代码
cargo check
```

## 关键概念对比

| 概念 | 其他语言 | Rust |
|------|---------|------|
| 内存管理 | 手动(C)/GC(Java) | 所有权+RAII |
| 空值 | null | Option<T> |
| 异常 | try-catch | Result<T, E> |
| 接口 | Java interface | Trait |
| 模板 | C++ templates | 泛型 |
| 引用 | C++引用 | 借用(&, &mut) |
| 闭包 | lambda表达式 | `\|\| {}` |
| 迭代器 | Iterator模式 | Iterator trait |
| 异步 | Promise/Future | async/await + Future |
| 宏 | 预处理器宏 | 卫生的宏系统 |

## 核心规则速查

### 所有权系统
```
- 每个值有唯一所有者
- 所有者离开作用域时值被释放
- 赋值/传递默认移动(move)
- Copy类型自动复制
```

### 借用规则
```
- 同时只能有一个可变引用 OR 多个不可变引用
- 引用必须始终有效（编译期检查）
- 引用不能比数据活得长
```

### 生命周期
```
- 'a 表示引用的有效作用域
- 编译器自动推断大多数情况
- 复杂场景需要手动标注
- 返回引用通常需要标注生命周期
```

### 智能指针选择
```
- 唯一所有权 -> Box<T>
- 共享所有权（单线程） -> Rc<T>
- 共享所有权（多线程） -> Arc<T>
- 内部可变性（Copy） -> Cell<T>
- 内部可变性（任意） -> RefCell<T>
- 共享可变 -> Rc<RefCell<T>> / Arc<Mutex<T>>
```

## 练习建议

1. **阅读代码** - 理解每个示例的注释
2. **注释代码** - 看编译错误理解规则
3. **修改示例** - 尝试不同的写法
4. **编写程序** - 实践完整的小项目
5. **阅读文档** - [The Rust Book](https://doc.rust-lang.org/book/)

## 进阶主题

| 主题 | 说明 | 重要程度 |
|------|------|---------|
| 测试 | 单元测试、集成测试、文档测试 | ⭐⭐⭐⭐ |
| 标准库集合 | HashMap, HashSet, BTreeMap, VecDeque | ⭐⭐⭐⭐ |
| 字符串处理 | String, &str, 格式化, 正则 | ⭐⭐⭐⭐ |
| unsafe Rust | 原始指针、unsafe块 | ⭐⭐⭐ |
| FFI | 与C交互, cbindgen | ⭐⭐ |
| Web开发 | actix-web, axum, warp | ⭐⭐⭐⭐ |
| 嵌入式 | no_std, embedded-hal | ⭐⭐⭐ |

## 项目结构

```
rust_learning/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs              # 主入口
    │
    │── 核心模块（按序号学习）
    ├── 01_basics.rs         # 基础语法
    ├── 02_ownership.rs      # 所有权系统 ⭐核心
    ├── 03_borrowing.rs      # 借用和引用 ⭐核心
    ├── 04_struct_enum.rs    # 结构体和枚举
    ├── 05_pattern_match.rs  # 模式匹配
    ├── 06_error_handling.rs # 错误处理
    ├── 07_trait_generics.rs # Trait和泛型 ⭐核心
    ├── 08_lifetime.rs       # 生命周期 ⭐核心
    ├── 09_concurrency.rs    # 并发编程
    │
    │── 进阶模块
    ├── closures.rs          # 闭包
    ├── iterators.rs         # 迭代器
    ├── smart_pointers.rs    # 智能指针
    ├── async_await.rs       # 异步编程
    ├── modules.rs           # 模块系统
    └── macros.rs            # 宏
```

## 参考资源

- [The Rust Programming Language](https://doc.rust-lang.org/book/) - 官方教程
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 代码示例
- [Rust标准库文档](https://doc.rust-lang.org/std/) - API参考
- [Rustlings](https://github.com/rust-lang/rustlings) - 练习题
- [Rust设计模式](https://rust-unofficial.github.io/patterns/) - 设计模式
- [Tokio教程](https://tokio.rs/tokio/tutorial) - 异步运行时