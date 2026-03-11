# Rust 学习项目

通过代码实践学习 Rust 核心语法和思想。

## 学习顺序

推荐按照以下顺序学习：

1. **01_basics.rs** - 基础语法（变量、数据类型、函数、控制流）
2. **02_ownership.rs** - 所有权系统 ⭐ **Rust核心**
3. **03_borrowing.rs** - 借用和引用 ⭐ **Rust核心**
4. **04_struct_enum.rs** - 结构体和枚举
5. **05_pattern_match.rs** - 模式匹配
6. **06_error_handling.rs** - 错误处理
7. **07_trait_generics.rs** - Trait和泛型 ⭐ **重要**
8. **08_lifetime.rs** - 生命周期 ⭐ **Rust核心**
9. **09_concurrency.rs** - 并发编程

## 运行

```bash
cargo run
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

## 核心区别

### 所有权系统
- 每个值有唯一所有者
- 所有者离开作用域时值被释放
- 赋值/传递默认**移动**(move)

### 借用规则
- 同时只能有一个可变引用或多个不可变引用
- 引用必须始终有效（编译期检查）

### 生命周期
- `'a` 表示引用的有效作用域
- 编译器自动推断大多数情况
- 复杂场景需要手动标注

## 练习建议

1. 注释掉代码看编译错误
2. 尝试修改示例代码
3. 编写自己的小程序
4. 阅读 [The Rust Book](https://doc.rust-lang.org/book/)

## 进阶

- 智能指针：Box, Rc, Arc, Cell, RefCell
- 异步编程：async/await, tokio
- 宏：macro_rules!
- unsafe Rust