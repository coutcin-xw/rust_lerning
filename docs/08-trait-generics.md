# 第 8 章：Trait 与泛型

## 学习目标

- 定义并实现 trait（Rust 的接口抽象）
- 用泛型编写类型参数化的代码
- 理解 trait bound 如何约束泛型
- 掌握 `impl Trait` 在参数和返回值中的用法
- 理解静态分发（单态化）和动态分发（`dyn Trait`）的取舍
- 使用 RPIT 和 `use<..>` 精确控制生命周期捕获

## 概念层：Trait 和泛型解决什么问题？

### 问题 1：共享行为

假设你有一个新闻文章和一条推文，它们都需要"生成摘要"。在 Java 里你会写一个 `interface`，在 Go 里你会写一个 `interface` 类型，在 C++ 里你会用虚函数。Rust 的选择是 **trait**：

```rust
// 不用 trait：每个类型有各自的方法名，无法统一调用
fn summarize_article(a: &NewsArticle) -> String { /* ... */ }
fn summarize_tweet(t: &Tweet) -> String { /* ... */ }

// 用 trait：统一的方法签名，调用者不需要知道具体类型
fn notify(item: &impl Summary) {
    println!("{}", item.summarize());  // 统一的调用方式
}
```

trait 不定义"对象是什么"，而定义**"能做什么"**——不关心你是 `NewsArticle` 还是 `Tweet`，只关心你能不能 `summarize()`。

### 问题 2：代码复用

泛型让你写一份代码处理多种类型。不用泛型的话，你需要为每个类型重复实现：

```rust
// 不用泛型：i32 和 char 各写一份
fn largest_i32(list: &[i32]) -> &i32 { /* ... */ }
fn largest_char(list: &[char]) -> &char { /* ... */ }

// 用泛型：一份代码适配所有可比较的类型
fn largest<T: PartialOrd>(list: &[T]) -> &T { /* ... */ }
```

### 与接口/继承的关键区别

| | Rust trait | Java interface | C++ 虚函数 |
|---|---|---|---|
| 定义位置 | trait 所在 crate **或** 类型所在 crate（孤儿规则） | 类型定义时必须声明实现 | 声明类时指定继承 |
| 后期绑定 | 明确用 `dyn Trait` 选择动态分发 | 默认动态分发 | 声明 `virtual` 即可 |
| 默认实现 | ✅ 可在 trait 中提供 | ✅ default 方法 | ✅ |
| 多继承 | ✅ 实现多个 trait | ✅ 实现多接口 | ✅ 多继承（含菱形问题） |
| 零成本静态分发 | ✅ 默认（`<T: Trait>`） | ❌ | ✅（模板） |

> 💡 Rust 的 trait 是**组合优于继承**的典范。你可以为已有类型实现新 trait（在孤儿规则允许下），不修改原类型定义。这在你使用第三方库的类型时尤其强大。

## Trait — 定义共享行为

Trait 告诉编译器"实现了这个 trait 的类型必须提供这些方法"：

```rust
trait Summary {
    // 抽象方法 — 实现者必须提供
    fn summarize(&self) -> String;

    // 默认实现 — 实现者可以覆盖
    fn summarize_author(&self) -> String {
        String::from("(未知作者)")
    }

    // 默认方法可以调用抽象方法 — 实现者获得此方法"免费"
    fn summarize_with_author(&self) -> String {
        format!("{} — 作者: {}", self.summarize(), self.summarize_author())
    }
}
```

### 为类型实现 Trait

```rust
struct NewsArticle {
    headline: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}", self.headline)
    }

    fn summarize_author(&self) -> String {
        format!("作者: {}", self.author)
    }
}

// 实现多个 trait
impl std::fmt::Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[新闻] {}", self.headline)
    }
}
```

> ⚠️ **孤儿规则**：你只能在 trait 所在的 crate **或**类型所在的 crate 中实现 trait。不能为外部类型实现外部 trait。这保证了 trait 实现的全局一致性。

## Trait 作为参数

### impl Trait 语法糖

```rust
fn notify(item: &impl Summary) {
    println!("新闻: {}", item.summarize());
}
// item 可以是任何实现了 Summary 的类型
```

等价泛型写法：

```rust
fn notify<T: Summary>(item: &T) {
    println!("新闻: {}", item.summarize());
}
```

### 多个 Trait Bound

```rust
fn notify(item: &(impl Summary + std::fmt::Display)) { }

// 泛型 + where
fn notify<T>(item: &T)
where
    T: Summary + std::fmt::Display,
{ }
```

### where 子句 — 复杂约束的救星

```rust
// 不用 where（难以阅读）
fn complex<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> String { }

// 用 where（清晰）
fn complex<T, U>(t: &T, u: &U) -> String
where
    T: Display + Clone,
    U: Clone + Debug,
{
    format!("{} {:?}", t, u)
}
```

## 泛型 — 参数化类型

### 泛型函数

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

let numbers = vec![34, 50, 25, 100];
let result = largest(&numbers);             // T = i32
let chars = vec!['y', 'm', 'a', 'q'];
let result = largest(&chars);               // T = char
```

> 💡 **单态化**：编译器为 `largest::<i32>` 和 `largest::<char>` 生成两份独立的机器码。运行时零开销——没有虚表查找，没有装箱。这就是"零成本抽象"。

### 泛型结构体

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point { x: self.x, y: other.y }
    }
}

let p1 = Point { x: 5, y: 10.0 };      // Point<i32, f64>
let p2 = Point { x: "x", y: 'c' };     // Point<&str, char>
let p3 = p1.mixup(p2);                  // Point<i32, char>
```

### 有条件地实现方法

```rust
impl<T: Display + PartialOrd> Point<T, T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x 更大: {}", self.x);
        } else {
            println!("y 更大: {}", self.y);
        }
    }
}

let p1 = Point { x: 5, y: 10 };
p1.cmp_display();  // ✅ i32 实现了 Display + PartialOrd

// 如果 T 没实现 Display，cmp_display 就不存在
// 编译器在调用处检查，没有实现就不让调用
```

## RPIT — 返回 impl Trait

```rust
fn returns_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("重大新闻"),
        author: String::from("佚名"),
        content: String::new(),
    }
}
// 调用者只知道返回类型实现了 Summary，不依赖具体类型
```

> 📘 *在 Rust 2024 Edition 中，`impl Trait` 返回值**自动捕获所有生命周期参数**。这与 `async fn` 的行为统一。*

```rust
// 2024 Edition：✅ 'a 被自动捕获
fn greet<'a>(name: &'a str) -> impl std::fmt::Display {
    format!("Hello, {}!", name)
}
```

### use<..> — 精确控制捕获

> 📘 *`use<..>` 语法（Rust 1.82 稳定，跨 Edition 可用）让你显式指定 `impl Trait` 捕获哪些泛型参数：*

```rust
fn process<'a, 'b, T>(x: &'a T, y: &'b T) -> impl std::fmt::Display + use<'a, T> {
    // 只捕获 'a 和 T，不捕获 'b
    format!("处理: {:?}", x)
}
```

## 静态分发 vs 动态分发

| | 静态分发（泛型） | 动态分发（trait 对象） |
|---|---|---|
| 语法 | `<T: Trait>` | `&dyn Trait` 或 `Box<dyn Trait>` |
| 确定时机 | 编译期单态化 | 运行时虚表 |
| 性能 | 内联优化、零开销 | 虚函数调用开销 |
| 二进制大小 | 每种类型一份代码 | 一份代码 |
| 集合 | 不能存不同类型 | 可以存不同类型的 trait 对象 |

### 什么是 `dyn Trait`？— 概念层

泛型 `<T: Trait>` 是一个**编译期占位符**——编译器为每个具体类型各生成一份代码。但有时你需要在**运行时**处理"某种实现了某个 trait 的东西"，而编译期你不知道具体是哪个类型。

这就是 `dyn Trait` 的用武之地：

```
dyn Draw  不是一个类型，而是一种"约束声明"
          它表示："任何实现了 Draw trait 的类型"
          
&dyn Draw  是一个引用，指向"某个实现了 Draw trait 的未知类型"
           注意 & 是必须的——因为 dyn Draw 的大小编译期未知（见下文）
```

**`dyn` 关键字的作用：**

`dyn` 是一个类型修饰符（不是类型本身）。它告诉编译器：
- "这个位置将要存放的是一个 trait 对象"
- "不要尝试在编译期确定具体类型——去查虚表"

```rust
// 对比理解：
fn static_dispatch<T: Draw>(item: &T) {   // T 是编译期占位符
    item.draw();  // 编译器知道：调用 T::draw
}

fn dynamic_dispatch(item: &dyn Draw) {     // dyn Draw 是运行时约束
    item.draw();  // 编译器生成：查虚表 → 调用对应函数
}
```

> 💡 **一句话**：`dyn` 不是类型，是告诉编译器"别找我确定类型，等到运行时去问虚表"。`dyn Draw` 不是一个具体类型，它就是"实现了 Draw 的某个东西"的运行时表示。

### trait 对象的底层机制：Fat Pointer

`dyn Trait` 本质上是一个**胖指针（fat pointer）**，由两部分组成：

```
┌──────────────────────────────────┐
│  &dyn Draw                       │
├──────────────┬───────────────────┤
│  data ptr    │  vtable ptr       │
│  (8 bytes)   │  (8 bytes)        │
├──────────────┴───────────────────┤
│  总大小: 16 bytes（在 64 位平台）  │
└──────────────────────────────────┘
```

- **data ptr**：指向堆上具体类型实例的数据（如 `Button` 的 `label` 字段）
- **vtable ptr**：指向虚表（virtual table），虚表中存储了该类型对 trait 每个方法的具体函数指针

当你调用 `component.draw()` 时，编译器生成代码：
1. 从胖指针中取出 vtable ptr
2. 在虚表中查找 `draw` 对应的函数指针
3. 将 data ptr 作为 `self` 传入，调用该函数

> 💡 这就是"运行时多态"的本质：**调用哪个函数，要到运行时查表才知道**。而泛型 `<T: Draw>` 在编译期已确定调用哪个具体函数，可以直接内联。

### 为什么必须用 Box<dyn Trait>？（Sized 与 DST）

Rust 中所有**直接存储在栈上**的值必须满足 `Sized` trait（编译期大小已知）。但 `dyn Draw` 是**动态大小类型（DST）**——我们不知道具体是 `Button` 还是 `TextField`，因此编译器无法确定其栈大小。

```rust,ignore
// ❌ 编译错误：dyn Draw 不满足 Sized，不能放在栈上
let c: dyn Draw = Button { label: String::from("OK") };
//        ^^^^^^^^ doesn't have a size known at compile-time

// ✅ 用引用：&dyn Draw 本质是一个胖指针，大小固定（16 字节）
let c: &dyn Draw = &Button { label: String::from("OK") };

// ✅ 用 Box：Box<dyn Draw> 在栈上存胖指针，数据在堆上
let c: Box<dyn Draw> = Box::new(Button { label: String::from("OK") });
```

| 形式 | 栈上大小 | 所有权 | 使用场景 |
|------|---------|--------|---------|
| `&dyn Trait` | 16 字节 | 借用，不拥有数据 | 函数参数、临时使用 |
| `Box<dyn Trait>` | 16 字节 | 拥有堆上数据 | 集合存储、返回值 |
| `Rc<dyn Trait>` | 16 字节 | 共享所有权 | 多所有者场景 |

### 使用 dyn Trait 实战

```rust
trait Draw { fn draw(&self); }

struct Button { label: String }
struct TextField { placeholder: String }

impl Draw for Button { fn draw(&self) { println!("[{}]", self.label); } }
impl Draw for TextField { fn draw(&self) { println!("({})", self.placeholder); } }

// 不同类型的对象放在同一个 Vec 中
let components: Vec<Box<dyn Draw>> = vec![
    Box::new(Button { label: String::from("OK") }),
    Box::new(TextField { placeholder: String::from("输入...") }),
];

for component in &components {
    component.draw();   // 运行时虚表查找：是 Button.draw 还是 TextField.draw？
}

// 函数中使用 &dyn Trait 作为参数（不获取所有权）
fn render(component: &dyn Draw) {
    component.draw();
}
render(&components[0]);

// 函数中返回 Box<dyn Trait>（转移所有权）
fn make_button(text: &str) -> Box<dyn Draw> {
    Box::new(Button { label: text.to_string() })
}
```

### 对象安全（Object Safety）— 哪些 trait 能写成 `dyn Trait`

并非所有 trait 都能写成 `dyn Trait`。例如你不能写 `Box<dyn Clone>`，因为 `Clone` 不是对象安全的。一个 trait 是**对象安全的**（即可以用于 `dyn Trait`）当且仅当：

1. `Self: Sized` 不作为 requirement（trait 本身不应要求 `Self: Sized`）
2. 所有方法必须满足以下条件之一：
   - 方法不使用 `Self`（即，没有 `self` 参数）  
   - 方法以 `&self`、`&mut self`、`Box<self>` 等引用形式接收 self（而非 `self` 或返回 `Self`）
   - 方法没有泛型参数

```rust,ignore
trait NotObjectSafe {
    fn new() -> Self;                     // ❌ 返回 Self
    fn consume(self);                     // ❌ 按值接收 self
    fn generic<T>(&self, x: T) -> T;      // ❌ 含泛型参数
}

trait ObjectSafe {
    fn draw(&self);                       // ✅ 接收 &self
    fn resize(&mut self, w: u32, h: u32); // ✅ 接收 &mut self
    fn name() -> &'static str;            // ✅ 关联函数，不用 self
}

// NotObjectSafe 不能做 trait 对象：
// let x: Box<dyn NotObjectSafe> = ...;  // ❌ 编译错误
```

> ⚠️ 如果确实需要用 `Self` 返回类型，考虑使用 `where Self: Sized` 将该方法标记为仅在已知具体类型时可用。这样的 trait 仍然是对象安全的，但该方法在 trait 对象上不可调用。

### trait 对象的限制

即使 trait 是对象安全的，使用 trait 对象时也有以下限制：

```rust,ignore
trait Animal {
    fn speak(&self);
    fn species() -> &'static str;  // 没有 self 参数
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) { println!("汪汪"); }
    fn species() -> &'static str { "犬科" }
}

let dog: Box<dyn Animal> = Box::new(Dog);
dog.speak();   // ✅ 通过虚表调用

// dog.species();   // ❌ 不能在 trait 对象上调用关联函数
// Animal::species() 无法确定是哪个实现的 species

// 关联类型也受限：
trait Container {
    type Item;
    fn get(&self) -> &Self::Item;
}
// let c: Box<dyn Container> = ...;  // ❌ Container 不是对象安全的
// 因为 get 返回 &Self::Item，Item 的具体类型不可知
```

### 向下转型（downcast）

用 `std::any::Any` 可以将 trait 对象转回具体类型：

```rust
use std::any::Any;

trait Component: Any {
    fn render(&self);
}

struct Button { label: String }
impl Component for Button {
    fn render(&self) { println!("[{}]", self.label); }
}

let component: Box<dyn Component> = Box::new(Button { label: String::from("OK") });

// 尝试向下转型
if let Some(btn) = component.downcast_ref::<Button>() {
    println!("这是一个按钮，标签: {}", btn.label);
}

// downcast 需要 trait 是 'static（不含非 'static 引用）
// 并且 trait 必须继承 Any（如 trait Component: Any）
```

> 💡 向下转型通常意味着设计可以改进——先考虑是否能用 trait 的多态来避免需要知道具体类型。

### 选择指南

| | 用 `impl Trait` / `<T: Trait>` | 用 `dyn Trait` |
|---|---|---|
| 当… | 类型在编译期可知 | 类型在运行时确定 |
| | 需要极致性能 | 需要存储多种不同类型 |
| | 类型数量有限 | 需要减少编译产物大小 |
| | | 编写库时需要隐藏实现细节 |

## 关联类型

关联类型让 trait 更简洁：

```rust
trait Iterator {
    type Item;  // 关联类型——每个实现指定一次
    fn next(&mut self) -> Option<Self::Item>;
}

// vs 如果用泛型参数：
// trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }
// 问题：同一个类型可以实现 Iterator<i32> 和 Iterator<String>
// 但这对迭代器没有意义——一个迭代器只产出一类元素
```

使用关联类型的场景：**一个类型只应实现 trait 一次**。

## 常用的标准库 Trait

| Trait | 用途 |
|-------|------|
| `Display` | 用户友好的 `{}` 格式化 |
| `Debug` | 调试 `{:?}` 格式化 |
| `Clone` | 显式深拷贝 |
| `Copy` | 赋值时复制（标记 trait） |
| `PartialEq` | `==` 和 `!=` |
| `Eq` | 等价关系（`PartialEq` + 自反性） |
| `PartialOrd` | `<`, `>`, `<=`, `>=` |
| `Ord` | 全序关系 |
| `Hash` | 可哈希（用于 HashMap key） |
| `Default` | 默认值 `Default::default()` |
| `From<T>` / `Into<T>` | 类型转换 |
| `Deref` / `DerefMut` | 解引用 |
| `Drop` | 离开作用域时自动清理 |
| `Send` | 可在线程间安全转移（自动实现） |
| `Sync` | 可在多线程间安全共享引用（自动实现） |
| `Fn` / `FnMut` / `FnOnce` | 闭包 trait |

## 练习

1. 定义一个 `Shape` trait（`fn area(&self) -> f64` 和 `fn perimeter(&self) -> f64`），为 `Circle` 和 `Rectangle` 实现
2. 用 `Box<dyn Draw>` 存储不同类型的 UI 组件，统一调用 `draw()`，并通过 `Any` 尝试向下转型
3. 尝试用 RPIT 返回一个迭代器（如 `.map().filter()`），观察返回类型的自动捕获行为
4. 定义一个对象安全的 trait 和一个非对象安全的 trait，尝试用 `dyn` 测试编译器的反馈

---

← [第 7 章：错误处理](./07-error-handling.md) | [返回目录](./README.md) | → [第 9 章：类型转换与运算符重载](./09-type-conversions.md)
