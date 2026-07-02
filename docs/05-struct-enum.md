# 第 5 章：结构体与枚举

## 学习目标

- 定义和使用结构体（struct）
- 为结构体添加方法（impl）
- 使用枚举（enum）表达多种可能
- 理解 `Option<T>` 及其设计意图

## 结构体

### 定义和实例化

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

let user = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    active: true,
};

// 访问字段
println!("{}", user.username);

// 修改字段（需要 mut）
let mut user = user;
user.email = String::from("new@example.com");
```

### 结构体更新语法

从已有实例创建新实例：

```rust
let user2 = User {
    email: String::from("bob@example.com"),
    ..user  // 其余字段从 user 复制
};
// 注意：user.username 是 String（非 Copy），所有权转移到 user2
// 之后不能再使用 user.username
```

### 元组结构体

有类型名但没有字段名：

```rust
struct Color(u8, u8, u8);
struct Point(i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0);

println!("R={}, G={}, B={}", black.0, black.1, black.2);
```

### 单元结构体

没有字段的结构体，常用作标记类型：

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;
```

### 调试输出

用 `#[derive(Debug)]` 让结构体可以打印：

```rust
#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }

let rect = Rectangle { width: 30, height: 50 };
println!("{:?}", rect);   // 紧凑格式
println!("{:#?}", rect);  // 美化格式
```

## 方法（impl）

```rust
struct Rectangle { width: u32, height: u32 }

impl Rectangle {
    // 方法：第一个参数是 &self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数（静态方法）：没有 self
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

let rect = Rectangle { width: 30, height: 50 };
println!("面积：{}", rect.area());

let square = Rectangle::square(20);  // :: 调用关联函数
```

> 💡 Rust 没有 `->` 运算符。当你写 `rect.area()` 时，编译器自动决定使用 `&self`、`&mut self` 还是 `self`（自动引用/解引用）。

## 枚举

Rust 的枚举比大多数语言的枚举强大得多——每个变体可以携带不同类型的数据：

```rust
enum Message {
    Quit,                       // 无数据
    Move { x: i32, y: i32 },   // 匿名结构体
    Write(String),              // 单个 String
    ChangeColor(u8, u8, u8),   // 三个 u8
}

// 为枚举实现方法
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
            Message::Write(s) => println!("写入: {}", s),
            Message::ChangeColor(r, g, b) => println!("颜色: #{:02x}{:02x}{:02x}", r, g, b),
        }
    }
}
```

## Option\<T\> — 替代 null

Rust **没有 null**。取而代之的是标准库中的 `Option<T>`：

```rust
enum Option<T> {
    None,       // 表示"没有值"
    Some(T),    // 表示"有值"，值类型为 T
}
```

`Option<T>` 和 `<T>` 是不同的类型，编译器强制你必须处理 `None` 的情况：

```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;

// let sum = x + 1;  // 编译错误！i32 和 Option<i32> 是不同的类型
```

处理 `Option` 的常用方法：

```rust
let x = Some(5);

// 提供默认值
let v = x.unwrap_or(0);          // 如果是 None 则返回 0

// 转换内部值
let doubled = x.map(|n| n * 2);  // Some(10)

// 链式处理
let result = x.and_then(|n| {
    if n > 0 { Some(n * 2) } else { None }
});
```

> ⚠️ `unwrap()` 在 `None` 上调用会 **panic**。生产代码中优先使用 `unwrap_or`、`map`、`and_then` 或模式匹配。

## 枚举的设计优势

在许多语言中，表达"可能的值"需要类和继承。Rust 的枚举天然适合表达不同类型的状态：

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
        }
    }
}
```

## 跨语言对比

| 概念 | Rust | C/C++ | Java | Python |
|------|------|-------|------|--------|
| 结构体 | `struct` | `struct` | `class` | `class` / `dataclass` |
| 方法 | `impl Struct { fn m(&self) }` | 成员函数 | 实例方法 | `self` 参数 |
| 枚举 | 可携带数据的 enum | 整数常量 | `enum`（有限增强） | `Enum`（类） |
| null | `Option<T>` | NULL/nullptr | null | None |
| 继承 | 无（用 trait 组合） | 支持 | 支持 | 支持 |

> 💡 Rust 的核心设计理念是**组合优于继承**。你不继承结构体，而是通过 trait 来共享行为。这避免了继承层次过深带来的问题。

## 练习

1. 定义一个 `Book` 结构体，包含书名、作者、页数，实现一个方法返回摘要
2. 定义一个 `TrafficLight` 枚举（Red/Yellow/Green），实现 `next()` 方法切换到下一个状态
3. 写一个函数接收 `Option<i32>`，如果是 `Some` 就加 1，`None` 返回 `None`

---

← [第 4 章：借用与引用](./04-borrowing.md) | [返回目录](./README.md) | → [第 6 章：模式匹配](./06-pattern-match.md)
