# 第 12 章：迭代器

## 学习目标

- 理解 Iterator trait 和"惰性求值"的概念
- 区分 `iter()` / `iter_mut()` / `into_iter()`
- 熟练组合适配器和消费者构建数据处理管道
- 实现自定义迭代器
- 理解迭代器的零成本抽象保证

## 概念层：迭代器是什么，为什么重要

### 不只是"遍历数组"

迭代器是 Rust 中处理数据序列的**统一抽象**。它提供了一致的方式来遍历、转换、过滤和收集数据——无论数据源是数组、Vec、HashMap、文件行还是网络流：

```rust
// 传统 for 循环：告诉计算机"怎么做"
let mut result = Vec::new();
for &x in &data {
    if x > 0 {
        result.push(x * 2);
    }
}

// 迭代器：告诉计算机"做什么"
let result: Vec<_> = data.iter()
    .filter(|&x| *x > 0)
    .map(|&x| x * 2)
    .collect();
```

迭代器不替代 for 循环——它们是两种思维模式。但 Rust 生态强烈倾向于迭代器风格。

### 惰性求值（Laziness）— 最核心的概念

迭代器是**惰性**的：创建迭代器不会立即执行任何工作，只有在调用**消费者**（如 `collect`、`sum`、`for_each`）时才触发计算：

```rust
let iter = (0..).filter(|x| x % 2 == 0);  // 没有循环，什么都没发生
let first_five: Vec<_> = iter.take(5).collect();  // 现在才真正计算
// first_five = [0, 2, 4, 6, 8]
```

这意味着你可以用**零中间分配**的方式串联多个操作——每个元素一次性流过整条管道，不产生临时 Vec：

```rust
// 不会先创建 filter 结果的 Vec，再创建 map 结果的 Vec
// 而是每个元素依次：读取 → filter → map → 放入最终 Vec
let result: Vec<_> = (1..1000)
    .filter(|x| x % 2 == 1)   // 链式调用，无中间分配
    .map(|x| x * x)
    .collect();                // 只有这里触发计算
```

> 💡 惰性求值是 Rust 迭代器"零成本抽象"的核心——迭代器链编译后的机器码等价于手写循环。

### 迭代器与所有权

Rust 有三种迭代方式，对应不同的所有权需求：

| | `iter()` | `iter_mut()` | `into_iter()` |
|---|---|---|---|
| 产出 | `&T` | `&mut T` | `T` |
| 原集合 | 仍可用 | 仍可用 | **被消费**（所有权转移） |
| 场景 | 只读遍历 | 修改元素 | 转移所有权、消费后释放 |

这是 Rust 独有的设计——迭代器和所有权系统深度融合。

## 迭代器的核心：Iterator Trait

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // + 70 多个带默认实现的方法
}
```

只需要实现 `next` 一个方法，就能获得 `map`、`filter`、`collect` 等全部方法。

```rust
let v = vec![1, 2, 3];
let mut iter = v.iter();

assert_eq!(iter.next(), Some(&1));  // Some(&1)
assert_eq!(iter.next(), Some(&2));  // Some(&2)
assert_eq!(iter.next(), Some(&3));  // Some(&3)
assert_eq!(iter.next(), None);      // 迭代结束
```

## 三种迭代方式的区别

```rust
let mut v = vec![1, 2, 3];

// iter()     — 不可变引用 &T
for item in v.iter() {
    // item: &i32
}

// iter_mut() — 可变引用 &mut T
for item in v.iter_mut() {
    *item += 1;  // 修改原值
}

// into_iter() — 获取所有权 T（消费集合）
for item in v.into_iter() {
    // item: i32，v 在此后不可用
}
```

> 💡 `for` 循环自动调用 `.into_iter()`。`for x in &v` 等价于 `for x in v.iter()`，`for x in &mut v` 等价于 `for x in v.iter_mut()`。

## 适配器 — 惰性转换

适配器返回**新的迭代器**，不立即执行。只有调用消费者时才真正计算：

```rust
let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let pipeline = v.iter()
    .filter(|&x| x % 2 == 0)   // 保留偶数
    .map(|&x| x * x)             // 平方
    .take(3);                    // 前 3 个
// 到这里什么都没算！只是构建了迭代器链

let result: Vec<i32> = pipeline.collect();  // 现在才执行
// [4, 16, 36]
```

### 常用适配器速查

| 适配器 | 功能 | 示例 |
|--------|------|------|
| `map` | 转换每个元素 | `.map(\|x\| x * 2)` |
| `filter` | 保留符合条件的 | `.filter(\|x\| x > &0)` |
| `filter_map` | 同时过滤和转换 | `.filter_map(\|x\| x.parse().ok())` |
| `take(n)` | 取前 n 个 | `.take(5)` |
| `skip(n)` | 跳过前 n 个 | `.skip(3)` |
| `take_while` | 取到条件不满足 | `.take_while(\|x\| x < &100)` |
| `skip_while` | 跳过到条件不满足 | `.skip_while(\|x\| x < &10)` |
| `chain` | 连接两个迭代器 | `a.iter().chain(b.iter())` |
| `zip` | 并行配对 | `a.iter().zip(b.iter())` |
| `enumerate` | 添加索引 | `.enumerate()` → `(index, value)` |
| `flatten` | 展平一层嵌套 | `nested.iter().flatten()` |
| `flat_map` | map + flatten | `.flat_map(\|x\| x.split(','))` |
| `inspect` | 调试偷看 | `.inspect(\|x\| println!("{}", x))` |

## 消费者 — 触发计算

消费者**实际执行**迭代并产出结果：

```rust
let v: Vec<i32> = (1..=10).collect();           // collect：收集到集合
let s: i32 = (1..=100).sum();                    // sum：求和
let p: Option<i32> = (1..=5).reduce(|a,b| a*b); // reduce：无初值折叠
let total = (1..=100).fold(0, |acc, x| acc + x); // fold：有初值折叠
let has_even = (1..10).any(|x| x % 2 == 0);      // any：是否存在
let all_pos = (1..10).all(|x| x > 0);            // all：全部满足
let first = (1..100).find(|&x| x > 50);           // find：查找第一个
let count = (1..100).filter(|x| x % 2 == 0).count(); // count：计数
let (min, max) = (1..=10).fold((i32::MAX, i32::MIN), |(min, max), x| {
    (min.min(x), max.max(x))
});
```

### collect 的类型标注

```rust
let v: Vec<_> = (0..5).collect();           // Vec<i32>
let s: String = "hello".chars().collect();  // "hello"
let m: HashMap<_, _> = pairs.into_iter().collect();
```

## 自定义迭代器

```rust
struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(current)  // 无限迭代器——永远不会返回 None
    }
}

let fib: Vec<u64> = Fibonacci::new().take(10).collect();
// [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
```

## IntoIterator — 让自定义类型支持 for 循环

```rust
struct Stack {
    items: Vec<i32>,
}

impl IntoIterator for Stack {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

let stack = Stack { items: vec![1, 2, 3] };
for item in stack {  // 现在可以直接 for in 了
    println!("{}", item);
}
```

## 迭代器的零成本抽象

编译后的迭代器链**等价于手写循环**——没有额外分配，没有函数调用开销：

```rust
// 这段迭代器代码
let result: i32 = vec.iter()
    .map(|x| x * 2)
    .filter(|x| **x > 10)
    .sum();

// 编译后的机器码等同于这段手写循环
let mut result = 0;
for x in &vec {
    let doubled = x * 2;
    if doubled > 10 {
        result += doubled;
    }
}
```

## 常用迭代器模式

```rust
// 构建 HashMap
let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
let map: HashMap<_, _> = pairs.into_iter().collect();

// 展平嵌套
let nested = vec![vec![1, 2], vec![3, 4]];
let flat: Vec<&i32> = nested.iter().flatten().collect();  // [1,2,3,4]

// 无限迭代器
let powers: Vec<i32> = (0..).map(|n| 2_i32.pow(n)).take(8).collect();
// [1, 2, 4, 8, 16, 32, 64, 128]

// 字符串操作
let s = "hello world";
let upper_words: Vec<String> = s.split_whitespace()
    .map(|w| w.to_uppercase())
    .collect();
// ["HELLO", "WORLD"]

// 扫描/累积
let running_sum: Vec<i32> = (1..=5)
    .scan(0, |state, x| {
        *state += x;
        Some(*state)
    })
    .collect();
// [1, 3, 6, 10, 15]
```

## 选择建议：什么时候用什么迭代方式

| 需求 | 方式 |
|------|------|
| 只读遍历 | `&v` 或 `v.iter()` |
| 修改元素 | `&mut v` 或 `v.iter_mut()` |
| 消费集合 | `v.into_iter()` |
| 需要索引 | `.enumerate()` |
| 需要跳过/截断 | `.skip(n)`, `.take(n)` |
| 复杂转换链 | `.map().filter().collect()` |

## 常见陷阱

> ⚠️ **迭代器惰性——忘记消费。** `v.iter().map(...)` 什么都不做。必须调用 `collect()`、`for_each()`、`sum()` 等消费者。

> ⚠️ **在迭代中修改集合。** `for item in &v` 持有不可变借用，期间不能 `v.push()`。通过索引遍历或分两步（先收集结果，再修改）。

> ⚠️ **对 `iter()` 结果使用 `into_iter()`。** `v.iter()` 返回的是引用的迭代器，`into_iter()` 不会消费原集合。只有直接在 `v` 上调用 `into_iter()` 才会消费。

## 练习

1. 用迭代器实现：给定 `Vec<String>`，返回所有长度 > 5 的单词的大写形式，按字母序排列
2. 实现一个 `Primes` 迭代器：生成质数序列（无需高效算法，简单试除法即可）
3. 用 `fold` 实现：统计字符串中每个字符出现次数，返回 `HashMap<char, usize>`
4. 用迭代器改写一段使用传统 `for` 循环的命令式代码，对比可读性和性能

---

← [第 11 章：闭包](./11-closures.md) | [返回目录](./README.md) | → [第 13 章：并发编程](./13-concurrency.md)
