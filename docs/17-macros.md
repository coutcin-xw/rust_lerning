# 第 17 章：宏

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

Rust 宏是**卫生的**——宏内部定义的变量不会污染外部作用域：

```rust
macro_rules! using_x {
    () => {
        let x = "宏内部的 x";
        println!("宏内: {}", x);
    };
}

let x = "外部的 x";
using_x!();            // 输出：宏内: 宏内部的 x
println!("外部: {}", x);  // 输出：外部: 外部的 x
// 两个 x 互不影响
```

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

过程宏需要**单独的 proc-macro crate**（`Cargo.toml` 中 `[lib] proc-macro = true`）：

### 三种类型

| 类型 | 用法 | 代表 |
|------|------|------|
| **派生宏** | `#[derive(Serialize)]` | serde, thiserror |
| **属性宏** | `#[tokio::main]` | tokio, rocket |
| **函数式宏** | `sql!("SELECT ...")` | 自定义 DSL |

派生宏示例（生成 trait 实现）：
```rust
use proc_macro::TokenStream;

#[proc_macro_derive(Hello)]
pub fn hello_derive(input: TokenStream) -> TokenStream {
    let output = quote! {
        impl Hello for #input_type {
            fn hello() { println!("Hello from derive macro!"); }
        }
    };
    output.into()
}
```

过程宏超出了本书范畴。如果你想深入了解，推荐阅读 dtolnay 的源码（如 `serde`, `thiserror`）。

## 练习

1. 实现 `hash_map!` 宏，用 `{ key => value, ... }` 语法创建 HashMap
2. 实现 `debug_vars!`，接受多个标识符，打印每个的名字和值（包含文件名和行号）
3. 实现 `struct_builder!`：接受结构体名和字段名，生成一个 builder 模式的代码
4. 研究标准库 `vec!` 宏的源码（在标准库中搜索 `macro_rules! vec`），理解它为什么有两套匹配规则

---

← [第 16 章：项目工程化](./16-project-engineering.md) | [返回目录](./README.md) | → [第 18 章：标准库集合](./18-collections.md)
