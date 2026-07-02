# 第 17 章：标准库集合

## 学习目标

- 熟练使用 `Vec<T>` 的各种操作
- 理解 `HashMap` 与 `BTreeMap` 的取舍
- 深入理解 `String` 与 `&str`
- 了解其他集合类型及其适用场景

## Vec\<T\> — 动态数组

Rust 中最常用的集合类型：

```rust
// 创建
let mut v: Vec<i32> = Vec::new();
let mut v = vec![1, 2, 3];       // 宏创建
let v = vec![0; 10];             // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

// 增删
v.push(4);                        // 追加
v.pop();                          // 弹出最后一个 → Some(4)
v.insert(0, 0);                   // 在索引 0 处插入 0
v.remove(0);                      // 移除索引 0 的元素

// 访问
let third = &v[2];               // 索引访问（越界会 panic）
let third = v.get(2);            // 返回 Option<&T>（安全）
let first = v.first();           // Some(&1)

// 遍历
for item in &v { }               // 不可变引用
for item in &mut v { *item += 1; }  // 可变引用
for (i, item) in v.iter().enumerate() { }

// 批量操作
v.extend(&[6, 7, 8]);            // 追加迭代器内容
v.sort();                        // 排序
v.sort_by(|a, b| b.cmp(a));      // 自定义排序
v.dedup();                       // 去重（相邻的）
let filtered: Vec<_> = v.iter().filter(|&&x| x > 2).collect();
```

> ⚠️ 在遍历时修改 Vec 会导致编译错误（借用规则）。需要修改就遍历索引或用 `split_at_mut`。

## String 与 &str

`String` 是可变的、堆分配的字符串；`&str` 是字符串切片（或字符串字面量）：

```rust
// 创建 String
let mut s = String::new();
let s = String::from("hello");
let s = "hello".to_string();

// 追加
s.push_str(" world");
s.push('!');

// 拼接
let s = format!("{} {}", "hello", "world");  // 不获取所有权
let s = "hello".to_owned() + " world";       // 获取第一个的所有权

// 索引：不能直接用 s[0]
// 因为 UTF-8 编码中，一个"字符"可能占多个字节
let chars: Vec<char> = s.chars().collect();  // 按 Unicode 字符
let bytes: Vec<u8> = s.bytes().collect();    // 按字节
```

> 💡 Rust 字符串是 UTF-8 编码，不能直接用索引访问（`s[0]` 不合法）。原因：UTF-8 的一个字符可能占用 1-4 个字节，索引操作在字节级别是 O(1)，但按字符是 O(n)——Rust 不允许隐藏 O(n) 的操作作为索引。

## HashMap\<K, V\> — 哈希表

```rust
use std::collections::HashMap;

// 创建
let mut scores = HashMap::new();
scores.insert("Alice", 100);
scores.insert("Bob", 90);

// 从迭代器创建
let teams = vec!["Red", "Blue"];
let scores = vec![10, 20];
let map: HashMap<_, _> = teams.into_iter().zip(scores).collect();

// 访问
let alice_score = scores.get("Alice");  // Some(&100)
let alice_score = scores["Alice"];      // 100（不存在则 panic）

// 插入与更新
scores.entry("Charlie")            // Entry API
    .or_insert(80);                // 不存在则插入 80
scores.entry("Alice")
    .and_modify(|v| *v += 10)      // 存在则修改
    .or_insert(100);
```

### HashMap vs BTreeMap

| | HashMap | BTreeMap |
|---|---|---|
| 底层 | 哈希表 | B-Tree |
| 顺序 | 无保证 | 按键排序 |
| 键要求 | `Hash + Eq` | `Ord` |
| 查找 | O(1) 均摊 | O(log n) |
| 内存 | 可能更大 | 紧凑 |
| 使用场景 | 通用 | 需要有序遍历 |

## 其他集合速查

| 类型 | 描述 | 使用场景 |
|------|------|---------|
| `VecDeque<T>` | 双端队列 | 需要头尾 O(1) 增删 |
| `LinkedList<T>` | 双向链表 | 频繁在中间插入（实际很少用） |
| `BinaryHeap<T>` | 二叉堆 | 优先队列（总是取出最大/最小） |
| `HashSet<T>` | 哈希集合 | 去重、快速判存在 |
| `BTreeSet<T>` | 有序集合 | 有序去重 |

```rust
use std::collections::{VecDeque, BinaryHeap, HashSet};

// VecDeque：双端操作 O(1)
let mut deque = VecDeque::new();
deque.push_front(1);
deque.push_back(2);
deque.pop_front();  // Some(1)

// BinaryHeap：最大堆
let mut heap = BinaryHeap::new();
heap.push(3);
heap.push(1);
heap.push(5);
heap.pop();  // Some(5)（总是最大）

// HashSet
let mut set = HashSet::new();
set.insert("apple");
set.insert("banana");
set.contains("apple");  // true
```

## 集合选择速查

```
需要动态数组？                                → Vec<T>
需要键值对 + 不关心顺序？                      → HashMap<K, V>
需要键值对 + 需要顺序遍历？                    → BTreeMap<K, V>
需要头尾都快？                                → VecDeque<T>
需要去重？                                    → HashSet<T>
需要优先队列？                                → BinaryHeap<T>
只需要借用/查看？                              → 切片 &[T]
```

## 练习

1. 统计一段文本中每个单词的出现次数（用 HashMap）
2. 用 VecDeque 实现一个简单的回文检查器
3. 用 BinaryHeap 找出 `Vec<i32>` 中第 K 大的元素

---

← [第 16 章：宏](./16-macros.md) | [返回目录](./README.md) | → [第 18 章：测试](./18-testing.md)
