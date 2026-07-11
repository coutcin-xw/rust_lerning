# 第 5 章：结构体与枚举

## 学习目标

- 定义三种结构体：具名字段、元组、单元结构体
- 为结构体实现方法（`impl`）
- 理解 Rust 枚举的表达力——每个变体可携带不同类型的数据
- 使用 `Option<T>` 替代 null
- 体会"组合优于继承"的 Rust 风格

## 概念层：Rust 的类型抽象

> 💡 Rust 的类型系统核心思想不是"对象"，而是"数据 + 行为"的分离与"总和类型"的表达力。

与 Java/C++ 等基于类的 OOP 不同，Rust 将**数据定义**（`struct`）与**行为实现**（`impl`）解耦，摒弃继承，仅用 trait 和组合复用代码。Rust 的枚举远不止整数标签：每个变体可携带不同类型的数据，构成**代数数据类型**（ADT），配合 `match` 穷尽匹配，一次性替代传统枚举、访问者模式和带标签联合体。`Option<T>` 便是最好例证——它只是普通枚举，并非语言内建机制，却彻底消除 null（"十亿美元错误"），由编译器强制检查值的存在性。

## 语法层：struct / enum / impl 三件套

本章所有代码由三种语法构成，先建立全景印象：

```rust
// 1. struct：定义数据结构（三种形式）
struct Point(i32, i32);                      // 元组结构体
struct User { name: String, age: u8 }        // 具名字段结构体
struct Unit;                                  // 单元结构体（无字段）

// 2. impl：为类型实现方法
impl User {
    fn new(name: &str, age: u8) -> Self {    // 关联函数（没有 self）
        User { name: name.into(), age }
    }
    fn greet(&self) {                         // 方法（&self = 借用实例）
        println!("我是 {}, {} 岁", self.name, self.age);
    }
}

// 3. enum：定义"可能是多种情况之一"的类型
enum Message {
    Quit,                              // 无数据变体
    Move { x: i32, y: i32 },          // 携带匿名结构体
    Write(String),                     // 携带单个值
    ChangeColor(u8, u8, u8),          // 携带多个值
}
```

> 💡 `struct` 定义数据形状，`impl` 定义行为，`enum` 定义可能的情况。三者独立但组合使用——不设继承树，只有组合关系。

## 结构体——自定义数据形状

结构体把相关数据组合在一起。Rust 有三种结构体形式。

### 具名字段结构体

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 创建实例
let user = User {
    email: String::from("alice@example.com"),
    username: String::from("alice"),
    active: true,
    sign_in_count: 1,
};

// 访问字段
println!("用户名: {}", user.username);

// 修改（需要整个实例是 mut）
let mut user = user;
user.email = String::from("new@example.com");
// 注意：Rust 没有"字段级别的 mut"——整个 struct 可变或不可变
```

### 结构体更新语法

从已有实例创建新实例：

```rust
let user2 = User {
    email: String::from("bob@example.com"),
    username: String::from("bob"),
    ..user  // 其余字段从 user 取
};
```

> ⚠️ `..user` 会**移动**未显式赋值的字段。这里 `active` 和 `sign_in_count`（都是 `Copy` 的）被复制，但如果 `user` 有 `String` 字段没被覆盖，那个字段的所有权就移动到 `user2` 了，`user` 中对应的字段失效。

### 元组结构体

有类型名、无字段名：

```rust
struct Color(u8, u8, u8);
struct Point(i32, i32);
struct Meters(f64);

let black = Color(0, 0, 0);
let origin = Point(0, 0);

// 按索引访问
println!("R={}, G={}, B={}", black.0, black.1, black.2);

// 元组结构体是不同的类型——即使内部类型相同
let p = Point(3, 4);
let m = Meters(5.0);
// let p2: Point = m;  // 编译错误！类型不匹配
// 但可以解构
let Point(x, y) = p;
```

元组结构体的典型用法是**类型安全的新类型包装**（newtype pattern）：

```rust
struct UserId(u64);
struct PostId(u64);

fn get_user(id: UserId) -> User { /* ... */ }
fn get_post(id: PostId) -> Post { /* ... */ }

let user_id = UserId(42);
let post_id = PostId(100);

get_user(user_id);   // ✅
// get_user(post_id);  // ❌ 编译错误！PostId 不是 UserId
```

### 单元结构体

没有字段，用作标记或占位：

```rust
struct AlwaysEqual;
struct ReadPermission;
struct WritePermission;

// 常在泛型和 trait 中使用
impl Permission for ReadPermission { }
impl Permission for WritePermission { }
```

### 调试输出

```rust
#[derive(Debug)]   // 自动实现 Debug trait
struct Rectangle {
    width: u32,
    height: u32,
}

let rect = Rectangle { width: 30, height: 50 };
println!("{:?}", rect);    // Rectangle { width: 30, height: 50 }
println!("{:#?}", rect);   // 带缩进的格式化

// 更好的调试方式：dbg! 宏
let scale = 2;
let rect = Rectangle {
    width: dbg!(30 * scale),  // 打印表达式和结果
    height: 50,
};
dbg!(&rect);  // 打印文件名、行号、值
```

## 方法——给结构体加上行为

```rust
struct Rectangle { width: u32, height: u32 }

impl Rectangle {
    // 方法：第一个参数是 &self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 参数的选择：
    // &self       — 只读借用（最常用）
    // &mut self   — 可变借用
    // self        — 获取所有权（消费实例）

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数（静态方法）：没有 self 参数
    fn square(size: u32) -> Self {  // Self = Rectangle
        Self { width: size, height: size }
    }
}

let rect = Rectangle { width: 30, height: 50 };
println!("面积: {}", rect.area());         // 方法调用

let square = Rectangle::square(20);        // :: 关联函数调用
```

> 💡 Rust 没有 `->` 运算符。你写 `rect.area()`，编译器自动判断需要 `&self`、`&mut self` 还是 `self`，然后自动添加引用/解引用。这叫**自动引用和解引用**（automatic referencing and dereferencing）。

### 多个 impl 块

```rust
impl Rectangle {
    fn area(&self) -> u32 { self.width * self.height }
}

impl Rectangle {
    fn perimeter(&self) -> u32 { 2 * (self.width + self.height) }
}
```

分开写不会影响性能，编译器会合并。这个特性对于有条件地实现方法很有用（见第 8 章泛型）。

## 枚举——表达"多种可能性"

Rust 的枚举比大多数语言强大得多——每个变体可以携带不同的数据：

```rust
enum Message {
    Quit,                              // 无数据（类似 C 的枚举）
    Move { x: i32, y: i32 },          // 匿名结构体
    Write(String),                     // 单个 String
    ChangeColor(u8, u8, u8),          // 三个 u8
}

// 枚举也可以有方法
impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("退出程序"),
            Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
            Message::Write(text) => println!("写入: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("颜色: #{:02x}{:02x}{:02x}", r, g, b)
            }
        }
    }
}
```

### 枚举内存布局

```rust
enum Status {
    Success,           // 不需要额外数据
    ErrorCode(u32),    // 需要 4 字节存储错误码
    Message(String),   // 需要 24 字节（String 的三个字段）
}
// Status 的大小 = max(各变体大小) + 判别值（discriminant）
// ≈ 24 + 8 = 32 字节
```

## Option\<T\> — 十亿美元错误的解决方案

Rust **没有 null**。

```rust
enum Option<T> {
    None,     // 表示"没有值"
    Some(T),  // 表示"有一个 T 类型的值"
}
```

为什么没有 null？其发明者 Tony Hoare 称之为"十亿美元的错误"——null 导致的崩溃和安全漏洞的损失。Rust 的解决方案：让"可能为空"成为**类型的一部分**。

```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;

// let sum = x + 1;  // ❌ 编译错误！Option<i32> 不是 i32
// 编译器强制你处理 None 的情况
```

处理 `Option` 的几种方式：

```rust
let opt = Some(5);

// 方式 1：模式匹配（最安全）
match opt {
    Some(n) => println!("值是 {}", n),
    None => println!("没有值"),
}

// 方式 2：if let（简洁）
if let Some(n) = opt {
    println!("值是 {}", n);
}

// 方式 3：提供默认值
let n = opt.unwrap_or(0);         // None 时返回 0
let n = opt.unwrap_or_default();  // None 时返回类型的默认值（0, "", false...）

// 方式 4：转换
let doubled = opt.map(|n| n * 2);          // Some(10) 或 None
let filtered = opt.filter(|n| n > &3);     // Some(5) 或 None

// 方式 5：直接获取（可能 panic）
let n = opt.unwrap();       // None 会 panic
let n = opt.expect("必须有值");  // panic 时附带消息
```

### Option 的实用模式

```rust
// 链式操作
fn id_from_name(name: &str) -> Option<u64> {
    database.find_user(name)?        // ? 在这里直接返回 None
        .get_settings().ok()?        // Result → Option
        .get("default_project")
        .and_then(|proj| proj.parse().ok())
}
```

## 枚举 vs 继承——Rust 的表达方式

其他语言用类继承来表达不同类型之间的共性。Rust 用**枚举 + match**：

```
Java/C++ 做法（继承）：
              Shape (abstract)
             /                \
    Circle                   Rectangle
    - radius                 - width
    + area()                 - height
                             + area()

Rust 做法（枚举 + match）：
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
        }
    }
}
```

枚举方案的优势：
- 所有变体在一处定义——一眼看清有哪些子类型
- 添加新操作（如 `perimeter()`）只需加方法，不需改每个子类
- 没有虚方法调用的运行时开销（match 在编译时确定跳转表）

## 常用标准库枚举

| 枚举 | 变体 | 用途 |
|------|------|------|
| `Option<T>` | `Some(T)`, `None` | 可能缺失的值 |
| `Result<T,E>` | `Ok(T)`, `Err(E)` | 可能失败的操作 |
| `Ordering` | `Less`, `Equal`, `Greater` | 比较结果 |
| `Cow<'a, T>` | `Borrowed(&'a T)`, `Owned(T)` | 写时克隆 |

## 跨语言对比

| 概念 | Rust | C/C++ | Java | Python |
|------|------|-------|------|--------|
| 结构体 | `struct` | `struct` | `class` / `record` | `dataclass` |
| 方法 | `impl Struct { fn m(&self) }` | 成员函数 | 实例方法 | `self` 参数 |
| 枚举 | 可携带数据的变体 | 整数常量 | 增强 enum | `Enum` (class) |
| null | `Option<T>` | NULL/nullptr | null | None |
| 继承 | 无（Trait 组合） | 支持 | 支持 | 支持 |

## 常见陷阱

> ⚠️ **结构体部分移动。** `..user` 会移动非 Copy 字段。如果你之后还需要 `user`，先用 clone 或重新设计。

> ⚠️ **方法参数选择错误。** `fn m(self)` 会消费实例。除非你确实要"用完即弃"，否则用 `&self`。

> ⚠️ **在不需要所有权的地方用了 `String` 而非 `&str`。** 结构体字段如果需要拥有数据用 `String`，函数参数优先用 `&str`。

## 练习

1. 定义一个 `Book` 结构体（书名、作者、ISBN、是否借出）。实现方法：借书、还书、获取摘要信息
2. 定义一个 `Shape` 枚举（Circle, Rectangle, Triangle），实现 `area()` 方法。为每种形状至少写一个测试
3. 实现一个 `Temperature` 枚举（Celsius(f64), Fahrenheit(f64), Kelvin(f64)），实现转换到 Celsius 的方法
4. 用 newtype 模式包装 `u64` 创建 `Email(String)` 结构体，实现一个验证方法确保包含 `@`
5. 尝试写出一个会触发"部分移动"的场景，理解编译错误

---

← [第 4 章：借用与引用](./04-borrowing.md) | [返回目录](./README.md) | → [第 6 章：模式匹配](./06-pattern-match.md)
