# 第 8 章：Trait 与泛型

## 学习目标

- 定义并实现 trait（Rust 的接口抽象）
- 用泛型编写类型参数化的代码
- 理解 trait bound 如何约束泛型
- 掌握 `impl Trait` 在参数和返回值中的用法
- 理解静态分发（单态化）和动态分发（`dyn Trait`）的取舍
- 使用 RPIT 和 `use<..>` 精确控制生命周期捕获

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

### 使用 dyn Trait

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
    component.draw();
}
```

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
2. 写一个泛型函数，计算任意 `&[T]`（`T: PartialOrd`）的中位数
3. 用 `Box<dyn Draw>` 存储不同类型的 UI 组件，统一调用 `draw()`
4. 尝试用 RPIT 返回一个迭代器（如 `.map().filter()`），观察返回类型的自动捕获行为

---

← [第 7 章：错误处理](./07-error-handling.md) | [返回目录](./README.md) | → [第 9 章：生命周期](./09-lifetime.md)
