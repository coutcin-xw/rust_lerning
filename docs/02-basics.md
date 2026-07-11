# 第 2 章：基础语法

## 学习目标

- 理解 Rust 变量默认不可变的设计原因
- 掌握所有基本数据类型及其适用场景
- 理解表达式与语句在 Rust 中的区别
- 熟练使用 `if` 表达式、三种循环、`match` 初探
- 理解类型推断和类型标注的平衡

## 概念层：Rust 的基本设计哲学

Rust 的基础设计与 C/Java/Python 有根本性不同：变量默认不可变、几乎所有语法结构都是表达式、类型系统严格但依赖推断而非强制标注。这些并非随意选择——默认不可变让重构和并发不再需要"提心吊胆"；表达式语法消除了"不小心加分号导致意外返回"这类整类 bug；严格的类型系统结合强大的推断能力，在编译期发现错误的同时避免了冗长的类型声明。

> 💡 本章介绍 Rust 的基本构建块——变量、类型、函数、控制流。它们看似零散，但每一个都会在第 3-4 章中与所有权系统交汇。记住：Rust 不是 C 的变种，而是一门以"安全+表达力"为轴心重新设计的语言。

## 变量与可变性

### 默认不可变：Rust 的设计哲学

Rust 变量**默认不可变**。这不是限制，而是设计选择：

```rust
let x = 5;       // x 不可变
// x = 6;        // 编译错误：cannot assign twice to immutable variable
```

**为什么默认不可变？**

1. **安全性**：你不会意外修改不该改的值。大型代码库中，大部分变量其实不需要变化。
2. **并发友好**：不可变数据天然线程安全——多线程读同一块数据无需同步。
3. **编译器优化**：编译器知道不可变值不会改变，可以更激进地优化。

如果你来自 C/Java/Python，这可能是最大的习惯转变——这些语言默认可变。但一旦适应，你会发现 `let` 默认不可变让代码的意图更清晰。

### 显式可变

```rust
let mut y = 10;  // mut = mutable，显式标记"这个变量会变"
y = 20;          // OK
y += 5;          // OK
```

`mut` 是给读者（和编译器）的信号："注意，这个值会改变。"

### 常量 const

```rust
const MAX_CONNECTIONS: u32 = 100_000;
const PI: f64 = 3.141592653589793;
```

| | `const` | `let`（不可变） |
|---|---|---|
| 声明位置 | 任何作用域（包括全局） | 只能在函数/代码块内 |
| 类型要求 | **必须**显式标注 | 编译器推断（也可显式标注） |
| 值 | 编译期常量表达式 | 运行时计算也可以 |
| 命名规范 | `SCREAMING_SNAKE_CASE` | `snake_case` |
| 内存 | 内联到使用处（无固定地址） | 栈上分配 |

```rust
// ✅ const 可以是表达式
const HOURS_IN_DAY: u32 = 24;
const MINUTES_IN_DAY: u32 = HOURS_IN_DAY * 60;

// ❌ const 不能是运行时值
// const NOW: u64 = std::time::SystemTime::now();  // 编译错误
```

### 变量遮蔽（Shadowing）

Rust 允许用 `let` 重新声明同名变量：

```rust
let x = 5;
let x = x + 1;       // x = 6，新的 x 遮蔽了旧的 x
let x = "字符串";     // x 的类型也变了！从 i32 变成 &str
```

这与 `mut` 的本质区别：

| | `let x = ...`（遮蔽） | `x = ...`（mut 赋值） |
|---|---|---|
| 类型 | **可以改变** | **不能改变** |
| 不可变性 | 新变量可以设为不可变 | 无——原变量本来就是可变的 |
| 创建新变量 | ✅ 每次 let 创建新绑定 | ❌ 修改同一个变量 |

```rust
// 遮蔽的典型用法：逐步变换值，但保持不可变性
let data = read_file("data.txt");    // String
let data = parse(&data);             // Data 结构体
let data = validate(data)?;          // 还是 Data，但经过验证
// 每个阶段 data 都是不可变的，但你在"变换"它
```

## 基本数据类型

Rust 是**静态类型**语言——每个值在编译期就有确定的类型。但得益于类型推断，你不需要每次都显式写出类型。

### 整型

Rust 的整型命名规则：`i` = 有符号（signed），`u` = 无符号（unsigned），后面跟位数：

| 长度 | 有符号 | 无符号 |
|------|--------|--------|
| 8 位 | `i8` | `u8` |
| 16 位 | `i16` | `u16` |
| 32 位 | `i32` | `u32` |
| 64 位 | `i64` | `u64` |
| 128 位 | `i128` | `u128` |
| 平台相关 | `isize` | `usize` |

```rust
let x: i32 = 42;           // 显式类型标注
let y = 100;               // 默认推断为 i32（未约束时）
let z: u64 = 10_000_000;   // 下划线分隔，提高可读性

// 进制表示
let hex = 0xff;            // 十六进制 = 255
let oct = 0o77;            // 八进制 = 63
let bin = 0b1111_0000;     // 二进制 = 240
let byte = b'A';           // 字节字面量 = 65u8
```

**默认选 `i32`**——它在绝大多数场景下性能最好，范围也够用。除非你有特殊原因（内存约束选更小的、大数值选更大的、FFI 交互匹配 C 类型）。

> ⚠️ **整型溢出**：debug 模式下会 panic，release 模式下会**回绕**（wrap），即 255u8 + 1 = 0。需要显式处理溢出用 `wrapping_add()`、`saturating_add()` 等方法。

### 浮点型

```rust
let x = 2.0;       // 默认 f64
let y: f32 = 3.0;  // 显式 f32

// f64 是默认选择——现代 CPU 上 f64 速度和 f32 一样快，但精度更高
```

> ⚠️ 浮点数**不实现 `Eq`**（不能直接用 `==` 比较是否精确相等，虽然操作符上可以写但可能得到意外结果）。涉及浮点比较时考虑用差值小于 epsilon 的方式。

### 布尔型和字符

```rust
let t = true;
let f: bool = false;

// if 的条件必须是 bool，不能像 C 那样 if (x) { }
let x = 5;
// if x { }  // 编译错误！expected `bool`, found integer
if x > 0 { }  // ✅ 明确的条件表达式
```

```rust
let c = 'A';         // ASCII 字符
let z = '中';        // 中文字符
let emoji = '😀';    // Emoji

// char 是 4 字节（Unicode 标量值），不是 C 的 1 字节
println!("'A' 占用 {} 字节", std::mem::size_of_val(&'A'));  // 4
```

### 元组（Tuple）

元组是固定长度的异构集合：

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// 解构——最常用的访问方式
let (x, y, z) = tup;
println!("x={}, y={}, z={}", x, y, z);

// 点标记法访��
let first = tup.0;   // 500
let second = tup.1;  // 6.4
let third = tup.2;   // 1

// 单元元组：表示"没有值"
let empty = ();  // 类型是 ()，也叫"单元类型"
```

元组的典型用法：
```rust
// 1. 函数返回多个值
fn split_at(s: &str, pos: usize) -> (&str, &str) {
    (&s[..pos], &s[pos..])
}
let (left, right) = split_at("hello", 2);

// 2. 临时打包相关数据
let point = (3.0, 4.0);
let distance = (point.0.powi(2) + point.1.powi(2)).sqrt();
```

### 数组（Array）

数组是固定长度、同类型的集合，**在栈上分配**：

```rust
let a = [1, 2, 3, 4, 5];          // 类型推断：[i32; 5]
let b: [u8; 3] = [0, 0, 0];
let c = [0; 100];                  // [0, 0, ..., 0]（100 个零）

// 访问
let first = a[0];
let last = a[a.len() - 1];

// 越界访问：运行时 panic（不是未定义行为！）
// let oops = a[99];  // panic: index out of bounds
```

数组 vs Vector：

| | 数组 `[T; N]` | Vector `Vec<T>` |
|---|---|---|
| 长度 | 编译期确定 | 运行时可变 |
| 分配 | 栈（大数组会放在静态区） | 堆 |
| 使用 | 长度固定且已知 | 需要动态增减 |
| 传递 | 复制或借用，不移动元素 | 移动（所有权），元素在堆上 |

## 函数

### 定义语法

```rust
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // 函数体
    // 最后一个表达式作为返回值（无分号）
}
```

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y  // 注意：没有分号！这是表达式，作为返回值
}

let result = add(3, 5);  // 8
```

### 表达式 vs 语句

这是 Rust 区别于 C 系语言的重要设计：

| | 语句（Statement） | 表达式（Expression） |
|---|---|---|
| 特征 | 执行动作，无返回值 | 计算并返回一个值 |
| 结尾 | 分号 `;` | 无分号（通常） |
| 示例 | `let x = 5;`, `fn foo() {}` | `5`, `x + y`, `{ let x = 3; x + 1 }` |
| 能放在赋值右边吗 | ❌ | ✅ |

```rust
// if 是表达式！
let condition = true;
let number = if condition { 5 } else { 6 };  // 类似三元运算符

// 代码块也是表达式！
let y = {
    let x = 3;
    x + 1  // 无分号，作为整个代码块的返回值
};  // y = 4
// 注意：x 只在代码块内有效

// loop 也是表达式！
let mut count = 0;
let result = loop {
    count += 1;
    if count == 10 {
        break count * 2;  // break 带出返回值
    }
};  // result = 20
```

> 💡 关键理解：Rust 中几乎所有东西都是表达式。这在写函数式风格的代码时特别方便——`map`、`and_then`、链式调用都依赖这个特性。

### 多返回值

使用元组返回多个值：

```rust
fn min_max(slice: &[i32]) -> (i32, i32) {
    let min = slice.iter().min().copied().unwrap_or(0);
    let max = slice.iter().max().copied().unwrap_or(0);
    (min, max)
}

let (min, max) = min_max(&[3, 1, 4, 1, 5, 9]);
println!("最小: {}, 最大: {}", min, max);
```

### 函数指针

```rust
fn add(x: i32, y: i32) -> i32 { x + y }
fn sub(x: i32, y: i32) -> i32 { x - y }

fn apply(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

let result = apply(add, 3, 5);   // 8
let result = apply(sub, 10, 3);  // 7
```

## 控制流

### if 表达式

```rust
let number = 13;

// 基本用法
if number < 10 {
    println!("个位数");
} else if number < 100 {
    println!("两位数");
} else {
    println!("三位数或更多");
}

// if 作为表达式
let category = if number < 0 {
    "负数"
} else if number == 0 {
    "零"
} else {
    "正数"
};
// 所有分支必须返回相同类型！
```

> ⚠️ `if` 的条件**必须是 `bool`**。不能像 C 那样 `if (x)` 判断非零。这是有意为之——Rust 拒绝隐式类型转换，避免一类常见 bug。

### 三种循环的选择

```rust
// 1. for —— 遍历集合（首选，最安全）
let arr = [10, 20, 30, 40, 50];
for element in arr {
    println!("{}", element);
}
for i in 0..arr.len() {
    println!("arr[{}] = {}", i, arr[i]);
}
for (index, value) in arr.iter().enumerate() {
    println!("arr[{}] = {}", index, value);
}

// 2. while —— 条件循环
let mut n = 10;
while n > 0 {
    print!("{} ", n);
    n -= 1;
}
// 输出: 10 9 8 7 6 5 4 3 2 1

// 3. loop —— 无限循环（需要 break）
let mut tries = 0;
loop {
    tries += 1;
    if tries > 10 {
        println!("放弃");
        break;
    }
    if try_connect() {
        println!("连接成功");
        break;
    }
}
```

**选择原则：**
- 需要遍历 → `for`
- 条件不确定的循环 → `while`
- 无限循环或复杂的退出逻辑 → `loop`

### 范围语法

```rust
for i in 0..5 { }     // 0, 1, 2, 3, 4（不包含 5）
for i in 0..=5 { }    // 0, 1, 2, 3, 4, 5（包含 5）
for i in (0..5).rev() { }  // 4, 3, 2, 1, 0（反向）

// 范围是惰性的，不分配内存
let range = 1..1_000_000;  // 这不会创建一百万个元素的数组
```

## 类型系统和类型推断

Rust 有强大的类型推断，但有些场景需要显式标注：

```rust
// ✅ 编译器能推断
let x = 42;                    // i32
let v = vec![1, 2, 3];        // Vec<i32>

// ❌ 需要显式标注
let guess: u32 = "42".parse().expect("不是数字");
//                    ^^^^^^ parse() 是泛型函数，编译器不知道你想要什么类型

// ✅ 也可以用"涡轮鱼"语法
let guess = "42".parse::<u32>().expect("不是数字");
```

## 跨语言对比速查

| 概念 | Rust | C | Java | Python |
|------|------|---|------|--------|
| 变量默认 | 不可变 | 可变 | 可变 | 可变 |
| 整型默认 | i32 | int (平台) | int (32位) | int (任意精度) |
| 浮点默认 | f64 | double | double | float (64位) |
| null | 无（用 Option\<T\>） | NULL | null | None |
| 函数签名 | `fn f(x: T) -> U` | `U f(T x)` | `U f(T x)` | `def f(x: T) -> U:` |
| 返回多值 | 元组 | 指针/结构体 | 对象 | tuple |
| if 条件 | 必须是 bool | 任意非零 | 必须是 boolean | 任意 truthy |
| 循环 | for/while/loop | for/while/do-while | for/while/do-while | for/while |
| 表达性 | 几乎一切皆表达式 | 语句为主 | 语句为主 | 几乎一切皆表达式 |

## 常见陷阱

> ⚠️ **在 `if` 条件中写赋值。** C 中可以写 `if (x = 5)`（赋值并检查），Rust 不允许——赋值语句没有返回值。这防止了一整类 bug。

> ⚠️ **`let` 语句不能放在 `if` 条件中。** `if let x = get_value() { }` 不是"赋值给 x 然后检查 x"，而是**模式匹配**（见第 6 章）。初学者常混淆。

> ⚠️ **忘记分号导致意外的返回值。** Rust 中省略分号意味着"把这个值作为返回值"。如果你在函数中间无意中写出了无分号的表达式，可能导致类型不匹配的编译错误。

## 练习

1. 写一个温度转换程序：摄氏 ↔ 华氏。用函数实现，接收值和温度制，返回转换后的值。公式：F = C × 9/5 + 32
2. 写一个生成第 n 个斐波那契数的函数。分别用 `for` 循环和递归实现
3. "Twelve Days of Christmas" 歌词生成器：用数组/切片存储歌词片段，循环拼接输出
4. 尝试故意触发以下编译错误，阅读错误信息：
   - 对不可变变量赋值
   - 把 `if` 表达式各分支写成不同类型
   - 访问数组越界的索引

---

← [第 1 章：环境搭建](./01-getting-started.md) | [返回目录](./README.md) | → [第 3 章：所有权系统](./03-ownership.md)
