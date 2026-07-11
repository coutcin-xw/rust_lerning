# 第 10 章：生命周期

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

## 多个生命周期参数

当多个引用有不同的生命周期约束时，需要多个标注：

```rust
// 结构体包含两个不同生命周期的引用
struct Context<'a, 'b> {
    config: &'a str,   // config 可能来自全局配置，生命周期较长
    request: &'b str,   // request 可能来自网络，生命周期较短
}

impl<'a, 'b> Context<'a, 'b> {
    fn new(config: &'a str, request: &'b str) -> Self {
        Context { config, request }
    }

    fn config(&self) -> &'a str { self.config }
    fn request(&self) -> &'b str { self.request }
}

// 函数返回不同生命周期的引用
fn first_word<'a, 'b>(full: &'a str, delimiter: &'b str) -> &'a str {
    // 返回值只和 full 的生命周期有关，和 delimiter 无关
    full.split(delimiter).next().unwrap_or(full)
}
// 标注 'a 和 'b 分别，编译器不会把返回值的生命周期限制为 &delimiter
```

> 💡 不需要的生命周期参数不要加——`first_word` 其实可以省略 `'b`（规则 1 自动处理），这里显式标出只是为了展示语法。

## `dyn Trait + 'a` — trait 对象上的生命周期

trait 对象有默认的生命周期推断，但有时需要显式标注：

```rust
trait Draw { fn draw(&self); }

// 默认情况：Box<dyn Draw> 等价于 Box<dyn Draw + 'static>
// 意味着被包装的值不能包含非 'static 的引用

struct View<'a> {
    label: &'a str,
}

impl<'a> Draw for View<'a> {
    fn draw(&self) { println!("视图: {}", self.label); }
}

fn main() {
    let name = String::from("主界面");

    // ❌ 错误：View<'_> 包含非 'static 引用，不能放入 Box<dyn Draw>
    // let v: Box<dyn Draw> = Box::new(View { label: &name });

    // ✅ 显式标注生命周期：允许包装包含 'a 引用的类型
    let v: Box<dyn Draw + '_> = Box::new(View { label: &name });
    v.draw();
}
```

生命周期标注规则：
- `Box<dyn Trait>` 默认 `Box<dyn Trait + 'static>`（trait 对象本身不含非 static 引用）
- `Box<dyn Trait + 'a>` 允许 trait 对象包含生命周期不短于 `'a` 的引用
- `Box<dyn Trait + '_>` 是常用简写——编译器自动推断最短可行的生命周期

## 常见生命周期错误及修复

### 错误 1：返回局部变量的引用

```rust,ignore
fn bad() -> &str {
    let s = String::from("hello");
    &s  // ❌ s 在函数结束时被释放
}
```

**修复：返回所有权（返回 `String` 而不是 `&str`）**

### 错误 2：借用和可变借用混淆

```rust,ignore
fn modify_and_read(v: &mut Vec<i32>) -> &i32 {
    v.push(42);
    &v[0]  // ❌ 可变借用 v 和不可变借用 &v[0] 冲突？其实可以
}
// 这个在 NLL 下其实是 OK 的——编译器能看出 push 不需要保留可变借用
// 真正的坑是：
fn bad() {
    let mut v = vec![1, 2, 3];
    let r = &mut v;
    r.push(4);
    println!("{}", v[0]);  // ❌ 可变借用 r 和不可变借用 v 冲突
    // 即使 r 没有再被使用，编译器有时还是保守（尤其在 2021 Edition）
}
```

### 错误 3：结构体引用比数据活得久

```rust,ignore
struct Holder<'a> {
    data: &'a str,
}

fn create_holder() -> Holder {
    let s = String::from("data");
    Holder { data: &s }  // ❌ s 在函数结束时释放，Holder.data 悬垂
}
```

**修复：让结构体持有所有权（用 `String` 而非 `&str`）**

### 错误 4：trait 对象生命周期不匹配

```rust,ignore
fn make_view<'a>(label: &'a str) -> Box<dyn Draw> {
    Box::new(View { label })  // ❌ View<'a> 不满足 dyn Draw 的 'static 要求
}

fn make_view<'a>(label: &'a str) -> Box<dyn Draw + 'a> {
    Box::new(View { label })  // ✅
}
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

← [第 9 章：类型转换与运算符重载](./09-type-conversions.md) | [返回目录](./README.md) | → [第 11 章：闭包](./11-closures.md)
