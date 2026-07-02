# 第 11 章：迭代器

## 学习目标

- 理解 Iterator trait 和迭代器的惰性本质
- 区分 `iter()`、`iter_mut()`、`into_iter()`
- 掌握常用适配器和消费者
- 实现自定义迭代器
- 理解迭代器的零成本抽象

## Iterator Trait

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // 还有数十个带默认实现的方法...
}
```

只需要实现 `next`，就能获得 `map`、`filter`、`collect` 等 70+ 个方法。

## 三种迭代方式

```rust
let v = vec![1, 2, 3];

for item in v.iter() { }        // 不可变引用迭代 &T
for item in v.iter_mut() { }    // 可变引用迭代 &mut T
for item in v.into_iter() { }   // 所有权迭代 T（消费 vec）
```

| 方法 | 产生 | 原集合 |
|------|------|--------|
| `iter()` | `&T` | 仍可用 |
| `iter_mut()` | `&mut T` | 仍可用 |
| `into_iter()` | `T` | 被消费 |

> 💡 `for` 循环会自动调用 `.into_iter()`，所以 `for x in &v` 等价于 `for x in v.iter()`。

## 适配器（Adapters）

适配器**惰性求值**——不会立即执行，返回一个新的迭代器：

```rust
let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// 链式组合
let result: Vec<i32> = v.iter()
    .filter(|&&x| x % 2 == 0)   // 只保留偶数
    .map(|&x| x * x)              // 平方
    .take(3)                      // 只取前 3 个
    .collect();                   // 收集到 Vec： [4, 16, 36]
```

常用适配器一览：

| 适配器 | 作用 |
|--------|------|
| `map` | 转换每个元素 |
| `filter` | 保留满足条件的元素 |
| `filter_map` | 同时过滤和转换 |
| `take(n)` | 取前 n 个 |
| `skip(n)` | 跳过前 n 个 |
| `take_while` | 取到条件不满足为止 |
| `skip_while` | 跳过直到条件不满足 |
| `chain` | 连接两个迭代器 |
| `zip` | 并行迭代两个迭代器 |
| `enumerate` | 添加索引 |
| `flatten` | 展平嵌套迭代器 |
| `flat_map` | 映射后展平 |
| `inspect` | 偷看每个元素（调试用） |

## 消费者（Consumers）

消费者**触发迭代**，实际执行计算并产生最终值：

```rust
// collect：收集到集合
let v: Vec<_> = (0..5).collect();

// fold：折叠/累加
let sum = (1..=100).fold(0, |acc, x| acc + x);  // 5050

// reduce：无初始值的 fold
let product = (1..=5).reduce(|a, b| a * b);  // Some(120)

// any / all：条件判断
let any_even = (1..10).any(|x| x % 2 == 0);  // true
let all_positive = (1..10).all(|x| x > 0);   // true

// find：查找
let first_big = (1..100).find(|&x| x > 50);  // Some(51)

// count：计数
let count = (1..100).filter(|x| x % 2 == 0).count();  // 49
```

## 自定义迭代器

```rust
struct Counter {
    count: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

let sum: usize = Counter::new(5).sum();  // 1+2+3+4+5 = 15
```

## 零成本抽象

Rust 的迭代器链在编译后被优化为等价的 **手写循环**，没有额外的中间分配或函数调用开销：

```rust
// 这段
let result: i32 = vec.iter().map(|x| x * 2).filter(|x| *x > 10).sum();

// 和这段手写循环生成的机器码几乎相同
// let mut result = 0;
// for x in &vec {
//     let double = x * 2;
//     if double > 10 { result += double; }
// }
```

## 常用模式

```rust
// 构建 HashMap
let pairs = vec![("a", 1), ("b", 2)];
let map: std::collections::HashMap<_, _> = pairs.into_iter().collect();

// 展平嵌套
let nested = vec![vec![1, 2], vec![3, 4]];
let flat: Vec<_> = nested.iter().flatten().collect();  // [1, 2, 3, 4]

// 分组
let words = vec!["apple", "banana", "apricot", "blueberry"];
let grouped: Vec<Vec<&&str>> = words.iter()
    .group_by(|w| w.chars().next())  // 按首字母
    .into_iter()
    .map(|(_, group)| group.collect())
    .collect();

// 无限迭代器
let first_five: Vec<i32> = (0..).map(|x| x * x).take(5).collect();
// [0, 1, 4, 9, 16]
```

> 📘 *在 Rust 2024 Edition 中，`Box<[T]>` 实现了 `IntoIterator`，对 `Box<[T]>` 调用 `.into_iter()` 会产出 `T`（而不是 `&T`）。*

## 跨语言对比

| 概念 | Rust | Java | Python | JS |
|------|------|------|--------|-----|
| 惰性求值 | ✅ 默认 | Stream API | Generator | ✅ |
| 收集 | `collect()` | `collect(Collectors.toList())` | `list()` | `Array.from()` |
| 性能 | 零成本 | 有装箱开销 | 解释执行 | JIT 优化 |

## 练习

1. 用迭代器写一个函数，接收 `Vec<String>`，返回所有长度 > 3 的单词的大写形式
2. 实现一个 `Fibonacci` 迭代器
3. 用 `fold` 计算字符串中每个字符的出现次数（返回 `HashMap<char, usize>`）

---

← [第 10 章：闭包](./10-closures.md) | [返回目录](./README.md) | → [第 12 章：并发编程](./12-concurrency.md)
