# 第 16 章：宏

## 学习目标

- 理解宏与函数的区别
- 编写声明式宏（`macro_rules!`）
- 掌握重复模式 `$(...),*`
- 了解过程宏的三种类型

## 宏 vs 函数

| | 宏 | 函数 |
|---|---|---|
| 调用方式 | `macro!()` | `func()` |
| 参数 | 可变数量、多种类型 | 固定签名 |
| 执行时机 | 编译期展开 | 运行时 |
| 代码生成 | ✅ | ❌ |
| 复杂度 | 高 | 低 |
| 调试 | 困难 | 容易 |

> 原则：**能用函数就不要用宏**。宏适用于减少样板代码、编译期代码生成、可变参数等场景。

## 声明式宏

### 基础语法

```rust
// 最简单的宏
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

say_hello!();  // 展开为 println!("Hello, world!");

// 带参数的宏
macro_rules! create_function {
    ($name:ident) => {
        fn $name() {
            println!("函数 {} 被调用", stringify!($name));
        }
    };
}

create_function!(foo);
foo();  // 输出：函数 foo 被调用
```

### Fragment Specifier（片段类型）

> 📘 *在 Rust 2024 Edition 中，`expr` fragment 现在也匹配 `const { }` 和 `_` 表达式。缺少 fragment specifier（如 `$x` 而非 `$x:expr`）现在是**硬错误**。*

| Specifier | 匹配 |
|-----------|------|
| `ident` | 标识符（变量名、函数名） |
| `expr` | 表达式 |
| `ty` | 类型 |
| `literal` | 字面量（`"hello"`, `42`） |
| `tt` | Token tree（最灵活） |
| `stmt` | 语句 |
| `block` | 代码块 `{ ... }` |
| `item` | 条目（函数、结构体等） |
| `vis` | 可见性修饰符 |

### 重复模式

```rust
macro_rules! vec_of_strings {
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push(format!("{}", $x));
            )*
            v
        }
    };
}

let v = vec_of_strings![1, 2, 3];
// v = ["1", "2", "3"]
```

语法说明：
- `$( ... ),*` — 匹配零个或多个逗号分隔的值
- `$( ... )*` — 对每个匹配重复展开
- `+` 替代 `*` 表示至少一个

### 实例：自定义 vec!

```rust
macro_rules! my_vec {
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

## 宏的卫生性

Rust 宏是**卫生的（hygienic）**——宏内部定义的变量不会与外部冲突：

```rust
macro_rules! create_var {
    () => { let x = 42; }
}

let x = 10;
create_var!();
println!("{}", x);  // 10（外部的 x 不受影响）
```

## 常用的内置宏

| 宏 | 用途 |
|----|------|
| `println!`, `format!` | 输出和格式化 |
| `vec!` | 创建 Vec |
| `assert!`, `assert_eq!` | 断言 |
| `stringify!` | 将 token 转为字符串 |
| `concat!` | 拼接字符串字面量 |
| `cfg!` | 条件编译检查 |
| `file!`, `line!`, `column!` | 源码位置（调试用） |
| `todo!`, `unimplemented!` | 占位标记 |
| `include_str!` | 编译期嵌入文件内容 |
| `dbg!` | 快速调试打印 |

### 实用自定义宏

```rust
// 计时宏
macro_rules! time_it {
    ($label:expr, $code:block) => {{
        let start = std::time::Instant::now();
        let result = $code;
        println!("{}: {:?}", $label, start.elapsed());
        result
    }};
}

let data = time_it!("数据处理", {
    // 耗时操作
    (0..1_000_000).sum::<u64>()
});

// 调试变量宏
macro_rules! debug_var {
    ($($var:ident),*) => {
        $( println!("{} = {:?}", stringify!($var), $var); )*
    };
}
```

## 过程宏简介

过程宏是更强大的宏，作为编译器插件运行：

| 类型 | 用法 | 示例 |
|------|------|------|
| 派生宏 | `#[derive(Debug)]` | 自动生成 trait 实现 |
| 属性宏 | `#[tokio::main]` | 修饰函数/结构体等 |
| 函数式宏 | `sql!` | 自定义语法 |

过程宏需要单独的 crate：

```toml
[lib]
proc-macro = true
```

```rust
use proc_macro::TokenStream;

#[proc_macro_derive(Hello)]
pub fn hello_derive(input: TokenStream) -> TokenStream {
    // 解析 input，生成新代码
    "fn hello() { println!(\"Hello from derive!\"); }".parse().unwrap()
}
```

## 常见陷阱

> ⚠️ **过度使用宏。** 宏会降低代码可读性、增加编译时间、难以调试。先用函数，确认不够用再考虑宏。

> ⚠️ **忘记 fragment specifier。** Rust 2024 Edition 中，`$x`（没有 `:expr` 等类型标注）是硬错误。以往只是 warning。

## 练习

1. 写一个 `hash_map!` 宏，用类似 JSON 的语法创建 HashMap
2. 写一个 `debug_vars!` 宏，接受多个变量名，打印每个变量的名和值
3. 研究 `serde` 的 `#[derive(Serialize, Deserialize)]`，理解派生宏的实际应用

---

← [第 15 章：模块系统](./15-modules.md) | [返回目录](./README.md) | → [第 17 章：标准库集合](./17-collections.md)
