# 第 8 章：Trait 与泛型

## 学习目标

- 定义和实现 trait（类似其他语言的接口）
- 使用泛型编写复用性高的代码
- 理解 trait bound 和 where 子句
- 掌握静态分发和动态分发的区别
- 了解 **RPIT**（返回位置 impl Trait）

## Trait — 定义共享行为

Trait 定义了一组可以被共享的方法签名：

```rust
trait Summary {
    // 抽象方法——实现者必须提供
    fn summarize(&self) -> String;

    // 默认实现——实现者可以覆盖
    fn summarize_author(&self) -> String {
        String::from("(未知)")
    }
}
```

> 📘 *RPIT（Return Position impl Trait）在 Rust 2024 Edition 中自动捕获所有生命周期参数，不再需要 `+ '_` 这样的标注技巧。*

### 实现 Trait

```rust
struct NewsArticle {
    headline: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}，作者 {}", self.headline, self.author)
    }

    // 可以覆盖默认实现
    fn summarize_author(&self) -> String {
        format!("作者：{}", self.author)
    }
}
```

### Trait 作为参数

`impl Trait` 语法糖：

```rust
fn notify(item: &impl Summary) {
    println!("新闻：{}", item.summarize());
}
```

等价泛型写法：

```rust
fn notify<T: Summary>(item: &T) {
    println!("新闻：{}", item.summarize());
}
```

### 多个 Trait Bound

```rust
fn notify(item: &(impl Summary + std::fmt::Display)) { }

// 泛型版本 + where 子句
fn notify<T>(item: &T)
where
    T: Summary + std::fmt::Display,
{ }
```

## 泛型

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
let result = largest(&numbers);  // 编译器生成 largest::<i32>
```

> 💡 **零成本抽象**：Rust 的泛型通过**单态化**（monomorphization）在编译期为每种具体类型生成专门的代码。运行时没有额外开销，类似 C++ 模板但更安全。

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

let p1 = Point { x: 5, y: 10.0 };        // Point<i32, f64>
let p2 = Point { x: "hello", y: 'c' };   // Point<&str, char>
let p3 = p1.mixup(p2);                    // Point<i32, char>
```

### 为特定类型实现方法

```rust
impl Point<f64, f64> {
    // 只有 Point<f64, f64> 有这个方法
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

## RPIT — 返回 impl Trait

函数可以返回"实现了某个 trait 的类型"，而不暴露具体类型：

```rust
fn returns_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("Breaking news!"),
        author: String::from("Alice"),
    }
}
```

> 📘 *在 Rust 2024 Edition 中，`impl Trait` 返回值自动捕获所有生命周期参数，这与 `async fn` 的行为一致。如需精确控制，可使用 `use<..>` 语法（1.82 稳定，跨 Edition 可用）*：
>
> ```rust
> fn process<'a, T>(data: &'a [T]) -> impl Iterator<Item = &T> + use<'a> {
>     data.iter()
> }
> ```

## 静态分发 vs 动态分发

| | 静态分发（泛型） | 动态分发（trait 对象） |
|---|---|---|
| 写法 | `<T: Trait>` | `&dyn Trait` |
| 确定时机 | 编译期（单态化） | 运行时（虚表） |
| 性能 | 零开销 | 虚表查找 |
| 代码量 | 多份（代码膨胀） | 一份 |
| 适用 | 类型已知、性能关键 | 类型异构、运行时确定 |

### Trait 对象（动态分发）

```rust
fn draw_shape(shape: &dyn Shape) {
    println!("面积：{:.2}", shape.area());
}

let shapes: Vec<&dyn Shape> = vec![
    &Circle { radius: 2.0 },
    &Rectangle { width: 3.0, height: 4.0 },
];

for shape in &shapes {
    draw_shape(shape);
}
```

## 其他 Trait 特性

### 关联类型

```rust
trait Iterator {
    type Item;  // 关联类型——每个实现指定一次
    fn next(&mut self) -> Option<Self::Item>;
}
```

关联类型让 trait 更简洁：不需要在每个方法上写泛型参数。

### 孤儿规则

> **你只能为你的类型实现你的 trait，或为你的类型实现外部 trait，或为外部类型实现你的 trait。不能为外部类型实现外部 trait。**

这保证了 trait 实现的全局一致性，防止冲突。

## 跨语言对比

| 概念 | Rust | Java | C++ | Go |
|------|------|------|-----|-----|
| 接口 | Trait | Interface | 抽象类/Concepts | Interface |
| 泛型 | 编译期单态化 | 类型擦除 | 模板展开 | 不支持 |
| 动态分发 | `&dyn Trait` | 虚方法调用 | `virtual` | 接口调用 |
| 默认实现 | ✅ | ✅ (default) | ✅ | ❌ |

## 练习

1. 定义一个 `Shape` trait（`area()` 和 `perimeter()`），为 `Circle` 和 `Rectangle` 实现
2. 用泛型写一个函数，接收 `&[T]` 返回中位数（`T` 需实现 `PartialOrd + Copy`）
3. 尝试用 `&dyn Trait` 存储不同类型的 trait 对象到 Vec 中

---

← [第 7 章：错误处理](./07-error-handling.md) | [返回目录](./README.md) | → [第 9 章：生命周期](./09-lifetime.md)
