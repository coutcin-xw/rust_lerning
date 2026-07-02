# 第 9 章：生命周期

## 学习目标

- 理解生命周期的概念和必要性
- 掌握生命周期标注语法 `'a`
- 熟记生命周期省略规则
- 理解 `'static` 的含义和使用限制
- 了解 **RPIT 生命周期自动捕获**（2024 Edition）

## 为什么需要生命周期？

看这个例子——编译器无法确定返回哪个引用：

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
// 编译错误：返回的引用和 x、y 的关系不明确
```

编译器需要知道：**返回的引用和输入参数之间存在什么关系？** 这就需要用生命周期标注来描述。

## 生命周期标注语法

```rust
// 'a 读作"生命周期 a"，它标注 x、y 和返回值之间的关系
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

> 💡 **关键理解：** 生命周期标注**不改变引用的实际生命周期长短**，只是向编译器描述引用之间的关系。编译器用这些信息检查你的代码是否安全。

标注 `'a` 的含义是：`x` 和 `y` 至少活得和 `'a` 一样长，返回的引用也活得和 `'a` 一样长。

## 结构体中的生命周期

当结构体包含引用时，必须标注生命周期：

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,  // 这个引用的有效期 ≥ 结构体实例的生命周期
}

let novel = String::from("从前有座山...");
let first = &novel[..3];
let excerpt = ImportantExcerpt { part: first };
// excerpt 不能比 novel 活得长
```

## 生命周期省略规则

编译器在大多数情况下可以自动推断生命周期，遵循三条规则：

1. **每个引用参数都有自己的生命周期参数**：`fn foo<'a>(x: &'a str)`
2. **如果只有一个输入生命周期参数，它被赋给所有输出生命周期参数**：`fn foo(x: &str) -> &str` 自动变成 `fn foo<'a>(x: &'a str) -> &'a str`
3. **如果有 `&self`，`self` 的生命周期赋给所有输出生命周期参数**

这些规则覆盖了大多数常见场景。当规则不够时，编译器会要求你手动标注。

## 方法中的生命周期

```rust
impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        // 根据省略规则（规则3），返回值的生命周期来自 &self
        println!("{}", announcement);
        self.part
    }
}
```

## 'static 静态生命周期

`'static` 表示引用在**整个程序运行期间都有效**：

```rust
let s: &'static str = "我是静态字符串";  // 字符串字面量默认是 'static
```

> ⚠️ 不要随意给引用标注 `'static`。编译器提示生命周期不够长时，通常是你的设计有问题——重新思考数据的所有关系，而不是用 `'static` 来"规避"。

## RPIT 生命周期捕获（2024 Edition）

> 📘 *在 Rust 2021 Edition 中，`impl Trait` 返回值只捕获出现在 bound 中的生命周期参数。在 Rust 2024 Edition 中，**所有生命周期参数都被自动捕获**，与 `async fn` 行为一致。*

这意味着以下代码在 2024 Edition 中直接可用：

```rust
// 2024 Edition：✅ 编译通过，'a 被自动捕获
fn process<'a>(data: &'a str) -> impl std::fmt::Display {
    format!("处理: {}", data)
}
```

如果需要精确控制捕获哪些生命周期，使用 `use<..>` 语法（跨 Edition 可用）：

```rust
fn foo<'a, 'b>(x: &'a str, y: &'b str) -> impl std::fmt::Display + use<'a> {
    // 只捕获 'a
    format!("{}", x)
}
```

## 泛型 + Trait Bound + 生命周期的组合

完整语法：

```rust
use std::fmt::Display;

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("公告：{}", ann);
    if x.len() > y.len() { x } else { y }
}
```

## 常见陷阱

> ⚠️ **在结构体中存储引用而不标注生命周期。** 任何包含引用的结构体都需要生命周期参数。想清楚——这个结构体实例能比它包含的引用活得更长吗？

> ⚠️ **误解生命周期标注的作用。** 生命周期标注是给编译器看的约束，不是给运行时的指令。标注 `'a` 不会延长引用实际存活的时间。

> ⚠️ **滥用 `'static`。** 如果编译器说"does not live long enough"，先想想能否重新组织代码让引用关系更清晰，而不是直接加 `'static`。

## 练习

1. 写一个函数，接收两个 `&str` 参数，返回较短的那个（需要标注生命周期）
2. 定义一个包含两个引用的结构体，分析不同生命周期标注对结构体使用的影响
3. 用 `use<..>` 语法练习精确控制 RPIT 的生命周期捕获

---

← [第 8 章：Trait 与泛型](./08-trait-generics.md) | [返回目录](./README.md) | → [第 10 章：闭包](./10-closures.md)
