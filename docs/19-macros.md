# 第 19 章：宏

## 学习目标

- 理解宏与函数的本质区别（编译期代码生成 vs 运行时执行）
- 编写声明式宏（`macro_rules!`），掌握重复模式和卫生性
- 了解过程宏的三种类型，能写简单的 derive 宏
- 建立"能用函数就不用宏"的判断力

---

## 一、概念层：什么是宏

### 宏不是函数

宏和函数最根本的区别：**宏在编译期展开为源代码，函数在运行时执行。**

```
函数调用：  main() → 跳转到函数代码 → 执行 → 返回
宏调用：    macro!() → 编译器展开成新代码 → 编译新代码 → 执行
```

这意味着宏能做函数做不到的事：
- 生成新的 `struct`、`fn`、`impl` 块
- 接受可变数量的参数
- 接受非表达式的语法（如 `struct Foo { a: i32 }` 整个结构体定义作为参数）
- 在编译期根据条件产生不同的代码

也意味着宏有函数没有的代价：
- 增加编译时间（每次调用都要展开）
- 错误信息指向展开后的代码，定位困难
- 不能作为值传递（不能 `let f = my_macro;`）

> 💡 **原则：能用函数解决的问题，不要用宏。** 只有函数真的做不到（如需要生成新类型、接受类型名作为参数、实现编译期 DSL）时，才用宏。

### Rust 的两种宏

| | 声明式宏 `macro_rules!` | 过程宏 |
|---|---|---|
| 定义方式 | 模式匹配 token 流 | Rust 函数处理 token 流 |
| 位置 | 同一个 crate 内 | **必须**在独立 proc-macro crate |
| 学习成本 | 低 | 高（需了解 `syn`、`quote`、AST） |
| 典型用途 | `vec!`、`println!`、简单 DSL | `#[derive(Serialize)]`、`#[tokio::main]` |
| 本章占比 | 重点展开 | 了解为主 |

---

## 二、机制层：声明式宏如何工作

### 编译阶段

```
源代码 → 词法分析(token流) → macro_rules! 展开 → AST → 类型检查 → 代码生成
                              ↑
                         宏在这一步被"内联展开"
                         展开后的代码与其他代码一视同仁
```

宏匹配的是 **token tree**——不是字符串、不是 AST：
- `1 + 2` 是三个 token：`1`, `+`, `2`
- `vec![1, 2]` 作为一个 `expr` fragment 整体匹配
- 宏不会穿透括号（除非用 `tt` fragment specifier）

### Fragment Specifier — 匹配不同类型的 token

> 📘 *在 Rust 2024 Edition 中，`expr` fragment 现在也匹配 `const { }` 和 `_` 表达式。缺少 fragment specifier（如 `$x` 而非 `$x:expr`）是**硬错误**。*

| Specifier | 匹配内容 | 示例 |
|-----------|---------|------|
| `ident` | 标识符 | `foo`, `Bar` |
| `expr` | 表达式 | `1 + 2`, `vec![1,2]`, `const { 5 }` |
| `ty` | 类型 | `i32`, `Vec<String>` |
| `literal` | 字面量 | `"hello"`, `42`, `3.14` |
| `tt` | Token tree（最灵活） | 几乎任何 token 序列 |
| `stmt` | 语句 | `let x = 1;` |
| `block` | 代码块 | `{ ... }` |
| `item` | 条目（函数、结构体等） | `fn foo() {}` |
| `vis` | 可见性修饰符 | `pub`, `pub(crate)` |
| `pat` | 模式 | `Some(x)`, `(a, b)` |

### 宏的卫生性（Hygiene）

Rust 宏是**卫生的**——宏内部定义的变量不会与外部同名变量冲突：

```rust
macro_rules! using_x {
    () => {
        let x = "宏内部的 x";
        println!("宏内: {}", x);
    };
}

let x = "外部的 x";
using_x!();                 // 宏内: 宏内部的 x
println!("外部: {}", x);     // 外部: 外部的 x
// 两个 x 互不影响
```

**例外：通过 `$name:ident` 传入的标识符会与调用处作用域交互**，因为它们来自调用处。除此之外，宏内部自己定义的名字是隔离的。

### 重复模式语法

- `$( ... ),*` — 零个或多个，逗号分隔
- `$( ... ),+` — 一个或多个，逗号分隔
- `$( ... );*` — 零个或多个，分号分隔
- `$( , )?` — 可选末尾符号（如逗号）
- 分隔符可以自定义（`,` `;` `=>` 等）

### 内部规则模式（`@` 约定）

复杂宏通常拆成多个辅助"内部规则"，用 `@` 前缀标记私有模式——调用者见不到 `@` 开头的规则：

```rust
macro_rules! sorted_vec {
    // 公开入口
    ($($x:expr),* $(,)?) => {
        sorted_vec!(@inner [$($x),*])
    };
    // 内部递归——调用者不能直接调用 sorted_vec!(@inner ...)
    (@inner [$($sorted:expr),*] $first:expr, $($rest:expr),*) => {
        sorted_vec!(@inner [$($sorted,)* insert($first, [$($rest),*])])
    };
    (@inner [$($sorted:expr),*]) => {
        vec![$($sorted),*]
    };
}
```

### TT Muncher 模式

利用 `tt` fragment specifier 逐个"吃掉" token 的递归模式：

```rust
macro_rules! html {
    ($tag:ident {}) => {
        format!("<{0}></{0}>", stringify!($tag))
    };
    ($tag:ident { $($rest:tt)* }) => {
        html!(@attrs $tag [] $($rest)*)
    };
    (@attrs $tag:ident [$($attrs:tt)*] $key:ident = $val:expr, $($rest:tt)*) => {
        html!(@attrs $tag [$($attrs)* $key=$val] $($rest)*)
    };
    (@attrs $tag:ident [$($attrs:tt)*]) => {
        format!("<{} {:?}></{}>", stringify!($tag), vec![$($attrs)*], stringify!($tag))
    };
}
```

### 调试宏：cargo expand

```bash
cargo install cargo-expand
cargo expand                    # 展开当前 crate 的所有宏
cargo expand --lib              # 只展开 lib.rs
cargo expand some_module::func  # 只展开特定项
```

写宏时 `cargo expand` 开着，写一条规则跑一次，立即看到展开结果。

---

## 三、代码层：声明式宏实战

### 入门：基本宏

```rust
// 无参数
macro_rules! say_hello {
    () => { println!("Hello, world!"); };
}
say_hello!();

// 带标识符参数——生成函数
macro_rules! create_function {
    ($name:ident) => {
        fn $name() {
            println!("函数 {:?} 被调用", stringify!($name));
        }
    };
}
create_function!(foo);
create_function!(bar);
foo();  // 函数 "foo" 被调用
bar();  // 函数 "bar" 被调用
```

### 进阶：重复模式 + 实用宏

```rust
// hash_map! — 用 JSON 风格创建 HashMap
macro_rules! hash_map {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        let mut map = std::collections::HashMap::new();
        $( map.insert($key, $value); )*
        map
    }};
}
let config = hash_map!["host" => "localhost", "port" => 8080];

// debug_vars! — 打印多个变量名和值
macro_rules! debug_vars {
    ( $( $var:ident ),* ) => {
        $( eprintln!("{} = {:?}", stringify!($var), $var); )*
    };
}
let x = 42; let name = "Alice";
debug_vars!(x, name);
// x = 42
// name = "Alice"

// my_vec! — 理解 vec! 的原理
macro_rules! my_vec {
    ( $( $x:expr ),* $(,)? ) => {
        {
            let mut v = Vec::new();
            $( v.push($x); )*
            v
        }
    };
}
let v = my_vec![1, 2, 3, 4, 5];
```

### 递归宏

```rust
macro_rules! calculate {
    (eval $e:expr) => { $e };
    (eval $e:expr, $( $tail:tt )*) => {
        calculate!(eval $e) + calculate!(eval $( $tail )*)
    };
}
let result = calculate!(eval 1, eval 2, eval 3);  // 6
```

### 工程级宏：struct_builder!

接收一个完整的结构体定义，自动生成 Builder 模式：

```rust
macro_rules! struct_builder {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($field_vis:vis $field:ident: $field_ty:ty),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $name { $($field_vis $field: $field_ty),* }

        #[derive(Default)]
        $vis struct [<$name Builder>] {
            $( $field: Option<$field_ty>, )*
        }

        impl [<$name Builder>] {
            $(
                pub fn $field(mut self, value: $field_ty) -> Self {
                    self.$field = Some(value);
                    self
                }
            )*

            pub fn build(self) -> Result<$name, String> {
                Ok($name {
                    $( $field: self.$field.ok_or(
                        format!("字段 {} 未设置", stringify!($field))
                    )?, )*
                })
            }
        }
    };
}

struct_builder! {
    pub struct Connection {
        host: String,
        port: u16,
        timeout: u64,
    }
}

let conn = ConnectionBuilder::default()
    .host("localhost".into())
    .port(8080)
    .timeout(30)
    .build()
    .unwrap();
```

### 工程级宏：enum_str!

自动为枚举生成 `Display` 和 `FromStr` 实现：

```rust
macro_rules! enum_str {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident $(= $val:expr)?),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $vis enum $name { $($variant),* }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $( $name::$variant => write!(f, "{}", stringify!($variant)), )*
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $( stringify!($variant) => Ok($name::$variant), )*
                    _ => Err(format!("未知变体: {}", s)),
                }
            }
        }
    };
}

enum_str! { pub enum Status { Active, Inactive, Pending } }
let s: Status = "Active".parse().unwrap();
println!("{}", s);  // Active
```

---

## 四、代码层：过程宏

> 过程宏有一个**硬性要求**：必须放在独立的 proc-macro crate 中。编译器先编译 proc-macro crate（生成动态库），再在编译依赖它的 crate 时**加载并执行**这个过程宏。这是两阶段编译——宏代码和被它处理的代码不可能在同一个编译单元中。

### 三种类型

| 类型 | 用法 | 代表 | 本书展开 |
|------|------|------|:--:|
| **派生宏** | `#[derive(Serialize)]` | serde, thiserror | ✅ 详细 |
| **属性宏** | `#[tokio::main]` | tokio, rocket | 速览 |
| **函数式宏** | `sql!("SELECT ...")` | 自定义 DSL | 速览 |

### derive 宏实战：实现 `Hello` 派生宏

#### 项目结构

```
hello_example/
├── hello_derive/        # proc-macro crate（库）
│   ├── Cargo.toml
│   └── src/lib.rs
└── hello_main/          # 普通 crate（二进制）
    ├── Cargo.toml
    └── src/main.rs
```

#### 第 1 步：创建 proc-macro crate

`hello_derive/Cargo.toml`：
```toml
[package]
name = "hello_derive"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true          # 标记为过程宏 crate

[dependencies]
syn = { version = "2", features = ["full"] }
quote = "1"
proc-macro2 = "1"
```

`hello_derive/src/lib.rs`：
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn hello_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl #name {
            pub fn hello() {
                println!("你好！我是 {}", stringify!(#name));
            }
        }
    };

    expanded.into()
}
```

> `syn` 把输入 token 流解析成结构化 AST，`quote!` 用类声明式宏的语法生成输出代码。

#### 第 2 步：在主 crate 中使用

`hello_main/Cargo.toml`：
```toml
[dependencies]
hello_derive = { path = "../hello_derive" }
```

`hello_main/src/main.rs`：
```rust
use hello_derive::Hello;

#[derive(Hello)] struct Dog;
#[derive(Hello)] struct Cat;

fn main() {
    Dog::hello();   // 你好！我是 Dog
    Cat::hello();   // 你好！我是 Cat
}
```

`#[derive(Hello)]` 的流程：
1. 编译器收集 `Dog` 的 token 流，传给 `hello_derive` 函数
2. `syn` 解析出结构体名
3. `quote!` 拼出 `impl Dog { fn hello() {...} }` 代码
4. 返回的 token 流被"缝合"到原代码中，一起编译

### 另两种过程宏速览

**属性宏** (`#[my_attr]`)：修饰任意条目，可以读取并替换整个条目。

```rust,ignore
// 用法：#[my_attr(参数)] fn foo() {}
// 实现：#[proc_macro_attribute]
// pub fn my_attr(attr: TokenStream, item: TokenStream) -> TokenStream
```

**函数式宏** (`my_macro!()`)：像声明式宏一样调用，但用 Rust 代码处理 token 流。

```rust,ignore
// 用法：my_macro!(自定义语法)
// 实现：#[proc_macro]
// pub fn my_macro(input: TokenStream) -> TokenStream
```

---

## 五、实践层

### 宏编写步骤

1. **先写目标代码**——写出你期望宏展开后产生的代码
2. **替换可变部分为 `$name`**——找出哪些部分需要参数化
3. **逐步添加匹配规则**——一次只加一条规则，用 `cargo expand` 验证
4. **处理边界情况**——空输入、单个输入、多个输入

### 声明式宏 vs 过程宏 — 选择指南

| 选择 | 场景 |
|------|------|
| `macro_rules!` | 简单的模式替换、重复展开、短小精悍 |
| 派生宏 `#[derive]` | 自动生成 trait 实现（serde 风格） |
| 属性宏 `#[attr]` | 需要访问/修改被修饰项的 AST |
| 函数式宏 `macro!()` | 自定义 DSL 语法 |

声明式宏的优点：写在同一个文件、不需要额外 crate、学习成本低。先用声明式宏解决问题，有明确需求再引入过程宏。

### 常用内置宏速查

| 宏 | 用途 |
|----|------|
| `println!` / `format!` | 输出和格式化 |
| `vec!` | 创建 Vec |
| `assert!` / `assert_eq!` | 断言 |
| `dbg!` | 快速调试打印（含文件名和行号） |
| `todo!` / `unimplemented!` | 占位/未完成标记 |
| `include_str!` | 编译期嵌入文件内容为 `&str` |
| `stringify!` | token 序列转为字符串 |
| `concat!` | 拼接字符串字面量 |
| `cfg!` | 条件编译检查（返回 bool） |
| `file!` / `line!` / `column!` | 源码位置 |

### 常见陷阱

> ⚠️ **左递归导致展开死循环。** 如果宏的第一条规则匹配自身（直接或间接），编译器无限展开。用"内部规则"模式解决——递归调用用不同前缀。

> ⚠️ **匹配歧义。** 多条规则同时匹配时，编译器选**第一条匹配的**。把更具体的规则放前面，通用的放后面。

> ⚠️ **忘记卫生性例外。** 从调用者传入的标识符（`$name:ident`）会引用调用者作用域的名字。如果宏内部创建了同名变量，通过 identifier 传入的引用不受 hygiene 保护。

> ⚠️ **过度使用 `tt`。** `tt` 最灵活但也最容易出错。优先用 `expr`、`ident`、`ty` 等具体 specifier——编译器会提供更好的错误信息。

---

## 练习

1. 实现 `hash_map!` 宏，用 `{ key => value, ... }` 语法创建 HashMap
2. 实现 `debug_vars!`，接受多个标识符，打印每个的名字和值（包含文件名和行号）
3. 实现 `struct_builder!`：接受结构体名和字段名，生成 builder 模式代码
4. 研究标准库 `vec!` 宏的源码（搜索 `macro_rules! vec`），理解它为什么有两套匹配规则

---

← [第 18 章：属性与文档](./18-attributes-docs.md) | [返回目录](./README.md) | → [第 20 章：标准库集合](./20-collections.md)
