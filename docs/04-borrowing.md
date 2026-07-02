# 第 4 章：借用与引用

## 学习目标

- 理解引用（&）和借用（borrow）的概念
- 掌握不可变引用和可变引用的规则
- 理解切片（slice）类型
- 了解 NLL（Non-Lexical Lifetimes）的作用

## 为什么要借用？

上一章中，每次把 `String` 传入函数都会失去所有权，需要再返回出来才能继续使用。这很繁琐：

```rust
fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)  // 必须返回 s，否则调用者无法继续使用
}

let s = String::from("hello");
let (s, len) = calculate_length(s);  // 麻烦！
```

**借用**解决了这个问题——你不需要拥有值，只需要暂时"借"来用一下。

## 引用（Reference）

```rust
fn calculate_length(s: &String) -> usize {  // & 表示引用
    s.len()
}  // s 离开作用域，但因为不拥有所有权，不会释放 String

let s = String::from("hello");
let len = calculate_length(&s);  // &s 创建一个引用
println!("'{}' 的长度是 {}", s, len);  // s 仍然有效！
```

`&` 创建引用，`*` 解引用。引用不会获取所有权，只是"借用"。

> 💡 Rust 的引用类似 C++ 的引用，但有**编译期的安全规则**约束。	

## 可变引用

默认引用是不可变的。要修改值，需要 `&mut`：

```rust
fn change(s: &mut String) {
    s.push_str(", world");
}

let mut s = String::from("hello");
change(&mut s);
println!("{}", s);  // "hello, world"
```

## 借用规则

这是 Rust 最重要的安全规则之一：

> **同一时刻，只能拥有以下两者之一：**
> - 任意数量的不可变引用（`&T`）
> - 恰好一个可变引用（`&mut T`）
>
> **引用必须始终有效**（不能有悬垂引用）

```rust
let mut s = String::from("hello");

// ✅ 多个不可变引用
let r1 = &s;
let r2 = &s;
println!("{}, {}", r1, r2);

// ✅ 一个可变引用
let r3 = &mut s;
r3.push_str("!");

// ❌ 同时有不可变和可变引用
let r1 = &s;
let r2 = &mut s;  // 编译错误！
```

> 💡 这个规则防止了**数据竞争**（data race）——多个指针同时访问同一数据，且至少有一个在写。

### NLL（Non-Lexical Lifetimes）

Rust 编译器足够智能，**引用的作用域到它最后一次使用为止**，而不是到作用域结束：

```rust
let mut s = String::from("hello");

let r1 = &s;
println!("{}", r1);  // r1 最后一次使用
// r1 在这里就"结束"了

let r2 = &mut s;  // ✅ OK！r1 已经不再使用
r2.push_str(" world");
```

这意味着很多看似违反借用规则的情况实际上能通过编译。

## 悬垂引用

Rust 在编译期**阻止悬垂引用**：

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // 编译错误：s 会在函数结束时被释放
}
// 返回的引用指向已经被释放的内存——危险！
```

正确做法是返回所有权：

```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 所有权转移出去
}
```

## 切片（Slice）

切片是对集合中一部分元素的**引用**：

```rust
let s = String::from("hello world");

// 字符串切片
let hello = &s[0..5];   // "hello"
let world = &s[6..11];  // "world"
let all = &s[..];       // "hello world"

// 数组切片
let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..3];  // [2, 3]
```

常用缩写：

| 写法 | 等价于 |
|------|--------|
| `&s[0..5]` | 从开头到索引 5（不含） |
| `&s[..5]` | 同上 |
| `&s[6..]` | 从索引 6 到末尾 |
| `&s[..]` | 整个字符串 |

> 💡 **技巧：** 在函数参数中使用 `&str` 而不是 `&String`，因为 `&str` 更通用——`&String` 会自动转换为 `&str`（deref coercion）。

```rust
fn first_word(s: &str) -> &str {  // 用 &str，更灵活
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

let s = String::from("hello world");
let word = first_word(&s);  // &String → &str 自动转换
let word = first_word("hello world");  // 字符串字面量也可以
```

## 跨语言对比

| 概念 | Rust | C/C++ | Java/Python |
|------|------|-------|-------------|
| 引用 | `&T` / `&mut T`，编译期检查 | 原始指针 / `&`，无保护 | 所有对象都是引用 |
| 数据竞争 | 编译错误 | 未定义行为 | 运行时保证 |
| 切片 | `&[T]`，安全 | 指针+长度，不安全 | 无直接对应 |
| 悬垂引用 | 编译错误 | 运行时崩溃/UB | 不会发生（有 GC） |

## 常见陷阱

> ⚠️ **在还有不可变引用时尝试获取可变引用。** 记住 NLL 规则——不可变引用的生命周期结束于最后一次使用。如果你在 `println!` 之后再用 `&mut`，可能是 OK 的。

> ⚠️ **返回局部变量的引用。** 编译器会直接报错。解决方案：返回所有权（`String` 而非 `&String`），或者让调用者传入引用。

## 练习

1. 写一个函数，接收 `&mut String` 并反转其中的字符（提示：`s.chars().rev().collect()`）
2. 故意写出违反借用规则的代码，仔细阅读编译错误信息
3. 写一个 `first_word` 函数返回第一个单词的切片

---

← [第 3 章：所有权系统](./03-ownership.md) | [返回目录](./README.md) | → [第 5 章：结构体与枚举](./05-struct-enum.md)
