# 第 3 章：所有权系统

## 学习目标

- 理解 Rust 所有权的三大规则
- 区分移动（move）语义和复制（copy）语义
- 理解 Drop trait 和自动释放机制
- 掌握 Clone 深拷贝的使用场景

## 为什么需要所有权？

大多数语言的内存管理方式：

| 方式 | 代表语言 | 问题 |
|------|---------|------|
| 手动管理 | C/C++ | 内存泄漏、悬垂指针、double free |
| 垃圾回收（GC） | Java/Python/Go | 运行时开销、"stop the world" 暂停 |
| **所有权系统** | **Rust** | **编译期确定性释放，零运行时开销** |

Rust 在**编译时**通过所有权规则检查内存安全，不需要 GC，也不用手动 free。

## 所有权三大规则

1. Rust 中的**每个值都有一个所有者**（owner）
2. **同一时刻只能有一个所有者**
3. 所有者**离开作用域**时，值被自动释放

```rust
{
    let s = String::from("hello");  // s 是此字符串的所有者
    // ... 使用 s
}  // s 离开作用域，String 的内存自动释放
```

## 移动（Move）语义

当把堆上分配的值赋给另一个变量时，发生**所有权转移**（move），原变量失效：

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 的所有权转移给 s2

// println!("{}", s1);  // 编译错误！s1 已失效
println!("{}", s2);     // OK
```

> 💡 这样设计是为了防止 **double free**：如果 `s1` 和 `s2` 都指向同一块堆内存，离开作用域时会释放两次，导致内存错误。

函数调用同样会转移所有权：

```rust
fn takes_ownership(s: String) {
    println!("{}", s);
}  // s 被 drop

let s = String::from("hello");
takes_ownership(s);
// println!("{}", s);  // 编译错误！s 的所有权已转移
```

## Copy 类型

**存储在栈上的简单类型**实现了 `Copy` trait，赋值时自动复制而不会移动：

```rust
let x = 5;
let y = x;   // x 被复制，仍然有效
println!("x={}, y={}", x, y);  // 两个都能用
```

实现了 `Copy` 的类型：

- 所有整数类型（`i32`, `u64` 等）
- 浮点类型（`f32`, `f64`）
- 布尔类型 `bool`
- 字符类型 `char`
- 元素全是 `Copy` 的元组，如 `(i32, i32)`

> ⚠️ `(i32, String)` **不是** `Copy`，因为 `String` 不是 `Copy`。

## Drop 与自动释放

当值离开作用域时，Rust 自动调用 `Drop` trait 的 `drop` 方法。类似 C++ 的析构函数：

```rust
struct Resource { name: String }

impl Drop for Resource {
    fn drop(&mut self) {
        println!("释放: {}", self.name);
    }
}

{
    let r = Resource { name: String::from("db连接") };
    println!("使用资源中...");
}  // r 离开作用域，自动调用 drop
```

> 📘 *尾表达式中临时值的释放时机在 Rust 2024 Edition 中有所优化——临时值在局部变量之前释放，这使得一些原本会编译失败的借用模式现在可以通过。*

### 手动提前释放

```rust
let s = String::from("hello");
drop(s);  // 显式释放，之后 s 不可用
// 注意：不能写成 s.drop()，编译器会报错
```

## 返回所有权

函数返回时，所有权转移给调用者：

```rust
fn create_string() -> String {
    let s = String::from("hello");
    s  // 所有权转移给调用者
}

fn take_and_return(s: String) -> String {
    s  // 接收所有权，再返回出去
}

let s1 = create_string();    // 获得所有权
let s2 = take_and_return(s1); // s1 转移进函数，s2 获得所有权
```

这种方式比较繁琐，下一章会介绍**借用**来解决。

## Clone 深拷贝

当需要堆数据的独立副本时：

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // 深拷贝，堆数据也被复制

println!("s1={}, s2={}", s1, s2);  // 两个都有效，互不影响
```

`clone()` 有性能开销，只在确实需要独立副本时使用。

## 常见陷阱

> ⚠️ **误以为所有类型赋值都是 copy。** `String`、`Vec` 等堆分配类型赋值是 move。判断方法：如果一个类型实现了 `Copy`，赋值就复制；否则就移动。

> ⚠️ **在 move 后使用原变量。** 编译器会明确报错 `value borrowed after move`，这是 Rust 最经典的编译错误之一，遇到它说明你在 move 后尝试访问变量。

## 练习

1. 写一个函数，接收并返回一个 `String`，观察所有权的流入和流出
2. 尝试让一个自定义 struct 在 move 后继续使用，观察编译错误
3. 实现 `Drop` trait 打印日志，验证值的释放时机

---

← [第 2 章：基础语法](./02-basics.md) | [返回目录](./README.md) | → [第 4 章：借用与引用](./04-borrowing.md)
