# 第 4 章：借用与引用

## 学习目标

- 理解引用（&）的语义——"借而不拥有"
- 掌握不可变引用和可变引用的规则
- 用 NLL 理解引用作用域的实际范围
- 使用切片类型安全地操作集合的子集
- 能解释为什么借用规则能防止数据竞争

## 上一章的问题

回顾所有权规则：把 `String` 传给函数后，所有权就没了。

```rust
fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)  // 必须把 String 还回去
}

let s = String::from("hello");
let (s, len) = calculate_length(s);  // 拿回来才能继续用
```

这太繁琐了。你需要一种"暂时使用但不拿走所有权"的方式——这就是**借用**。

## 语法层：引用与切片的语法全貌

本章引入三种新语法形式，先看清全貌：

```rust
let s = String::from("hello");

// 1. 引用：&T（不可变）和 &mut T（可变）
let r: &String = &s;          // 创建不可变引用（可以有多个）
let r: &mut String = &mut s;  // 创建可变引用（同一作用域只能有一个）

// 2. 解引用：* 访问引用指向的值
fn len(s: &String) -> usize {
    (*s).len()    // *s = 取出引用指向的 String，然后 .len()
    s.len()       // Rust 自动解引用，等价于上一行
}

// 3. 切片：&[T]（数组切片）和 &str（字符串切片）
let arr = [1, 2, 3, 4, 5];
let slice: &[i32] = &arr[1..4];   // [2, 3, 4]，含头不含尾
let s = &s[0..2];                  // &str = "he"
let s = &s[..];                    // &str = 整个字符串
```

> 💡 引用 `&T` 本质是存了一个地址的变量——类似 C 的指针，但编译器保证它始终指向有效数据。切片 `&[T]` 是一个胖指针（地址 + 长度），提供对集合子集的零成本视图。

## 引用——借用的实现

```rust
fn calculate_length(s: &String) -> usize {  // & 表示"借用"
    s.len()           // 使用借来的 String
}                     // s 离开作用域，但因为不拥有所有权，什么都不会释放

let s = String::from("hello");
let len = calculate_length(&s);  // &s 创建一个指向 s 的引用
println!("'{}' 的长度是 {}", s, len);  // s 仍然有效！
```

发生的三件事：
1. `&s` 创建了一个**引用**——指向 `s` 但不拥有它
2. 函数通过引用使用 `s`，不获取所有权
3. 函数结束时没有释放 `s`——它只是"还回了"借用

### 图解引用

```
let s = String::from("hello");    // s 拥有 String
let r = &s;                       // r 是 s 的引用

栈上：
s ──→ [ptr][len][cap]    ← s 是所有者的绑定
r ──→ [s 的地址]          ← r 是引用（实际上存的是 s 的栈地址）

堆上：
[h][e][l][l][o]          ← 实际数据
```

> 💡 引用本质上是**存了一个地址的变量**，类似 C/C++ 的指针，但有编译期安全检查：引用保证指向有效数据，永远不会是 null（除非用 `unsafe`）。

## 不可变引用（&T）

默认引用是**不可变**的——你只能读，不能改：

```rust
let s = String::from("hello");
let r1 = &s;
let r2 = &s;         // ✅ 可以有多个不可变引用
println!("{}, {}", r1, r2);

// r1.push_str("!"); // ❌ 不能通过不可变引用修改
```

## 可变引用（&mut T）

要修改，需要 `&mut`：

```rust
fn append_world(s: &mut String) {
    s.push_str(", world!");
}

let mut s = String::from("hello");  // s 本身必须是 mut
append_world(&mut s);                // 创建可变引用
println!("{}", s);                   // "hello, world!"
```

注意：`s` 必须是 `mut`——即使你打算通过引用修改，原变量也要声明为可变的。这保证你意识到谁可能修改它。

## 借用规则——Rust 安全性的基石

> **同一时刻，只能拥有以下两者之一：**
> - 任意数量的不可变引用（`&T`）
> - 恰好一个可变引用（`&mut T`）
>
> **且：引用必须始终有效**（不能指向已释放的内存）

### 为什么需要这个规则？

防止**数据竞争**——两个或多个指针同时访问同一数据，至少有一个在写，且没有同步机制：

```rust
let mut data = vec![1, 2, 3];

// ❌ 数据竞争被编译器阻止：
let r1 = &data;           // 不可变引用
let r2 = &mut data;       // 编译错误！同时有不可变和可变引用
// 如果允许，r1 可能读到被 r2 修改了一半的数据

// ✅ 编译器保证：
let r1 = &data;
let r2 = &data;           // 多个不可变引用，没问题
println!("{:?} {:?}", r1, r2);
// r1, r2 最后一次使用后，可以创建可变引用
let r3 = &mut data;       // ✅
```

### NLL（Non-Lexical Lifetimes）——编译器比你想象的聪明

引用从声明开始，到**最后一次使用**结束（不是到作用域结束！）：

```rust
let mut s = String::from("hello");

let r1 = &s;              // r1 的引用在此开始
let r2 = &s;              // r2 的引用在此开始
println!("{} and {}", r1, r2);  // r1, r2 在这是最后一次使用
// r1 和 r2 在此之后"不再存在"（按 NLL 规则）

let r3 = &mut s;          // ✅ 没问题！r1, r2 已经"结束"了
r3.push_str(" world");
```

在 NLL 之前（Rust 1.0-1.30），引用要活到作用域结束，这段代码会报错。NLL 让借用检查更精确，减少了不必要的编译错误。

### 实际案例：遍历中修改

```rust
let mut v = vec![1, 2, 3, 4, 5];

// ❌ 错误：遍历持有不可变引用时不能修改
// for item in &v {
//     v.push(*item * 2);  // 编译错误
// }

// ✅ 正确：通过索引遍历（不持有引用）
for i in 0..v.len() {
    let val = v[i];  // 拷贝出值
    v.push(val * 2); // 修改 v
    if v.len() > 20 { break; }
}
```

## 可变引用的限制：防止"别名可变"

```rust
let mut s = String::from("hello");

let r1 = &mut s;
// let r2 = &mut s;  // ❌ 编译错误：同时有两个可变引用

r1.push_str("!");
// r1 的最后一次使用

let r2 = &mut s;     // ✅ r1 已经结束，可以创建新的可变引用
r2.push_str("?");
```

这个限制防止了一种经典的并发 bug——即使代码是单线程的。如果两个可变引用同时存在，修改可能互相覆盖，导致不可预测的结果。

## 悬垂引用——Rust 阻止你犯错

```rust
fn dangle() -> &String {     // 返回引用
    let s = String::from("hello");
    &s                         // 引用 s
}                              // s 被释放！
// 返回的引用指向已经被释放的内存 → 悬垂引用
// 编译器报错：missing lifetime specifier
```

正确做法：

```rust
fn no_dangle() -> String {   // 返回所有权
    let s = String::from("hello");
    s                           // 所有权转移给调用者
}
```

## 切片（Slice）——对集合的引用

切片是对集合中**连续一段元素**的引用：

### 字符串切片 `&str`

```rust
let s = String::from("hello world");

let hello = &s[0..5];   // "hello"
let world = &s[6..11];  // "world"
```

内存布局：
```
s ──→ [h][e][l][l][o][ ][w][o][r][l][d]  (String 在堆上)
hello ──→ 指向 s 的索引 0，长度 5          (&str，存指针+长度)
world ──→ 指向 s 的索引 6，长度 5          (&str，存指针+长度)
```

切片的缩写语法：

| 写法 | 等价于 | 含义 |
|------|--------|------|
| `&s[..5]` | `&s[0..5]` | 开头到索引 5（不含） |
| `&s[6..]` | `&s[6..len]` | 索引 6 到末尾 |
| `&s[..]` | `&s[0..len]` | 整个字符串 |

### 数组切片 `&[T]`

```rust
let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..4];  // [2, 3, 4]
let all = &arr[..];      // [1, 2, 3, 4, 5]

println!("第一个: {}", slice[0]);  // 2
println!("长度: {}", slice.len()); // 3
```

### `&str` vs `&String` — 重要的设计选择

在函数参数中，**优先使用 `&str` 而非 `&String`**：

```rust
// ❌ 不推荐：限制调用者必须传 &String
fn greet_bad(s: &String) { println!("Hello, {}!", s); }

// ✅ 推荐：更灵活
fn greet_good(s: &str) { println!("Hello, {}!", s); }

let owned = String::from("Alice");
greet_good(&owned);       // &String → &str（自动转换，deref coercion）
greet_good("Bob");        // 字符串字面量直接是 &str

// greet_bad("Bob");      // 编译错误！需要 &String
```

这个转换是**自动的**——`String` 实现了 `Deref<Target=str>`，所以 `&String` 可以自动转换为 `&str`。这就是 Deref 强制转换（详见第 14 章）。

### 一个实际的切片例子

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];  // 返回第一个空格之前的切片
        }
    }
    &s[..]  // 没有空格：返回整个字符串的切片
}

let s = String::from("hello world");
let word = first_word(&s);
println!("第一个单词: {}", word);  // "hello"
// s 仍然可用——first_word 只是借用了 s
```

## 借用规则总结

```
                        ┌──────────────────┐
                        │   借用规则总结     │
                        ├──────────────────┤
                        │ &T  ：多个可共存  │
                        │ &mut T：独占唯一  │
                        │ 两者不能同时活跃  │
                        │ 引用必须始终有效  │
                        └──────────────────┘
```

这个规则的意义：**在编译期保证，访问共享数据时，要么多人读（安全），要么一个人写（独占）。**

## 跨语言对比

| 概念 | Rust | C/C++ | Java/Python |
|------|------|-------|-------------|
| 引用/指针 | `&T` / `&mut T` | `T*` / `T&` | 所有对象变量 |
| 空引用 | 不允许 | NULL/nullptr | null/None |
| 数据竞争 | 编译错误 | 未定义行为 | 运行时处理 |
| 切片 | `&[T]`，安全 | 指针+长度，危险 | 无内置对应 |
| 悬垂引用 | 编译错误 | 运行时崩溃 | 不适用（GC） |

## 常见陷阱

> ⚠️ **在还有不可变引用时尝试获取可变引用。** NLL 规则下，只要不可变引用不再使用，编译器就允许。在 `println!` 之后创建 `&mut` 通常是 OK 的。

> ⚠️ **尝试返回局部变量的引用。** 编译器会报错。解决方式：返回所有权（`String`），或者接受一个引用参数并返回（像 `first_word` 那样）。

> ⚠️ **可变引用只能有一个。** 当你需要"借用数组的不同部分"时，可以用 `split_at_mut`——它安全地创建两个互不重叠的可变切片。

```rust
let mut arr = [1, 2, 3, 4, 5];
let (left, right) = arr.split_at_mut(3);
// left: &mut [1, 2, 3]
// right: &mut [4, 5]
// 两个可变引用——但编译器知道它们不重叠，所以是安全的
```

## 练习

1. 写一个函数，接收 `&mut String` 并反转其内容（提示：先 `s.chars().rev().collect()` 转为 String，再 `*s = ...`）
2. 写 `first_word` 和 `last_word` 函数，返回 `&str` 切片。思考：为什么返回 `&str` 而不是 `String`？
3. 尝试故意创建同时存在的可变引用和不可变引用，观察编译错误信息
4. 用 `split_at_mut` 实现一个函数：接收 `&mut [i32]`，将前半部分所有元素乘以 2，后半部分所有元素加 1

---

← [第 3 章：所有权系统](./03-ownership.md) | [返回目录](./README.md) | → [第 5 章：结构体与枚举](./05-struct-enum.md)
