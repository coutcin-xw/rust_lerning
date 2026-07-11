# 第 6 章：模式匹配

## 学习目标

- 掌握 `match` 的穷尽性检查和值绑定
- 使用多种解构模式（元组、结构体、枚举）
- 用好匹配守卫和 `@` 绑定
- 善用 `if let` / `while let` 简化单一模式匹配
- 使用 **let chains** 简化多层嵌套的条件模式

## 概念层：模式匹配不是 switch

Rust 的 `match` 与 C/Java 的 `switch` 有本质区别：它是**穷尽的**（缺失分支即编译错误）、是**表达式**（可返回值）、**无 fallthrough**（分支相互独立）。与枚举结合后，编译器会验证所有变体都已被处理——这意味着"忘记 case"和"空引用解构"整类 bug 在编译期就消失了。

本章从基础 `match` 到嵌套解构、守卫、let chains 逐步展开，贯穿始终的主线只有一个：**编译器替你证明穷尽性**。

> 💡 `match` 不仅是控制流——它是 Rust 中证明"所有情况都被处理了"的编译器验证工具。

## 语法层：模式匹配的语法图谱

Rust 的模式系统由几种语法形式构成，先看清全貌：

```rust
// 1. match：穷尽匹配，每分支 = 模式 => 表达式
match value {
    Pattern1 => expr1,
    Pattern2 if guard => expr2,    // 守卫条件
    _ => default_expr,             // 通配符
}

// 2. 模式类型（可以嵌套组合）
//   字面量模式:    1 | 2                  // 或模式
//   变量绑定:      x                      // 匹配任何值，绑定
//   解构:          Some(x)                // 提取 Option 内部值
//                  Point { x, y }         // 提取结构体字段
//   范围:          1..=5                  // 匹配 1 到 5
//   守卫:          x if x > 0            // 附加条件

// 3. 其他使用模式的地方
if let Pattern = expr { }          // 单分支 match
while let Pattern = expr { }       // 循环 + 模式
let (a, b) = pair;                 // let 解构
fn foo((x, y): (i32, i32)) { }    // 函数参数解构
for (i, v) in vec.iter().enumerate() { }  // for 循环
```

> 💡 Rust 中模式不是一个独立特性，而是**散布在多种语法位置中的统一机制**：`match`、`if let`、`let`、函数参数、`for` 循环——同一个模式语法处处可用。

## match —— Rust 最强大的控制流

`match` 同时做两件事：**分支选择** + **值提取**。

### 基本语法

```rust
enum Coin { Penny, Nickel, Dime, Quarter }

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny   => 1,
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter => 25,
    }
}
```

`match` 的关键特性：

| 特性 | 说明 |
|------|------|
| **穷尽性** | 必须覆盖所有可能的值，遗漏 → 编译错误 |
| **表达式** | `match` 可以作为右值，各分支返回相同类型 |
| **无穿透** | 每个分支独立，不会 fall through 到下一个 |
| **绑定** | 匹配到的数据可以绑定到变量 |

### 穷尽性——编译器为你查漏

```rust
let x = Some(5);
match x {
    Some(n) => println!("{}", n),
    // 编译错误：遗漏了 None！
}
```

如果你添加了新的枚举变体，编译器会自动告诉你所有需要更新的 `match`。这是 Rust 重构安全性的一部分。

### 通配符 `_`

```rust
match dice_roll {
    3 => add_hat(),
    7 => remove_hat(),
    _ => reroll(),  // 匹配所有其他值
}
```

> ⚠️ `_` 不会绑定值。如果需要用到值，用变量名：`other => process(other)`。

## 模式绑定——match 真正的威力

模式不只是比较——它把匹配到的数据**提取出来**：

```rust
enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
    ChangeColor(u8, u8, u8),
}

fn process(msg: Message) {
    match msg {
        Message::Quit                       => println!("退出"),
        Message::Write(text)                => println!("写入: {}", text),     // text 绑定
        Message::Move { x, y }              => println!("到 ({}, {})", x, y), // x, y 绑定
        Message::ChangeColor(r, g, b)       => println!("#{:02x}{:02x}{:02x}", r, g, b),
    }
}
```

## 解构（Destructuring）

### 元组解构

```rust
let pair = (0, 5);
match pair {
    (0, y) => println!("在 y 轴上，y={}", y),
    (x, 0) => println!("在 x 轴上，x={}", x),
    _      => println!("不在轴上"),
}
```

### 结构体解构

```rust
struct Point { x: i32, y: i32 }

let p = Point { x: 0, y: 7 };

match p {
    Point { x, y: 0 } => println!("在 x 轴上，x={}", x),
    Point { x: 0, y } => println!("在 y 轴上，y={}", y),
    Point { x, y }    => println!("不在轴上: ({}, {})", x, y),
}
```

### 嵌套解构

```rust
enum Color { Rgb(u8, u8, u8), Hsv(u16, u8, u8) }

struct Item { name: String, color: Color }

fn describe(item: &Item) -> String {
    match &item.color {
        Color::Rgb(255, 0, 0) => format!("{} 是纯红色", item.name),
        Color::Rgb(0, g, 0)   => format!("{} 的绿色分量是 {}", item.name, g),
        Color::Hsv(h, s, v)   => format!("{} 的 HSV: ({}, {}, {})", item.name, h, s, v),
        _                     => format!("{} 的其他颜色", item.name),
    }
}
```

### 部分解构 `..`

```rust
let p = Point { x: 3, y: 4, z: 5 };
match p {
    Point { x, .. } => println!("x = {}", x),  // 只关心 x
}

let numbers = (1, 2, 3, 4, 5);
match numbers {
    (first, .., last) => println!("首={}, 尾={}", first, last),
}
```

## 匹配中的额外条件

### 多值匹配 `|`

```rust
match x {
    1 | 2 | 3 => println!("小数字"),
    _         => println!("其他"),
}
```

### 范围匹配

```rust
match x {
    1..=5   => println!("1 到 5"),
    6..=10  => println!("6 到 10"),
    _       => println!("其他"),
}

match c {
    'a'..='j' => println!("前半字母"),
    'k'..='z' => println!("后半字母"),
    _          => println!("不是小写字母"),
}
```

### 匹配守卫——`if` 条件

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("小于 5: {}", x),
    Some(x)          => println!("大于等于 5: {}", x),
    None             => println!("什么都没有"),
}
```

排序守护（结合 `|`）：

```rust
match num {
    Some(1) | Some(2) if some_condition => println!("条件匹配"),
    Some(1) | Some(2)                   => println!("普通匹配"),
    _                                   => (),
}
// ⚠️ if 只作用于紧邻的模式：Some(2)，不是 Some(1)
```

### `@` 绑定——既匹配又绑定

```rust
match 3 {
    n @ 1..=5 => println!("n={} 在 1-5 之间", n),  // n 绑定到匹配的值
    _         => println!("其他"),
}

// 解构中也可以使用
let opt = Some(Point { x: 1, y: 2 });
match opt {
    Some(p @ Point { x: 0..=5, y }) => println!("小坐标: {:?}", p),
    Some(p)                          => println!("其他坐标: {:?}", p),
    None                             => println!("没有坐标"),
}
```

## if let —— 单分支 match 的简写

```rust
// match 写法（冗长）
match optional_value {
    Some(max) => println!("max = {}", max),
    _         => (),  // 为了满足穷尽性必须写
}

// if let 写法（简洁）
if let Some(max) = optional_value {
    println!("max = {}", max);
}
```

`if let` 本质是 `match` 的语法糖——只关心一个模式，忽略其他。

### if let 带 else

```rust
if let Some(value) = get_optional() {
    println!("收到: {}", value);
} else {
    println!("没有收到值");
}
```

## while let —— 在循环中匹配

```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("{}", top);  // 3, 2, 1
}
// 当 pop() 返回 None 时循环自动结束
```

实际应用——处理消息队列：

```rust
while let Some(msg) = receiver.recv().ok() {
    process(msg);
}
```

## let chains —— 链式模式匹配（2024 Edition 新特性）

> 📘 *let chains 是 **Rust 2024 Edition 的独占特性**（Rust 1.88.0 稳定），允许用 `&&` 链式连接多个条件模式匹配。*

### 问题：嵌套的 if let 地狱

```rust
// 旧写法：深层嵌套
if let Some(user) = request.user {
    if user.is_active() {
        if let Some(project) = user.current_project() {
            if let Some(task) = project.next_task() {
                assign_task(task, user);
            }
        }
    }
}
```

### 解决：let chains

```rust
// 2024 Edition：扁平化的链式条件
if let Some(user) = request.user
    && user.is_active()
    && let Some(project) = user.current_project()
    && let Some(task) = project.next_task()
{
    assign_task(task, user);
}
```

### let chains 的更多用法

```rust
// 解析函数签名
if let Some((name, rest)) = expr.split_once("(")
    && !name.is_empty()
    && let Some((args, "")) = rest.rsplit_once(")")
{
    println!("函数: {}, 参数: {}", name, args);
}

// 结合布尔条件和模式匹配
if config.is_enabled
    && let Some(cache) = cache_manager.get("key")
{
    return cache;
}

// 在循环中
while let Some(line) = reader.next_line()
    && !line.is_empty()
{
    process(line);
}
```

### 工作原理

`let chains` 依赖于 2024 Edition 的 `if let` 临时值作用域规则。在 2024 Edition 中，`if let` 创建的临时值在进入 `else` 之前就被释放，这使得链式 `&&` 的条件可以安全和高效地求值。

## for 循环和 let 语句中的模式

```rust
// for 循环解构
let pairs = vec![(1, 2), (3, 4), (5, 6)];
for (x, y) in pairs {
    println!("({}, {})", x, y);
}

// 带索引的解构
for (i, (x, y)) in pairs.iter().enumerate() {
    println!("pairs[{}] = ({}, {})", i, x, y);
}
```

```rust
// let 语句本身也是模式匹配！
let (x, y, z) = (1, 2, 3);       // 解构元组
let Point { x, y } = point;      // 解构结构体
let Some(value) = optional;      // 如果 optional 是 None 会 panic！

// 函数参数中的模式
fn print_coords(&(x, y): &(i32, i32)) {
    println!("({}, {})", x, y);
}
```

## 模式匹配实战：表达式求值器

```rust
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

impl Expr {
    fn eval(&self) -> i32 {
        match self {
            Expr::Num(n)        => *n,
            Expr::Add(l, r)     => l.eval() + r.eval(),
            Expr::Mul(l, r)     => l.eval() * r.eval(),
            Expr::Neg(e)        => -e.eval(),
        }
    }
}

// Expr::Add(
//     Box::new(Expr::Num(3)),
//     Box::new(Expr::Mul(
//         Box::new(Expr::Num(4)),
//         Box::new(Expr::Num(5))
//     ))
// ).eval() = 23
```

## 常见陷阱

> ⚠️ **在 `if let` 中同时用 `|` 和 `if` 守卫。** `Some(1) | Some(2) if cond` 中，`if cond` 只作用于 `Some(2)`。用括号明确意图或拆成两个分支。

> ⚠️ **用 `_` 绑定代替变量名导致值丢失。** 如果你需要用到匹配的值，不要用 `_`——用一个有意义的变量名。

> ⚠️ **match 分支返回不同类型。** 所有分支必须是同一类型，否则编译器报错。如果确实需要不同处理，可以在分支内完成（`println!` / `return` / `panic!`）。

## 练习

1. 扩展上面的 `Expr` 枚举：放入 `Sub`（减法）和 `Div`（除法）变体，更新 `eval` 方法
2. 用 **let chains** 改写以下嵌套代码（需要在 Rust ≥1.88.0 + 2024 Edition 下运行）：
   ```rust
   if let Some(config) = load_config() {
       if config.debug_mode {
           if let Some(log_file) = config.log_file {
               println!("日志文件: {}", log_file);
           }
       }
   }
   ```
3. 写一个函数，用 `match` 处理 `Result<Option<i32>, String>` 的所有四种组合（Ok(Some), Ok(None), Err("msg1"), Err("msg2")）
4. 用 `while let` 实现一个读取命令行输入直到输入 `"quit"` 的循环

---

← [第 5 章：结构体与枚举](./05-struct-enum.md) | [返回目录](./README.md) | → [第 7 章：错误处理](./07-error-handling.md)
