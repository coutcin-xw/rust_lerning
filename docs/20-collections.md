# 第 20 章：标准库集合

## 学习目标

- 熟练使用 `Vec<T>` 和 `HashMap<K,V>` 的所有常用操作
- 理解 `String` 与 `&str` 的内存布局和使用选择
- 知道何时用 `BTreeMap` 而非 `HashMap`
- 了解其他集合类型的适用场景

## Vec\<T\> — 动态数组

`Vec<T>` 是 Rust 中使用最多的集合。它在堆上分配，可动态增长。

### 基本操作

```rust
// 创建
let mut v: Vec<i32> = Vec::new();
let mut v = vec![1, 2, 3];          // 宏
let v = vec![0; 100];                // 100 个零

// 增
v.push(4);                           // 末尾追加
v.insert(0, 0);                      // 指定位置插入

// 删
v.pop();                             // 弹出最后一个 → Option<T>
v.remove(0);                         // 移除指定位置（O(n)）
v.clear();                           // 清空
v.retain(|&x| x > 0);               // 保留满足条件的元素

// 访问
let third = v[2];                    // 索引访问（越界 panic）
let third = v.get(2);                // 返回 Option<&T>（安全）
let first = v.first();               // Option<&T>
let last = v.last_mut();             // Option<&mut T>
v.swap(0, 1);                        // 交换两个位置

// 大小
v.len();                             // 当前元素数
v.capacity();                        // 当前分配的容量
v.is_empty();                        // 是否为空
v.shrink_to_fit();                   // 缩容到 len

// 排序
v.sort();                            // 升序
v.sort_by(|a, b| b.cmp(a));          // 自定义比较器
v.sort_unstable();                   // 不稳定排序（更快）

// 批量操作
v.extend(&[4, 5, 6]);               // 追加迭代器内容
v.append(&mut other_vec);            // 移动另一个 Vec 的全部元素
v.split_off(3);                      // 从索引 3 分裂为两个 Vec

// 与切片互转
let slice: &[i32] = &v;             // Vec → &[T]（自动 deref）
let deref_slice: &[i32] = v.as_slice();  // 显式
```

### 遍历

```rust
// 各种遍历方式
for item in &v { }                   // 只读
for item in &mut v { *item += 1; }   // 修改
for (i, item) in v.iter().enumerate() { }  // 带索引
v.iter_mut().for_each(|x| *x *= 2);  // for_each 消费迭代器
```

### 内存和性能提示

```rust
// 预分配容量（避免多次重新分配）
let mut v = Vec::with_capacity(1000);

// 对大数据量的 push，预分配可以显著提升性能
// 因为避免了多次 realloc + 拷贝
```

## HashMap\<K, V\> — 键值存储

```rust
use std::collections::HashMap;

// 创建
let mut scores = HashMap::new();
scores.insert("Alice", 100);
scores.insert("Bob", 90);

// 从迭代器创建
let teams = vec!["Red", "Blue"];
let scores_vec = vec![10, 20];
let map: HashMap<_, _> = teams.into_iter().zip(scores_vec).collect();

// 访问
let alice_score = scores.get("Alice");     // Option<&V>
let alice_score = scores["Alice"];          // &V（不存在则 panic）

// Entry API —— 最强大的更新方式
scores.entry("Charlie")
    .or_insert(80);                         // 不存在则插入 80
scores.entry("Alice")
    .and_modify(|v| *v += 10)               // 存在则修改
    .or_insert(100);                        // 不存在则插入

// 移除
scores.remove("Bob");                       // 返回 Option<V>

// 遍历
for (key, value) in &scores { }             // 顺序不确定！
```

> ⚠️ HashMap 的迭代顺序是**不确定的**。需要有序用 `BTreeMap`。

### Entry API 进阶

```rust
use std::collections::hash_map::Entry;

// or_insert_with：惰性计算默认值（只在 key 不存在时调用闭包）
map.entry("expensive")
    .or_insert_with(|| {
        println!("计算默认值...");  // 只在 key 不存在时打印
        compute_default()
    });

// or_default：用 Default::default() 作为默认值
let mut counters: HashMap<&str, Vec<u32>> = HashMap::new();
counters.entry("events").or_default().push(42);

// occupied / vacant 手动分流
match map.entry("b") {
    Entry::Occupied(mut entry) => *entry.get_mut() += 1,
    Entry::Vacant(entry) => { entry.insert(42); }
}
```

> 💡 `or_insert_with` 优于 `or_insert(compute_default())`——前者只在 key 不存在时才调用函数，后者不管存不存在都先求值。

### 哈希函数与性能

`HashMap` 默认使用 `SipHash`（防 HashDoS 攻击的安全哈希）。在对安全性要求不高的场景，可切换到更快的哈希器：

```rust
// Cargo.toml: rustc-hash = "2"
use rustc_hash::FxHashMap;

let mut map: FxHashMap<&str, u32> = FxHashMap::default();
// 比标准 HashMap 快 2-3 倍，但不防 HashDoS
```

### HashMap vs BTreeMap

| | `HashMap` | `BTreeMap` |
|---|---|---|
| 底层 | 哈希表 | 平衡二叉搜索树 (B-Tree) |
| keys 要求 | `Hash + Eq` | `Ord` |
| 查找 | O(1) 平摊 | O(log n) |
| 迭代顺序 | 不确定 | 按 key 排序 |
| 适用 | 通用键值存储 | 需要有序遍历/范围查询 |

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert(3, "c");
map.insert(1, "a");
map.insert(2, "b");

for (k, v) in &map {
    println!("{}: {}", k, v);
}
// 输出（一定按 key 排序）：
// 1: a
// 2: b
// 3: c
```

## String 深入

### 内部结构

```
String（在栈上）:  [ptr][len][cap]   ← 24 字节 (64 位)
                      │
                      ▼
堆上:                [h][e][l][l][o]  ← UTF-8 编码的字节
```

### 常见操作

```rust
let mut s = String::new();
let s = String::from("hello");
let s = "hello".to_string();

// 追加
s.push_str(" world");
s.push('!');

// 拼接
let s = format!("{} {}", "hello", "world");   // 不影响参数
let s = "hello".to_owned() + " " + "world";   // 获取第一个的所有权

// 索引访问 —— 不能直接用 s[0]！
// 原因：UTF-8 中一个字符可能占 1-4 字节，O(1) 的字节索引语义模糊

// 替代方式：
for c in s.chars() { }              // 按字符迭代
for b in s.bytes() { }              // 按字节迭代
let first_char = s.chars().next();  // 第一个字符（O(1)）

// 切片（小心：必须在字符边界上！）
let hello = &s[0..5];  // 如果 [0..5] 恰好在字符边界上 → OK
// &s[0..1] 在包含多字节字符时可能 panic！
```

### `&str` 和 `String` 的转换

```rust
String → &str：自动（Deref），或用 .as_str()
&str  → String：.to_string(), .to_owned(), String::from()
```

## 其他集合速查

| 类型 | 描述 | 何时使用 |
|------|------|---------|
| `VecDeque<T>` | 双端队列 | 需要头尾高效增删（O(1)） |
| `LinkedList<T>` | 双向链表 | 极少使用；大部分场景 Vec 和 VecDeque 更快 |
| `BinaryHeap<T>` | 二叉最大堆 | 优先级队列 |
| `HashSet<T>` | 哈希集合 | 快速去重/判存在 |
| `BTreeSet<T>` | 有序集合 | 有序去重/范围查询 |

```rust
use std::collections::{VecDeque, BinaryHeap, HashSet};

// VecDeque
let mut deque = VecDeque::new();
deque.push_front(1);
deque.push_back(2);
deque.pop_front();  // Some(1)

// BinaryHeap
let mut heap = BinaryHeap::new();
heap.push(3); heap.push(1); heap.push(5);
while let Some(top) = heap.pop() {
    println!("{}", top);  // 5, 3, 1（从大到小）
}

// HashSet
let mut set = HashSet::new();
set.insert("apple");
set.insert("banana");
set.contains("apple");  // true
```

## 集合选型决策

按你的需求找到最合适的集合：

```
需要  键值对 ？
├── 是 → 需要有序遍历？
│   ├── 是 → BTreeMap
│   └── 否 → HashMap（考虑 FxHashMap 提速）
│
├── 只需要值（去重）？
│   ├── 是 → 需要有序？
│   │   ├── 是 → BTreeSet
│   │   └── 否 → HashSet
│   │
│   └── 需要队列顺序 → VecDeque
│
├── 需要优先级（最大/最小先出）→ BinaryHeap
│
├── 需要头尾双端操作 → VecDeque
│
└── 只需要动态数组 → Vec<T>（首选）
```

**性能口诀：**
- 90% 的场景用 `Vec`，键值访问用 `HashMap`
- 需要排序遍历用 `BTreeMap`/`BTreeSet`
- 头尾操作用 `VecDeque`（别用 `LinkedList`）
- 优先级队列用 `BinaryHeap`

## 练习

1. 统计一段文本中每个单词的出现次数（忽略大小写），用 HashMap 存储，按频次降序输出
2. 用 VecDeque 实现一个简单的循环缓冲区（固定大小，满了以后覆盖最旧的）
3. 用 BinaryHeap 实现一个 Top-K 问题：找到数组中第 K 大的元素

---

← [第 19 章：宏](./19-macros.md) | [返回目录](./README.md) | → [第 21 章：测试](./21-testing.md)
