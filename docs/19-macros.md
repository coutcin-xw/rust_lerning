# 第 19 章：宏

## 学习目标

- 理解宏与函数的本质区别
- 编写声明式宏（`macro_rules!`）
- 掌握重复模式 `$(...),*` / `$(...);*`
- 了解过程宏的三种类型
- 建立"能用函数就不用宏"的判断力

## 宏 vs 函数

| | 宏 | 函数 |
|---|---|---|
| 调用语法 | `macro!()`（带感叹号） | `func()` |
| 参数 | 可变数量，多种语法形式 | 固定签名 |
| 执行时机 | 编译期展开为源代码 | 运行时执行 |
| 代码生成 | ✅ 可以生成 struct/fn/impl | ❌ |
| 可变参数 | ✅ | ❌ |

> 原则：**能用函数解决的问题，不要用宏。** 宏让代码难以调试、难以阅读、增加编译时间。只在函数做不到时用宏。

## 声明式宏（macro_rules!）

### 最简单的宏

```rust
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

say_hello!();  // 编译时展开为 println!("Hello, world!");
```

### 带参数的宏

```rust
macro_rules! create_function {
    // $name:ident 匹配一个标识符（函数名）
    ($name:ident) => {
        fn $name() {
            println!("函数 {:?} 被调用", stringify!($name));
            // stringify! 把标识符变成字符串
        }
    };
}

create_function!(foo);
create_function!(bar);

foo();  // 输出：函数 "foo" 被调用
bar();  // 输出：函数 "bar" 被调用
```

### Fragment Specifier —— 匹配不同类型的 token

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

### 重复模式 —— 宏真正的威力

```rust
macro_rules! my_vec {
    // $( $x:expr ),*  匹配零个或多个逗号分隔的表达式
    // $( , )?          可选末尾逗号
    ( $( $x:expr ),* $(,)? ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    };
}

let v = my_vec![1, 2, 3, 4, 5];
let v = my_vec![1, 2, 3,];  // 末尾逗号也可以
```

重复模式语法：
- `$( ... ),*` — 零个或多个，以逗号分隔
- `$( ... ),+` — 一个或多个，以逗号分隔
- `$( ... );*` — 零个或多个，以分号分隔
- 分隔符可以自定义（`,` `;` `=>` 等）

### 递归宏——实现计算器

```rust
macro_rules! calculate {
    (eval $e:expr) => { $e };
    (eval $e:expr, $( $tail:tt )*) => {
        calculate!(eval $e) + calculate!(eval $( $tail )*)
    };
}

let result = calculate!(eval 1, eval 2, eval 3);  // 6
```

### 实用的宏示例

```rust
// hash_map! —— 像 JSON 对象一样创建 HashMap
macro_rules! hash_map {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

let config = hash_map![
    "host" => "localhost",
    "port" => 8080,
];

// debug_vars! —— 打印多个变量名和值
macro_rules! debug_vars {
    ( $( $var:ident ),* ) => {
        $( eprintln!("{} = {:?}", stringify!($var), $var); )*
    };
}

let x = 42;
let name = "Alice";
debug_vars!(x, name);
// 输出：
// x = 42
// name = "Alice"
```

## 宏的卫生性

Rust 宏是**卫生的（hygienic）**——宏内部定义的变量不会与外部同名变量冲突：

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
// 两个 x 互不影响——这是编译器的 hygiene 机制
```

**但也有例外：** 通过 `$name:ident` 传入的标识符**会**与外部作用域交互，因为它们来自调用处。除此之外，宏内部自己定义的名字是隔离的。

## 机制层：宏展开如何工作

### 编译器阶段

```
源代码 → 词法分析(token) → macro_rules! 展开 → AST → 类型检查 → 代码生成
                            ↑
                      宏在这一步被"内联展开"
                      展开后的代码和其他代码一视同仁
```

宏匹配的是 **token tree**——不是字符串、不是 AST。这意味着：
- `1 + 2` 是三个 token：`1`, `+`, `2`
- `vec![1, 2]` 作为一个 `expr` fragment 整体匹配
- 宏不会匹配括号内部（除非用 `tt` 深入）

### 调试宏：cargo expand

```bash
cargo install cargo-expand
cargo expand                    # 展开当前 crate 的所有宏
cargo expand --lib              # 只展开 lib.rs
cargo expand some_module::func  # 只展开特定项
```

这是理解宏行为最有效的工具。写宏时，`cargo expand` 开着，写一条规则跑一次，立即看到展开结果。

### 内部规则模式（`@` 约定）

复杂宏通常拆成多个辅助"内部规则"，用 `@` 前缀标记私有模式：

```rust
macro_rules! sorted_vec {
    // 公开入口——接收普通参数
    ($($x:expr),* $(,)?) => {
        sorted_vec!(@inner [$($x),*])
    };

    // 内部规则——用 @inner 标记，不暴露给调用者
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
    // 基础情况：无内容标签
    ($tag:ident {}) => {
        format!("<{0}></{0}>", stringify!($tag))
    };

    // 递归情况：逐个吃掉属性
    ($tag:ident { $($rest:tt)* }) => {
        html!(@attrs $tag [] $($rest)*)
    };

    // 内部：累积属性
    (@attrs $tag:ident [$($attrs:tt)*] $key:ident = $val:expr, $($rest:tt)*) => {
        html!(@attrs $tag [$($attrs)* $key=$val] $($rest)*)
    };

    (@attrs $tag:ident [$($attrs:tt)*]) => {
        format!("<{} {:?}></{}>", stringify!($tag), vec![$($attrs)*], stringify!($tag))
    };
}

// 使用
let el = html!(div { class = "container", id = "main" });
```

## 代码层：更多实战宏

### struct_builder! — 生成 Builder 模式

```rust
macro_rules! struct_builder {
    // 入口：接收结构体名和字段定义
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($field_vis:vis $field:ident: $field_ty:ty),* $(,)?
        }
    ) => {
        // 原始结构体
        $(#[$meta])*
        $vis struct $name {
            $($field_vis $field: $field_ty),*
        }

        // 自动生成的 Builder
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
                    $(
                        $field: self.$field.ok_or(
                            format!("字段 {} 未设置", stringify!($field))
                        )?,
                    )*
                })
            }
        }
    };
}

// 使用
struct_builder! {
    pub struct Connection {
        host: String,
        port: u16,
        timeout: u64,
    }
}

// 宏展开后自动生成 ConnectionBuilder
let conn = ConnectionBuilder::default()
    .host("localhost".into())
    .port(8080)
    .timeout(30)
    .build()
    .unwrap();
```

### enum_str! — 自动为枚举生成 Display 和 FromStr

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
        $vis enum $name {
            $($variant),*
        }

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

enum_str! {
    pub enum Status {
        Active,
        Inactive,
        Pending,
    }
}

let s: Status = "Active".parse().unwrap();
println!("{}", s);  // Active
```

## 实践层：宏编写指南

### 调试宏的步骤

1. **先写目标代码**——写出你期望宏展开后产生的代码
2. **替换可变部分为 `$name`**——找出哪些部分需要参数化
3. **逐步添加匹配规则**——一次只加一条规则，用 `cargo expand` 验证
4. **处理边界情况**——空输入、单个输入、多个输入

### 常见陷阱

> ⚠️ **左递归导致展开死循环。** 如果宏的第一条规则匹配自身（直接或间接），编译器无限展开直到报错。用"内部规则"模式解决——递归调用用不同前缀。

> ⚠️ **匹配歧义。** 多条规则同时匹配时，编译器选**第一条匹配的**。把更具体的规则放前面，通用的放后面。

> ⚠️ **忘记卫生性例外。** 从调用者传入的标识符（`$name:ident`）会引用调用者作用域的名字。如果宏内部创建了同名变量，通过 identifier 传入的引用不受 hygiene 保护。

> ⚠️ **过度使用 `tt`。** `tt` 最灵活但也最容易出错。优先用 `expr`、`ident`、`ty` 等具体 specifier——编译器会提供更好的错误信息。

### 声明式宏 vs 过程宏

| 选择 | 场景 |
|------|------|
| `macro_rules!` | 简单的模式替换、重复展开、短小精悍 |
| 派生宏 `#[derive]` | 自动生成 trait 实现（serde 风格） |
| 属性宏 `#[attr]` | 需要访问/修改被修饰的项的 AST |
| 函数式宏 `macro!()` | 自定义 DSL 语法 |

声明式宏的优点：写在同一个文件、不需要额外 crate、学习成本低。

## 常用内置宏

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

## 过程宏简介

> 过程宏有一个**硬性要求**：必须放在独立的 proc-macro crate 中。这意味着你不能直接在 `main.rs` 或 `lib.rs` 里写过程宏——必须在另一个 crate 中定义，然后在主 crate 中引用。这是 Rust 编译器施加的限制。

### 三种类型

| 类型 | 用法 | 代表 |
|------|------|------|
| **派生宏** | `#[derive(Serialize)]` | serde, thiserror |
| **属性宏** | `#[tokio::main]` | tokio, rocket |
| **函数式宏** | `sql!("SELECT ...")` | 自定义 DSL |

属性宏和函数式宏较为进阶，本书只展开最常见的派生宏。

### derive 宏实战

下面实现一个 `Hello` 派生宏：给结构体添加 `hello()` 方法，打印结构体的名字。

#### 项目结构

```
hello_example/
├── hello_derive/        # proc-macro crate（库）
│   ├── Cargo.toml
│   └── src/lib.rs
└── hello_main/          # 普通 crate（使用宏的二进制项目）
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
    // 把输入解析为 AST
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;  // 结构体的名字

    // 用 quote! 生成代码
    let expanded = quote! {
        impl #name {
            pub fn hello() {
                println!("你好！我是 {}", stringify!(#name));
            }
        }
    };

    // 把 proc_macro2::TokenStream 转回 proc_macro::TokenStream
    expanded.into()
}
```

> `syn` 负责把输入 token 流解析成结构化 AST，`quote!` 用类似声明式宏的语法生成输出代码，`proc_macro2` 是跨平台的中间层。

#### 第 2 步：在主 crate 中使用

`hello_main/Cargo.toml`：

```toml
[package]
name = "hello_main"
version = "0.1.0"
edition = "2021"

[dependencies]
hello_derive = { path = "../hello_derive" }
```

`hello_main/src/main.rs`：

```rust
use hello_derive::Hello;

#[derive(Hello)]
struct Dog;

#[derive(Hello)]
struct Cat;

fn main() {
    Dog::hello();   // 输出：你好！我是 Dog
    Cat::hello();   // 输出：你好！我是 Cat
}
```

#### 运行

```bash
cd hello_main && cargo run
```

#### 发生了什么？

`#[derive(Hello)]` 告诉编译器："用 `hello_derive` 函数处理这个结构体的 token 流"。`hello_derive` 函数：
1. 接收 `TokenStream`（原始 token 序列）
2. 用 `syn` 解析出结构体名
3. 用 `quote!` 拼出 `impl` 块代码
4. 返回生成的 token 流，编译器把它"缝合"到原代码中

展开后等价于：

```rust
struct Dog;
impl Dog {
    pub fn hello() {
        println!("你好！我是 Dog");
    }
}
```

#### 关键概念：为什么必须是独立 crate？

编译器先编译 proc-macro crate（生成动态库），然后在编译依赖它的 crate 时**加载并执行**这个过程宏。这是两阶段编译——宏代码和被它处理的代码不可能在同一个编译单元中。

### 另两种过程宏速览

**属性宏** (`#[my_attr]`)：修饰任意条目（函数、结构体、模块等），可以读取并替换整个条目：

```rust
// 用法：#[my_attr] fn foo() {}
// 实现：#[proc_macro_attribute]
// pub fn my_attr(attr: TokenStream, item: TokenStream) -> TokenStream
```

**函数式宏** (`my_macro!()`)：像声明式宏一样调用，但用 Rust 代码处理 token 流：

```rust
// 用法：my_macro!(some custom syntax here)
// 实现：#[proc_macro]
// pub fn my_macro(input: TokenStream) -> TokenStream
```

过程宏的能力远超声明式宏，但成本也更高——学习曲线、编译时间、维护复杂度。先用声明式宏解决问题，有明确需求（如 `#[derive]`）再引入过程宏。

## 练习

1. 实现 `hash_map!` 宏，用 `{ key => value, ... }` 语法创建 HashMap
2. 实现 `debug_vars!`，接受多个标识符，打印每个的名字和值（包含文件名和行号）
3. 实现 `struct_builder!`：接受结构体名和字段名，生成一个 builder 模式的代码
4. 研究标准库 `vec!` 宏的源码（在标准库中搜索 `macro_rules! vec`），理解它为什么有两套匹配规则

---

← [第 18 章：属性与文档](./18-attributes-docs.md) | [返回目录](./README.md) | → [第 20 章：标准库集合](./20-collections.md)
