# 第 9 章：生命周期

## 学习目标

- 理解生命周期标注的**真实含义**（不是改变生命周期，只是描述关系）
- 掌握 `'a` 语法在函数、结构体、方法中的使用
- 熟记生命周期省略的三条规则
- 理解 `'static` 的含义和恰当的使用场景
- 体会 RPIT 2024 Edition 中生命周期自动捕获带来的便利

## 为什么会有生命周期？

看这个函数——编译器不知道返回哪个引用：

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
// 编译错误：expected named lifetime parameter
```

编译器的问题：**返回的 `&str` 和参数 `x`、`y` 之间是什么关系？** 返回的引用是从哪里来的？调用者应该怎么处理？

答案需要用生命周期标注告诉编译器。

## 生命周期标注语法

```rust
// 'a 读作"生命周期 a"
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

标注 `'a` 的含义：

```
'x 是 x 的生命周期，'y 是 y 的生命周期。
'a 是 x 和 y 的公共部分（两者"重叠"的生命周期）。
返回值必须和 'a 一样长。
```

> 💡 **关键洞察：生命周期标注不延长任何变量的实际生命周期。** 它只是**向编译器描述引用之间的关系**。编译器用它来检查你的代码是否安全——如果标注的关系成立就编译通过，否则报错。

### 带标注后如何使用

```rust
fn main() {
    let s1 = String::from("short");
    {
        let s2 = String::from("loooooong");
        let result = longest(s1.as_str(), s2.as_str());
        // s1 和 s2 在此都有效
        // result 的生命周期 = min(s1, s2) = s2 的作用域
        println!("{}", result);  // ✅
    }  // s2 被释放
    // println!("{}", result);  // ❌ result 在此无效
}
```

## 结构体中的生命周期

当结构体包含引用时：

```rust
struct Excerpt<'a> {
    part: &'a str,  // 这个 part 引用的有效期 ≥ 结构体实例的生命周期
}

let novel = String::from("从前有座山。山上有座庙。");
let first = &novel[..9];  // "从前有座山。"
let excerpt = Excerpt { part: first };
// excerpt 不能比 novel 活得久——否则 part 就变成悬垂引用了
```

直观理解：
```
novel:     |═════════════|  (String 的作用域)
first:        |═══════|      (&str 引用，必须在 novel 内)
excerpt:      |═══════|      (Excerpt<'a>, part 的有效期≥它的有效期)
```

## 方法中的生命周期

```rust
impl<'a> Excerpt<'a> {
    // 规则 3：&self 的生命周期自动赋给返回值
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("{}", announcement);
        self.part
    }
}
```

## 生命周期省略规则

编译器大多数时候能自动推断，不需要你标注。规则有三条：

1. **每个引用参数各有自己的生命周期**：`fn foo(x: &str, y: &str)` → `fn foo<'a, 'b>(x: &'a str, y: &'b str)`
2. **如果只有一个输入生命周期，它赋给所有输出生命周期**：`fn foo(x: &str) -> &str` → `fn foo<'a>(x: &'a str) -> &'a str`
3. **如果有 `&self`，`self` 的生命周期赋给所有输出**

这三条规则覆盖了大约 90% 的日常场景。当三条规则都不适用时（如 `longest` 函数有两个输入生命周期），编译器要求你手动标注。

## 'static —— 整个程序存活期

```rust
let s: &'static str = "我是静态字符串";  // 字符串字面量有 'static 生命周期

// 'static 意味着"活到程序结束"
// 常见于：
// - 字符串字面量
// - 全局常量
// - 泄露的 Box（Box::leak）
```

> ⚠️ **不要用 `'static` 来"修复"生命周期错误。** 如果你写了 `fn foo() -> &'static str` 只是为了让编译通过，那快止。重新思考设计——你的数据所有权结构可能有问题。

### 'static 的合法使用

```rust
use std::thread;

fn spawn_task() {
    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        // data 被 move 进线程，线程可能比函数活得久
        println!("{:?}", data);
    });
    handle.join().unwrap();
}
// 这个闭包的类型是 impl FnOnce() + Send + 'static
// 'static 表示"没有借用数据"或"借用的数据是 'static"
// move 闭包把所有权转移进去，所以满足了 'static 约束
```

## 泛型 + Trait Bound + 生命周期的组合

```rust
use std::fmt::Display;

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,  // 注意这个 T 不含引用——不需要给它生命周期
{
    println!("公告: {}", ann);
    if x.len() > y.len() { x } else { y }
}
```

## RPIT 生命周期捕获 —— 2024 Edition 的改进

> 📘 *Rust 2024 Edition 中，`impl Trait` 返回值**自动捕获所有生命周期参数**，不需要手动 `+ '_`。*

### 以前的问题

```rust
// 2021 Edition：需要 + '_ 来捕获 'a
fn process<'a>(data: &'a str) -> impl Display + '_ {
    format!("处理: {}", data)
}
```

### 2024 Edition：自然用法

```rust
// 2024 Edition：'a 被自动捕获，直接写就行
fn process<'a>(data: &'a str) -> impl Display {
    format!("处理: {}", data)
}
```

### 精确控制：use<..>

当你不希望捕获某个生命周期时：

```rust
fn get_iter<'a, 'b>(x: &'a str, y: &'b str) -> impl Iterator<Item = char> + use<'a> {
    // 只捕获 'a，y 不能在迭代器中使用
    x.chars()
}
```

## 生命周期速记口诀

```
引用不拥有，只是暂时借。
借用有长短，标注来描述。
一个入参时，自动赋出参。
多个入参时，标注要手动。
设计先思考，谁拥有数据？
owner 活最久，引用嵌套中。
```

## 常见陷阱

> ⚠️ **以为生命周期标注能让变量活得更久。** 标注不改变任何事情。它只是把已经存在的关系明确化。

> ⚠️ **错误信息 "does not live long enough"。** 这不是"生命周期太短"，而是"引用和数据的所有关系不对"。检查：谁拥有这个数据？引用从哪来的？它们的作用域如何嵌套？

> ⚠️ **滥用 `'static`。** 如果编译器建议你加 `'static`，99% 的情况是你的设计有问题。重新思考所有权链条。

> ⚠️ **在结构体中不加生命周期。** 任何包含引用的结构体都需要生命周期。想清楚：结构体能比引用活得久吗？大多数时候你不需要引用——直接用 `String` 而不是 `&str`。

## 练习

1. 实现 `fn shortest<'a>(x: &'a str, y: &'a str) -> &'a str`——返回较短的那个
2. 定义一个 `Config<'a>` 结构体，包含一个 `path: &'a str` 字段，实现方法和关联函数
3. 写一个 RPIT 返回函数：接收 `&str`，返回 `impl Display`。尝试在 2024 Edition 下编译，验证生命周期自动捕获
4. 分析以下代码为什么不编译，并修复：

```rust
fn return_ref() -> &str {
    let s = String::from("hello");
    &s
}
```

---

← [第 8 章：Trait 与泛型](./08-trait-generics.md) | [返回目录](./README.md) | → [第 10 章：闭包](./10-closures.md)
