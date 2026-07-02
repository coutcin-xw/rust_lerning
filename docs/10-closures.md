# 第 10 章：闭包

## 学习目标

- 掌握闭包的基本语法
- 理解三种捕获方式：不可变借用、可变借用、获取所有权
- 理解 Fn / FnMut / FnOnce 的区别
- 学会将闭包作为参数和返回值
- 了解 **async closures**（2024 Edition）

## 闭包语法

闭包是**可以捕获其环境中变量的匿名函数**：

```rust
// 基本语法：|参数| { 表达式 }
let add = |a: i32, b: i32| -> i32 { a + b };

// 大部分时候类型可以推断
let add = |a, b| a + b;

// 无参数
let greet = || println!("Hello!");

// 多行
let complex = |x| {
    let y = x + 1;
    y * 2
};

let result = add(1, 2);  // 3
```

## 捕获环境变量

闭包可以从定义它的作用域中捕获变量：

```rust
let x = 10;
let y = 20;

// 不可变借用
let print = || println!("x={}, y={}", x, y);
print();
// x 和 y 仍然可用

// 可变借用
let mut count = 0;
let mut inc = || {
    count += 1;
    println!("count={}", count);
};
inc();
inc();
// 注意：inc 释放后可再用 count

// 获取所有权（move）
let name = String::from("hello");
let consume = move || {
    println!("{}", name);  // name 所有权移入闭包
};
// println!("{}", name);  // 编译错误！name 已移动
consume();
```

## Fn / FnMut / FnOnce

闭包根据捕获方式自动实现以下一个或多个 trait：

| Trait | 捕获方式 | 调用方式 |
|-------|---------|---------|
| `FnOnce` | 获取所有权（可能消耗捕获的值） | 只能调用一次 |
| `FnMut` | 可变借用 | 可多次调用，可能修改捕获值 |
| `Fn` | 不可变借用 | 可多次调用，不修改捕获值 |

继承关系：`Fn: FnMut: FnOnce`（实现了 `Fn` 的也实现了另外两个）

```rust
// FnOnce：消耗捕获的值
let s = String::from("hello");
let consume = || drop(s);  // s 被移入，闭包结束 s 就没了
consume();  // 只能调用一次

// FnMut：修改捕获的值
let mut v = vec![1, 2, 3];
let mut push = || v.push(4);  // 可变借用 v

// Fn：只读
let v = vec![1, 2, 3];
let count = || v.len();  // 不可变借用 v
```

## 闭包作为参数

使用 `impl` trait 或泛型：

```rust
// impl Trait 语法
fn apply<F>(f: F) where F: FnOnce() {
    f();
}

// 或者用 trait bound
fn process<F: Fn(i32) -> i32>(data: i32, f: F) -> i32 {
    f(data)
}

let triple = |x| x * 3;
let result = process(5, triple);  // 15
```

使用 `Box<dyn Fn>` 存储不同类型的闭包：

```rust
let operations: Vec<Box<dyn Fn(i32) -> i32>> = vec![
    Box::new(|x| x + 1),
    Box::new(|x| x * 2),
    Box::new(|x| x * x),
];
```

## 闭包作为返回值

```rust
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y  // move 是必须的，x 会离开作用域
}

let add5 = make_adder(5);
println!("{}", add5(3));  // 8
```

## Async Closures（2024 Edition）

> 📘 *Async closures 于 Rust 1.85.0 稳定，可以捕获环境变量并返回 Future：*

```rust
// 基本语法
let async_closure = async |x: i32| -> i32 {
    // 异步操作...
    x * 2
};

// 使用时需要 .await
let result = async_closure(5).await;

// 作为参数
async fn process<F>(f: F) -> i32
where
    F: AsyncFn(i32) -> i32,
{
    f(10).await
}
```

Async closures 有对应的 trait：`AsyncFn`、`AsyncFnMut`、`AsyncFnOnce`（类比同步闭包的 Fn 系列）。

## 闭包的实际应用

```rust
// 迭代器操作
let doubled: Vec<i32> = (1..5).map(|x| x * 2).collect();

// 自定义排序
let mut words = vec!["rust", "is", "awesome"];
words.sort_by_key(|w| w.len());

// 惰性求值
let expensive = |x| {
    println!("computing...");
    x * x
};

// 回调模式
fn on_event<F: Fn()>(callback: F) {
    callback();
}
```

## 闭包性能

没有捕获变量的闭包是**零大小类型**，可以通过函数指针传递：

```rust
let f: fn(i32) -> i32 = |x| x + 1;  // 等同于函数指针
```

捕获变量的闭包类似于编译器生成的匿名结构体，大小取决于捕获的内容。需要存储不同类型的闭包时，使用 `Box<dyn Fn(...)>`。

## 常见陷阱

> ⚠️ **闭包捕获引用后持有 Borrow。** 当闭包持有可变引用时，其他代码无法同时借用。这是借用规则的体现。

> ⚠️ **返回闭包时忘记 `move`。** 如果闭包引用了函数内的局部变量，必须用 `move` 将所有权转移到闭包中，因为局部变量在函数返回后就不存在了。

## 练习

1. 写一个函数，接收一个闭包和一个 `i32`，返回闭包执行 3 次的结果之和
2. 实现一个缓存器 `Cacher`：用结构体存储闭包和结果，首次调用时计算，后续直接返回缓存值
3. 用 async closure 写一个简单的异步回调处理

---

← [第 9 章：生命周期](./09-lifetime.md) | [返回目录](./README.md) | → [第 11 章：迭代器](./11-iterators.md)
