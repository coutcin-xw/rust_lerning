# 第 6 章：模式匹配

## 学习目标

- 掌握 `match` 表达式的各种用法
- 熟练使用解构和匹配守卫
- 理解 `if let` / `while let` 的使用场景
- 学会使用 **let chains**（2024 Edition 新语法）

## match 基础

`match` 是 Rust 中最强大的控制流结构。它比较一个值和一系列模式，执行第一个匹配到的分支：

```rust
enum Coin {
    Penny, Nickel, Dime, Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

关键特点：
- **穷尽性**：编译器检查所有可能值是否被覆盖，遗漏会导致编译错误
- **表达式**：`match` 可以作为返回值
- **无穿透（No fall-through）**：每个分支不会自动进入下一个

### 绑定值

模式可以绑定匹配到的数据：

```rust
enum Message { Quit, Write(String), Move { x: i32, y: i32 } }

match msg {
    Message::Quit => println!("退出"),
    Message::Write(text) => println!("写入: {}", text),  // text 绑定了 String
    Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
}
```

### 多条件匹配

```rust
let x = 2;

match x {
    1 | 2 => println!("一或二"),           // | 表示"或"
    3..=7 => println!("三到七之间"),       // 范围匹配
    _ => println!("其他"),                 // _ 匹配所有其他值
}
```

## 解构（Destructuring）

### 解构元组

```rust
let pair = (0, 5);
match pair {
    (0, y) => println!("x=0, y={}", y),
    (x, 0) => println!("x={}, y=0", x),
    _ => println!("都非零"),
}
```

### 解构结构体

```rust
struct Point { x: i32, y: i32 }

let p = Point { x: 0, y: 7 };

match p {
    Point { x, y: 0 } => println!("x={}, 在x轴上", x),
    Point { x: 0, y } => println!("y={}, 在y轴上", y),
    Point { x, y } => println!("不在轴上: ({}, {})", x, y),
}
```

### 部分解构

```rust
match p {
    Point { x, .. } => println!("x = {}", x),  // 只关心 x
}
```

## 匹配守卫（Match Guards）

在模式后面加 `if` 条件进行额外过滤：

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("小于5: {}", x),
    Some(x) => println!("大于等于5: {}", x),
    None => println!("没有值"),
}
```

### @ 绑定

`@` 同时测试模式并绑定值：

```rust
match 3 {
    n @ 1..=5 => println!("n={} 在 1-5 之间", n),
    _ => println!("其他"),
}
```

## if let / while let

当只关心一种匹配情况时，`if let` 比 `match` 更简洁：

```rust
let config_max = Some(3u8);

// match 写法（冗长）
match config_max {
    Some(max) => println!("max = {}", max),
    _ => (),
}

// if let 写法（简洁）
if let Some(max) = config_max {
    println!("max = {}", max);
}
```

`while let` 在循环中匹配：

```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("{}", top);
}
// 输出：3 2 1
```

## let chains（2024 Edition 新语法）

> 📘 `let` chains 是 Rust 2024 Edition 的**独占特性**（Rust 1.88.0 稳定），允许用 `&&` 链式连接多个 `let` 模式匹配。

```rust
// 检查字符串是否符合 "函数名(参数)" 的格式
if let Some((fn_name, after_name)) = s.split_once("(")
    && !fn_name.is_empty()
    && let Some((args_str, "")) = after_name.rsplit_once(")")
{
    println!("函数名: {}, 参数: {}", fn_name, args_str);
}
```

相比嵌套 `if let`，let chains 更易读：

```rust
// 旧写法（嵌套）
if let Some((fn_name, after_name)) = s.split_once("(") {
    if !fn_name.is_empty() {
        if let Some((args_str, "")) = after_name.rsplit_once(")") {
            // ...
        }
    }
}
```

## for 循环中的模式

```rust
let v = vec![(1, 2), (3, 4), (5, 6)];

for (x, y) in &v {       // 直接解构元组
    println!("x={}, y={}", x, y);
}

for (i, (x, y)) in v.iter().enumerate() {  // 带索引的解构
    println!("v[{}] = ({}, {})", i, x, y);
}
```

## 穷尽性检查

编译器确保 `match` 覆盖所有可能：

```rust
let x = Some(5);
match x {
    Some(n) => println!("{}", n),
    // 编译错误！遗漏了 None 分支
}
```

这是 Rust 防止 bug 的重要机制——如果你添加了新的枚举变体，编译器会帮你找到所有需要更新的 `match`。

## 跨语言对比

| 概念 | Rust | C/C++ | Java | Python |
|------|------|-------|------|--------|
| match/switch | 穷尽性检查、表达式 | 穿透、整数限制 | 穿透、支持 String | `match` (3.10+) |
| 解构 | 深层解构 | 无（C++17 结构化绑定有限） | 无 | 支持 |
| if let | `if let Pat = expr` | 无 | 无 | `match` 简化 |
| let chains | `if let A && let B` | 无 | 无 | 无 |

## 常见陷阱

> ⚠️ **匹配守卫中的 `|` 优先级。** `Some(1) | Some(2) if condition` 中，`if condition` 只作用于 `Some(2)`，不对 `Some(1)` 生效。

> ⚠️ **忘记 match 穷尽性。** 使用 `_` 通配符可以覆盖所有剩余情况，但新增枚举变体时编译器不会提示你更新——需要权衡便利性和完整性。

## 练习

1. 定义一个 `Expr` 枚举（整数、加法、乘法），写一个 `eval` 函数用 match 递归求值
2. 用 `if let` 改写一段只关心单个 match 分支的代码
3. 用 **let chains** 写一个函数，检查字符串是否符合 `"名字:年龄"` 格式，且年龄在 0-150 之间

---

← [第 5 章：结构体与枚举](./05-struct-enum.md) | [返回目录](./README.md) | → [第 7 章：错误处理](./07-error-handling.md)
