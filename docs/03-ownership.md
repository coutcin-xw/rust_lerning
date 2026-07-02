# 第 3 章：所有权系统

## 学习目标

- 理解 Rust 所有权三大规则的底层原理
- 深刻区分移动（move）和复制（copy）语义
- 理解 `Drop` trait 和 RAII 模式
- 知道何时使用 `clone()`
- 能够解释"为什么 Rust 需要所有权"

## 内存管理的历史背景

在你理解所有权之前，先看其他语言怎么管理内存：

### 方式一：手动管理（C/C++）

```c
// C 代码
char* s = malloc(100);  // 申请内存
// ... 使用 s ...
free(s);                // 必须手动释放！
// 问题：
//   忘记 free → 内存泄漏
//   free 太早 → 悬垂指针（use-after-free）
//   free 两次 → double free（未定义行为）
```

### 方式二：垃圾回收（Java/Python/Go/JS）

```java
// Java 代码
String s = new String("hello");
// ... 使用 s ...
// 不需要手动释放，GC 会在某个不确定的时刻自动回收
// 问题：
//   运行时开销（GC 线程占用 CPU）
//   "stop the world" 暂停（卡顿）
//   内存释放时机不可控
```

### Rust 的方式：编译期所有权检查

```
          手动管理          GC           所有权
         (C/C++)       (Java/Go)       (Rust)
控制权：  ✅ 完全控制     ❌ 无控制       ✅ 完全控制
安全性：  ❌ 全靠自己     ✅ 安全         ✅ 安全
性能：    ✅ 零开销       ❌ 运行时开销    ✅ 零开销
时机：    ✅ 精确         ❌ 不可控       ✅ 编译期确定
```

Rust 通过所有权系统，在**编译时**确定每块内存的分配和释放时机。编译通过的内存管理就已经是正确的——不需要运行时 GC，也不需要人工 `free`。

## 所有权三大规则

> **规则 1**：Rust 中每个值都有一个**所有者**（owner）。
>
> **规则 2**：同一时刻只能有**一个所有者**。
>
> **规则 3**：所有者离开作用域时，值被**自动释放**（调用 `drop`）。

这三条规则看起来很抽象。让我们用代码把它们具体化。

### 作用域和自动释放

```rust
{                               // s 还不存在
    let s = String::from("hello");  // s 进入作用域，是 "hello" 的所有者
    println!("{}", s);
}                               // s 离开作用域，"hello" 占用的内存自动释放
```

这段简单的代码背后发生的事：
1. `String::from("hello")` 在**堆**上分配内存存储 "hello"
2. `s` 是这块内存的所有者
3. `}` 时编译器自动插入 `drop(s)`——释放堆内存

你不用写 `free(s)`，不用等 GC。编译器精确知道每个值什么时候不再需要。

### 堆与栈——为什么区分很重要？

```rust
// 栈上数据：大小编译期已知，复制很快
let x = 5;
let y = x;   // 复制了 4 字节，x 和 y 各自独立

// 堆上数据：大小可能变化，复制慢
let s1 = String::from("hello");
let s2 = s1;  // 这里发生了什么？
```

`String` 的内部结构：
```
栈上（s1/s2 变量本身）：
┌─────────┬─────────┬─────────┐
│ 指针 ptr │ 长度 len │ 容量 cap │  ← 3 个 usize，共 24 字节（64 位）
└────┬─────┴─────────┴─────────┘
     │ 指向
     ▼
堆上（实际的字符串数据）：
┌───┬───┬───┬───┬───┐
│ h │ e │ l │ l │ o │  ← 5 字节 + 容量预留
└───┴───┴───┴───┴───┘
```

## 移动（Move）——所有权的转移

```rust
let s1 = String::from("hello");
let s2 = s1;  // 所有权从 s1 转移给 s2

// println!("{}", s1);  // 编译错误！s1 已失效
println!("{}", s2);     // OK
```

赋值后的内存状态：

```
赋值前：
s1 ──→ [h][e][l][l][o]  (堆上)

赋值后（move）：
s1 ──X                       (s1 失效)
s2 ──→ [h][e][l][l][o]  (堆上，同一块内存)

离开作用域时：
只有 s2 的 drop 被调用，释放堆内存
不会发生 double free！
```

**关键洞察：** 如果 `s1` 和 `s2` 都指向同一块堆内存，离开作用域时会释放两次（double free）。Rust 通过 move 语义——令 `s1` 失效——避免了这个问题。**没有运行时开销**，完全是编译期的规则。

### 函数调用也是 move

```rust
fn eat(s: String) {
    println!("吃掉了: {}", s);
}  // s 被 drop

let food = String::from("食物");
eat(food);
// println!("{}", food);  // 编译错误！food 已经被"吃掉"了
```

如果你想在函数调用后继续使用这个值，你有两个选择：

**选择 1：返回所有权**
```rust
fn process_and_return(s: String) -> String {
    println!("处理: {}", s);
    s  // 把所有权还回去
}

let data = String::from("data");
let data = process_and_return(data);  // data 又回来了
// 但这种方式比较繁琐...
```

**选择 2：借用（下一章重点）**
```rust
fn peek(s: &String) {  // & 表示借用，不获取所有权
    println!("{}", s);
}

let data = String::from("data");
peek(&data);       // 借出去
println!("{}", data);  // data 仍然可用！
```

## Copy 类型——赋值时自动复制

```rust
let x = 5;
let y = x;   // x 被**复制**给 y
println!("x={}, y={}", x, y);  // 两个都能用！
```

根据规则 2，同一时刻只能有一个所有者。但 `x = 5; let y = x;` 两个都能用——为什么？

因为 `i32` 实现了 `Copy` trait。赋值时，Rust 在**栈上复制了一份新数据**，而不是移动所有权。

### 哪些类型是 Copy？

| 类别 | 示例 | Copy？ |
|------|------|--------|
| 整型 | `i32`, `u64`, `usize` | ✅ |
| 浮点 | `f32`, `f64` | ✅ |
| 布尔 | `bool` | ✅ |
| 字符 | `char` | ✅ |
| Copy 的元组 | `(i32, i32)` | ✅ |
| 非 Copy 的元组 | `(i32, String)` | ❌ |
| 引用 | `&T`, `&mut T` | ✅ |
| String | — | ❌ |
| Vec\<T\> | — | ❌ |
| 自定义 struct | — | ❌（默认） |

判断法则是：**数据全部在栈上**的类型可以实现 `Copy`。如果类型拥有堆数据（`String`, `Vec`），就不能是 `Copy`——复制整个堆内存代价太高。

> 💡 如果你想检查一个类型是否实现了 `Copy`，试着赋值后访问原变量。编译器会明确告诉你。

### 函数调用时的 Copy

```rust
fn process(n: i32) {
    println!("{}", n);
}

let x = 5;
process(x);        // x 被复制传给函数
println!("{}", x); // x 还在！
```

## Clone——显式深拷贝

当你确实需要堆数据的独立副本时：

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // 在堆上新分配一块内存，复制 "hello"

println!("s1={}, s2={}", s1, s2);  // 两个独立的值，互不影响
```

`clone()` 后的内存状态：
```
clone 后：
s1 ──→ [h][e][l][l][o]  (堆上，第一份)
s2 ──→ [h][e][l][l][o]  (堆上，第二份，独立分配)
```

> ⚠️ `clone()` 是有代价的——它会复制堆上的数据。只在确实需要独立副本时使用。Rust 倾向于"复用"和"借用"而不是复制。这就是为什么 clone 必须显式调用——让你意识到这个操作的成本。

## Drop trait

`Drop` 是 Rust 版的析构函数。当值离开作用域时，编译器自动调用 `drop()`：

```rust
struct Connection {
    id: u32,
    name: String,
}

impl Drop for Connection {
    fn drop(&mut self) {
        // 在这里做清理工作
        println!("关闭连接 #{}: {}", self.id, self.name);
    }
}

{
    let conn = Connection { id: 1, name: String::from("db") };
    println!("使用连接中...");
    // 此处可以做任何事
}  // conn 离开作用域 → 自动调用 drop → 打印"关闭连接 #1: db"
```

**Drop 的典型用途：**
- 关闭文件/网络连接
- 释放锁（`MutexGuard`）
- 归还内存到分配器
- 日志记录资源生命周期

### 变量的释放顺序

```rust
{
    let a = Resource("A");
    let b = Resource("B");
    let c = Resource("C");
    // 释放顺序：c → b → a（后进先出，类似栈）
}
```

### 手动提前释放

```rust
let s = String::from("hello");
drop(s);  // ✅ 显式释放
// 注意：不能写 s.drop()！编译器禁止直接调用 drop 方法
// 原因：编译器会在作用域结束时再次调用 drop，导致 double free
```

> 📘 *Rust 2024 Edition 优化了尾表达式中临时值的释放时机：临时值在局部变量**之前**释放。这使得一些在 2021 中会报错的借用模式现在可以正常编译。*

```rust
// 2024 Edition：✅ 通过
fn demo() -> usize {
    let c = std::cell::RefCell::new("hello");
    c.borrow().len()  // 临时 Ref 在 c 之前释放
}
// 2021 Edition：❌ 编译错误：c 活得不够长
```

## 所有权的常见模式

### 模式 1：创建后立即传递给函数

```rust
fn build_report(data: String) -> String {
    format!("# 报告\n\n{}", data)
}

let raw = fetch_data();       // String
let report = build_report(raw);  // raw 移动进去，report 出来
// raw 在此不可用
```

### 模式 2：使用 Clone 保留原值

```rust
let original = String::from("重要数据");
let copy = original.clone();     // 显式深拷贝
process(original);               // 移动原值
store(copy);                     // 使用副本
```

### 模式 3：使用借用代替移动

```rust
fn analyze(data: &String) {     // 借用，不移动
    println!("分析: {}", data);
}

let data = String::from("data");
analyze(&data);                  // 借出去
analyze(&data);                  // 可以再借
println!("{}", data);            // data 还在
```

## 跨语言对比

| 操作 | C/C++ | Java/Python | Rust |
|------|-------|-------------|------|
| 赋值堆类型 | 复制指针（浅拷贝） | 复制引用 | **移动**（原变量失效） |
| 赋值简单类型 | 复制值 | 引用（自动装箱） | 复制值（Copy） |
| 深拷贝 | 手动实现 | `clone()` / `copy.copy()` | `clone()` |
| 释放 | `free`/`delete` | GC 自动 | 自动 `drop` |
| 重复释放 | 未定义行为 | 不适用（GC 处理） | 编译错误 |

## 常见陷阱和误解

> ⚠️ **"为什么 i32 能 Copy 而 String 不能？"** 不是编译器偏向基本类型——而是所有数据都在栈上的类型才能 Copy。String 包含堆指针，浅拷贝会导致 double free，深拷贝成本高不应隐式发生。

> ⚠️ **move 后编译器报 "value borrowed after move"。** 这是 Rust 最经典的编译错误。遇到它，问自己两个问题：(1) 我能用引用（`&`）代替所有权转移吗？(2) 我需要 clone 吗？

> ⚠️ **误用 clone 掩盖设计问题。** 代码中到处都是 `.clone()` 通常意味着你没有正确使用借用。clone 应该是例外，不是规则。

> ⚠️ **"Rust 太麻烦了，每次赋值都要想所有权。"** 开始确实如此。但几周后你会发现，C/C++ 的自由其实是"不知道哪里会出 bug 的恐惧"，Rust 的约束其实是"编译器帮你想清楚了"。

## 心理模型：把所有权看作"唯一责任方"

把 Rust 的值看作需要维护的资产。每个值有一个负责人（所有者）。负责人离开工作岗位时，资产被清理。你不能把同一份责任同时交给两个人（否则他们会 double-free）。你可以暂时借给别人使用（借用，下一章），或者复制一份独立的资产（clone）。

这个心理模型适用于所有 Rust 代码。当你写 `let x = y` 时，问自己：我是在复制栈上的数据，还是在转移所有权？

## 练习

1. **跟踪所有权**：写一段代码，创建 String、移动它、尝试使用它。仔细阅读编译错误信息，理解 move 发生的位置。然后改写为使用借用的版本。
2. **实现 Drop**：定义一个 `FileResource` 结构体，存储文件名。实现 `Drop` trait，在释放时打印 `"关闭文件: {name}"`。观察在不同作用域中释放的时机。
3. **所有权链**：写三个函数 `create() -> String`, `transform(s: String) -> String`, `display(s: &String)`。把 `create` 的结果传给 `transform`，再传给 `display`。观察所有权流动。
4. **Copy vs Clone**：分别尝试对 `i32`、`String`、`Vec<i32>` 进行赋值操作，对比哪些能继续使用原变量。猜猜为什么。

---

← [第 2 章：基础语法](./02-basics.md) | [返回目录](./README.md) | → [第 4 章：借用与引用](./04-borrowing.md)
