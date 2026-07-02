# 第 13 章：智能指针

## 学习目标

- 掌握 `Box<T>`、`Rc<T>`、`Arc<T>` 的使用场景
- 理解 `Deref` 和 `Drop` trait
- 理解内部可变性：`Cell<T>` 和 `RefCell<T>`
- 了解 `Cow<T>` 写时克隆
- 能根据需求选择合适的智能指针

## 智能指针总览

| 指针 | 所有权 | 线程安全 | 用途 |
|------|--------|---------|------|
| `Box<T>` | 独占 | ✅ | 堆分配、递归类型、trait 对象 |
| `Rc<T>` | 共享（单线程） | ❌ | 单线程引用计数 |
| `Arc<T>` | 共享（多线程） | ✅ | 多线程引用计数 |
| `Cell<T>` | + 内部可变（Copy） | ❌ | `Copy` 类型的内部可变 |
| `RefCell<T>` | + 内部可变（任意） | ❌ | 运行时的借用检查 |
| `Cow<T>` | 借用或拥有 | ✅ | 写时克隆 |

## Box\<T\> — 堆分配

```rust
// 1. 在堆上存储数据
let b = Box::new(5);

// 2. 递归类型（编译期大小不定）
enum List {
    Cons(i32, Box<List>),
    Nil,
}

let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));

// 3. 转移大数据（只复制指针，不复制数据）
let big_data = Box::new([0u8; 1_000_000]);

// 4. Trait 对象
let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { radius: 2.0 }),
    Box::new(Rectangle { width: 3.0, height: 4.0 }),
];
```

## Rc\<T\> — 引用计数（单线程）

```rust
use std::rc::Rc;

let a = Rc::new(String::from("hello"));
let b = Rc::clone(&a);  // 只增加引用计数，不复制数据
let c = Rc::clone(&a);

println!("引用计数: {}", Rc::strong_count(&a));  // 3
```

### Weak\<T\> — 防止循环引用

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,     // 弱引用，避免循环
    children: RefCell<Vec<Rc<Node>>>,
}
```

> `Rc` 的循环引用会导致内存泄漏，用 `Weak<T>` 打破循环。`Weak::upgrade()` 返回 `Option<Rc<T>>`——若原数据已释放则返回 `None`。

## Arc\<T\> — 原子引用计数（多线程）

`Arc<T>` 就是 `Rc<T>` 的多线程版本，使用原子操作保证引用计数的线程安全：

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);
let mut handles = vec![];

for _ in 0..3 {
    let data = Arc::clone(&data);
    handles.push(thread::spawn(move || {
        println!("{:?}", data);
    }));
}
```

## Deref Trait — 解引用

`Deref` 让自定义类型可以用 `*` 解引用，同时支持 **Deref 强制转换**：

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self { MyBox(x) }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

let mb = MyBox::new(String::from("hello"));

// Deref 强制转换链：&MyBox<String> → &String → &str
fn hello(name: &str) { println!("Hello, {}!", name); }
hello(&mb);  // 自动转换！
```

## Drop Trait — 自动释放

类似 C++ 析构函数。智能指针通常都实现了 `Drop`，在离开作用域时自动清理：

```rust
struct CustomPointer { data: String }

impl Drop for CustomPointer {
    fn drop(&mut self) {
        println!("释放: {}", self.data);
    }
}

// 手动提前释放（不是 c.drop()！）
let c = CustomPointer { data: String::from("test") };
drop(c);  // 显式调用 std::mem::drop
```

## Cell 和 RefCell — 内部可变性

即使变量是不可变的，也可以通过 `Cell`/`RefCell` 修改内部值：

### Cell\<T\>（Copy 类型）

```rust
use std::cell::Cell;

let x = Cell::new(42);
let y = &x;  // 不可变引用
x.set(100);  // 但仍然能修改（因为 Cell 是整体替换）
println!("{}", x.get());  // 100
```

### RefCell\<T\>（运行时借用检查）

```rust
use std::cell::RefCell;

let data = RefCell::new(vec![1, 2, 3]);

// 运行时借用检查（而非编译期）
{
    let mut v = data.borrow_mut();  // 可变借用
    v.push(4);
}

println!("{:?}", data.borrow());  // 不可变借用：[1, 2, 3, 4]
```

> ⚠️ `RefCell::borrow_mut()` 在有活跃借用时会 **panic**（运行时）。编译期不报错，但违反规则会崩溃。

## Cow\<T\> — 写时克隆

```rust
use std::borrow::Cow;

fn process(input: &str) -> Cow<str> {
    if input.contains(' ') {
        Cow::Owned(input.replace(' ', "_"))  // 需要修改：分配新 String
    } else {
        Cow::Borrowed(input)  // 不需要修改：直接返回借用
    }
}
```

## 智能指针选择指南

```
需要堆分配？                                → Box<T>
多个所有者（单线程）？                       → Rc<T>
多个所有者（多线程）？                       → Arc<T>
需要修改不可变数据结构内部？                  → Cell<T> 或 RefCell<T>
  - 值是 Copy 类型？                        → Cell<T>
  - 值不是 Copy？                           → RefCell<T>
需要共享可变状态？                           → Rc<RefCell<T>> 或 Arc<Mutex<T>>
需要惰性修改（大部分时间借用，偶尔拥有）？     → Cow<T>
```

## 练习

1. 用 `Box<T>` 实现一个二叉搜索树
2. 用 `Rc<RefCell<T>>` 实现一个支持双向遍历的链表节点
3. 写一个接收 `&str` 并返回 `Cow<str>` 的函数，仅在需要时才分配新字符串

---

← [第 12 章：并发编程](./12-concurrency.md) | [返回目录](./README.md) | → [第 14 章：异步编程](./14-async-await.md)
