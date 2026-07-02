# 第 2 章：基础语法

## 学习目标

- 理解 Rust 变量默认不可变的设计理念
- 掌握基本数据类型和复合类型
- 理解 Rust 中表达式与语句的区别
- 熟练使用三种循环形式

## 变量与可变性

### 默认不可变

Rust 的变量**默认不可变**。这是安全哲学的核心——减少意外修改，让代码更容易推理。

```rust
let x = 5;       // 不可变变量
// x = 6;        // 编译错误！不能修改不可变变量
```

需要修改变量时，显式加 `mut`：

```rust
let mut y = 10;  // mut = mutable
y = 20;          // OK
```

### 常量

常量与不可变变量的区别：

| | `const` | `let`（不可变） |
|---|---|---|
| 值确定时机 | 编译时 | 运行时也可以 |
| 类型标注 | 必须显式标注 | 可自动推断 |
| 作用域 | 全局有效 | 当前作用域 |

```rust
const MAX_SIZE: u32 = 100;  // 常量，全大写+下划线命名
```

### 变量遮蔽（Shadowing）

可以用同名变量覆盖旧变量，类型也可以改变：

```rust
let x = 5;
let x = x + 1;       // x = 6
let x = "现在是字符串";  // x 的类型变成了 &str
```

这比 `mut` 更灵活，因为你可以在变换后保持不可变性。

## 基本数据类型

### 标量类型

| 类别 | 类型 | 说明 |
|------|------|------|
| 整数 | `i8` `i16` `i32` `i64` `i128` `isize` | 有符号，默认 `i32` |
| 整数 | `u8` `u16` `u32` `u64` `u128` `usize` | 无符号 |
| 浮点 | `f32` `f64` | 默认 `f64` |
| 布尔 | `bool` | `true` / `false` |
| 字符 | `char` | Unicode 标量值，4 字节 |

```rust
let i: i32 = 42;
let big = 100_000_000_u128;  // 下划线分隔 + 类型后缀
let f = 3.14;                 // 默认 f64
let f2 = 2.0_f32;             // f32 后缀
let b: bool = true;
let c: char = '中';
let emoji = '😀';             // char 支持 emoji
```

### 复合类型

**元组（Tuple）**——固定长度，类型可以不同：

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// 解构
let (a, b, c) = tup;

// 索引访问
let first = tup.0;  // 500
let second = tup.1; // 6.4
```

**数组（Array）**——固定长度，元素类型相同，栈上分配：

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let same = [3; 5];  // [3, 3, 3, 3, 3]
```

> ⚠️ 数组越界访问会导致运行时 panic（程序崩溃），这是 Rust 的安全保护机制。

数组 vs Vector 的选择：知道长度且不变 → 数组；需要动态增减 → Vector。

## 函数

### 定义与返回值

```rust
fn greet(name: &str) -> String {
    let greeting = "Hello";             // 语句（以分号结尾）
    format!("{}, {}!", greeting, name)  // 表达式（无分号，作为返回值）
}
```

关键规则：
- 参数**必须声明类型**
- 返回值类型用 `->` 指定
- 最后一个表达式作为返回值（不加分号）
- 无返回值时，函数返回单元类型 `()`

### 多返回值

通过元组返回多个值：

```rust
fn swap(x: i32, y: i32) -> (i32, i32) {
    (y, x)
}

let (a, b) = swap(1, 2);  // a=2, b=1
```

## 控制流

### if 表达式

`if` 是**表达式**，可以有返回值：

```rust
let condition = true;
let result = if condition { 5 } else { 6 };
// result = 5
```

各分支必须返回相同类型。条件必须是 `bool`——不能像 C 那样写 `if x { }`。

### 三种循环

| 循环 | 场景 | 示例 |
|------|------|------|
| `for` | 遍历集合（推荐） | `for i in 0..5 { }` |
| `while` | 条件循环 | `while n > 0 { }` |
| `loop` | 无限循环 | `loop { break; }` |

`for` 是最常用、最安全的循环形式：

```rust
// 遍历范围
for i in 0..5 { }      // 0..5 不包含 5
for i in 0..=5 { }     // 0..=5 包含 5

// 遍历数组
let arr = [10, 20, 30];
for item in &arr {
    println!("{}", item);
}

// 带索引
for (i, val) in arr.iter().enumerate() {
    println!("arr[{}] = {}", i, val);
}
```

`loop` 可以**返回值**：

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;  // break 带返回值
    }
};
// result = 20
```

`loop` 支持**标签**来跳出嵌套循环：

```rust
'outer: loop {
    loop {
        break 'outer;  // 跳出外层循环
    }
}
```

## 跨语言对比

| 概念 | Rust | C/C++ | Java | Python |
|------|------|-------|------|--------|
| 默认可变性 | 不可变 | 可变 | 可变 | 可变 |
| 空值 | Option\<T\> | NULL/nullptr | null | None |
| 返回多值 | 元组 | struct/指针 | 对象 | tuple |
| if 语句 | 表达式 | 语句 | 语句 | 表达式 |

## 练习

1. 写一个函数，接收摄氏度温度，返回华氏度（`°F = °C × 9/5 + 32`）
2. 用 `for` 循环打印九九乘法表
3. 用 `loop` + `break` 写一个猜数字小游戏（暂时用固定数字代替随机数）

---

← [第 1 章：环境搭建](./01-getting-started.md) | [返回目录](./README.md) | → [第 3 章：所有权系统](./03-ownership.md)
