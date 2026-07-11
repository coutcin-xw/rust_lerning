# 第 11 章：闭包

## 学习目标

- 掌握闭包的三种捕获方式
- 理解 Fn / FnMut / FnOnce 的层次和选择
- 将闭包作为函数参数和返回值
- 使用 **async closures** 进行异步回调
- 知道何时用闭包，何时用函数

## 什么是闭包？

闭包是**可以捕获其定义环境中变量的匿名函数**：

```rust
let x = 10;
let y = 20;

// 闭包捕获了 x 和 y
let print_sum = || println!("{} + {} = {}", x, y, x + y);
print_sum();  // 10 + 20 = 30
```

vs 普通函数——函数不能捕获环境变量：

```rust
fn print_sum() {  // 无法访问外面的 x, y！
    // println!("{}", x);  // 编译错误
}
```

## 闭包语法

```rust
// 最简形式：类型从上下文推断
let add = |a, b| a + b;

// 显式类型标注
let add: fn(i32, i32) -> i32 = |a, b| a + b;              // 函数指针
let add = |a: i32, b: i32| -> i32 { a + b };              // 完整标注

// 无参数
let greet = || println!("Hello!");

// 多行代码块
let complex = |x: i32| {
    let y = x * 2;
    let z = y + 1;
    z * z
};

let result = add(1, 2);     // 3
let result = complex(5);    // 121
```

> 💡 闭包的类型由编译器为每个闭包单独生成（匿名类型）。即使两个闭包签名完全相同，它们的类型也不同——除非被强制转换为函数指针 `fn`。

## 三种捕获方式

闭包根据使用捕获变量的方式，自动选择最小权限的捕获方式：

```rust
let x = 10;
let mut y = 20;
let s = String::from("hello");

// 不可变借用（&T）—— 只读
let read_only = || println!("x={}", x);
read_only();    // x 被不可变借用
println!("{}", x);  // x 还能用

// 可变借用（&mut T）—— 修改
let mut modify = || {
    y += 1;
    println!("y={}", y);
};
modify();

// 获取所有权（move）—— 消费
let consume = move || {
    println!("{}", s);   // s 的所有权移入闭包
    // s 在闭包结束时被 drop
};
// println!("{}", s);  // ❌ s 已移动
consume();
```

## Fn / FnMut / FnOnce — 闭包的三种 trait

闭包自动实现以下 trait（取决于捕获方式）：

```
FnOnce  ←  FnMut  ←  Fn
(调用1次) (可改)   (只读)
```

| Trait | 捕获方式 | 可调用次数 | 典型场景 |
|-------|---------|-----------|---------|
| `FnOnce` | 获取所有权 | 至少一次 | `drop(s)`、线程 spawn |
| `FnMut` | 可变借用 | 多次 | `sort_by_key`、增量器 |
| `Fn` | 不可变借用 | 多次 | `map`、`filter` |

```rust
// Fn：只读捕获
let f: &dyn Fn() = &|| println!("read only");

// FnMut：可变捕获
let mut count = 0;
let mut f = || { count += 1; count };  // 隐式 FnMut

// FnOnce：消耗捕获
let s = String::from("hello");
let f = || drop(s);  // 隐式 FnOnce
```

### AsyncFn / AsyncFnMut / AsyncFnOnce

Rust 2024 Edition 引入了异步闭包的 trait 体系，与同步版本完全对应：

```
AsyncFnOnce  ←  AsyncFnMut  ←  AsyncFn
(调用1次)       (可改)         (只读)
```

| Trait | 捕获方式 | 可调用次数 | 对应同步 trait |
|-------|---------|-----------|---------------|
| `AsyncFnOnce` | 获取所有权 | 至少一次 | `FnOnce` |
| `AsyncFnMut` | 可变借用 | 多次 | `FnMut` |
| `AsyncFn` | 不可变借用 | 多次 | `Fn` |

异步闭包使用 `async || { ... }` 语法，返回一个 `Future`：

```rust
use std::future::Future;

// 异步闭包，不可变捕获
let factor = 10;
let async_calc = async |x: i32| -> i32 {
    // 模拟异步操作
    x * factor
};
// async_calc 实现了 AsyncFn(i32) -> i32
// 调用返回 impl Future<Output = i32>
let fut: impl Future<Output = i32> = async_calc(5);
```

> 💡 异步闭包在 Rust 1.85.0 稳定，需使用 2024 Edition。调用异步闭包不会立即执行——它返回一个 `Future`，需要 `.await` 或传递给执行器。

## 闭包作为参数

```rust
// 泛型方式（静态分发，零开销）
fn apply_twice<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(x))
}

let triple = |x| x * 3;
println!("{}", apply_twice(triple, 2));  // 2*3*3 = 18

// 动态分发
fn apply_and_log(f: Box<dyn Fn(i32) -> i32>, x: i32) -> i32 {
    let result = f(x);
    println!("f({}) = {}", x, result);
    result
}
```

## 闭包作为返回值

```rust
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y  // move 是必需的——x 在函数返回后就不存在了
}

let add5 = make_adder(5);
println!("5 + 3 = {}", add5(3));  // 8

// 如果需要返回不同类型的闭包，用 Box<dyn Fn>
fn make_closure(kind: &str) -> Box<dyn Fn(i32) -> i32> {
    match kind {
        "double" => Box::new(|x| x * 2),
        "square" => Box::new(|x| x * x),
        _        => Box::new(|x| x),
    }
}
```

## Async Closures（2024 Edition）

> 📘 *Async closures 于 Rust 1.85.0 稳定，可以捕获环境并返回 Future：*

```rust
// 捕获环境的异步闭包
let factor = 2;
let async_double = async |x: i32| -> i32 {
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    x * factor  // 捕获了 factor
};

let result = async_double(5).await;  // 10
```

对应的 trait：`AsyncFn`、`AsyncFnMut`、`AsyncFnOnce`（类比同步版本）。

## 闭包的实际应用

```rust
// 1. 迭代器组合
let evens_squared: Vec<i32> = (1..=10)
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .collect();

// 2. 自定义排序
let mut words = vec!["rust", "is", "awesome", "language"];
words.sort_by_key(|w| w.len());

// 3. 捕获上下文
let threshold = 5;
let big_numbers: Vec<_> = numbers.iter()
    .filter(|&&n| n > threshold)
    .collect();

// 4. 惰性求值
let expensive = |n: u64| {
    println!("正在计算...");
    (1..=n).sum::<u64>()
};
// 闭包不会立即执行，只在调用时执行
```

## 闭包性能

没有捕获变量的闭包是**零大小类型**（ZST），与函数指针性能相同：

```rust
let f: fn(i32) -> i32 = |x| x + 1;  // 可以强制为函数指针
// 没有堆分配，没有虚表，和直接调用函数一样快
```

捕获变量的闭包等效于编译器自动生成的结构体：
- 每个捕获的变量是该结构体的字段
- 调用闭包等于调用该结构体的方法
- 编译器可以**内联**闭包调用（就像内联普通函数）

## 闭包 vs 函数

| | 闭包 | 函数 |
|---|---|---|
| 捕获环境 | ✅ | ❌ |
| 类型 | 匿名类型 | 具名 `fn` 类型 |
| 作为参数 | 泛型（每处生成一份代码） | 函数指针（统一类型） |
| 适用场景 | 短小的内联逻辑 | 较大、需要复用的逻辑 |

**选择建议：** 优先用函数。当函数需要访问上下文数据，且逻辑简短（1-10 行）时，用闭包。

## 常见陷阱

> ⚠️ **闭包持有 `&mut` 时其他代码不能借用。** 如果闭包持有可变引用，在该闭包的生命周期内，原始变量不能被其他代码访问。

> ⚠️ **忘记 `move` 导致的生命周期错误。** 当闭包需要活得比捕获的引用更久时（如线程 spawn、返回闭包），必须用 `move`。

> ⚠️ **闭包类型不兼容。** 即使两个闭包有相同签名，类型也不同。需要统一类型时用 `Box<dyn Fn>` 或函数指针。

### Cacher 备忘录模式

> 📘 *这是 《Rust 程序设计语言》中的经典闭包模式——用结构体持有闭包和缓存结果。*

```rust
use std::collections::HashMap;

// 泛型 T 表示闭包的计算结果类型
struct Cacher<T>
where
    T: Fn(u32) -> u32,  // 闭包 trait 约束
{
    calculation: T,
    value: HashMap<u32, u32>,  // 缓存多个输入对应的结果
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        // 如果已有缓存，直接返回；否则计算并缓存
        *self.value.entry(arg).or_insert_with(|| {
            (self.calculation)(arg)
        })
    }
}
```

> 💡 `value` 字段的类型必须与闭包的返回类型一致（这里都是 `u32`），否则缓存无法存储计算结果。如果闭包返回不同类型，需将泛型拆分为两个类型参数。

## 练习

1. 用 `sort_by_key` + 闭包对一个 `Vec<(String, u32)>` 按数字降序排列
2. 实现 `Cacher<T>` 泛型结构体：存储一个闭包和计算结果，首次调用时计算，后续调用直接返回缓存值
3. 写一个 `make_counter` 函数，返回一个每次调用递增并返回新值的闭包
4. 尝试 async closure：写一个异步闭包，等待 100ms 后返回捕获的字符串的长度

---

← [第 10 章：生命周期](./10-lifetime.md) | [返回目录](./README.md) | → [第 12 章：迭代器](./12-iterators.md)
