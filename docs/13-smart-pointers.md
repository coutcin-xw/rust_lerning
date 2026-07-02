# 第 13 章：智能指针

## 学习目标

- 理解每种智能指针的设计意图和使用场景
- 掌握 `Box<T>`、`Rc<T>`、`Arc<T>` 的所有权模型
- 理解 `Deref` 强制转换的工作方式
- 掌握 `Cell<T>` 和 `RefCell<T>` 的内部可变性
- 使用 `Cow<T>` 实现写时克隆优化
- 根据需求选择正确的智能指针

## 智能指针是什么？

智能指针是包装了原始指针的数据结构，提供额外的功能和保证：

| 普通引用 `&T` | 智能指针 |
|---|---|
| 只借用 | 可能拥有数据 |
| 编译期借用检查 | `RefCell` 运行时检查 |
| 无引用计数 | `Rc`/`Arc` 引用计数 |
| 无自动释放（借用不拥有） | `Drop` 自动释放 |

所有智能指针都实现了 `Deref`（使它们能像引用一样使用）和 `Drop`（离开作用域时自动清理）。

## 全景速查

| 智能指针 | 用途 | 线程安全 | 核心场景 |
|----------|------|---------|---------|
| `Box<T>` | 堆分配、递归类型 | ✅ | 编译期大小未知的类型 |
| `Rc<T>` | 引用计数、共享所有权 | ❌ | 单线程图/树形结构 |
| `Arc<T>` | Rc 的线程安全版 | ✅ | 多线程共享 |
| `Cell<T>` | 内部可变（Copy 类型） | ❌ | 小数据的内部修改 |
| `RefCell<T>` | 运行时借用检查 | ❌ | 共享可变数据（单线程） |
| `Cow<T>` | 写时克隆 | ✅ | 惰性分配 |

## Box\<T\> — 堆上分配

```rust
// 1. 在堆上存储简单数据（实际中很少这么用）
let five = Box::new(5);

// 2. 递归类型——编译器无法确定大小
enum List {
    Cons(i32, Box<List>),
    Nil,
}
// 没有 Box，List 的大小将是无限的（自引用）
// Box 将递归部分放在堆上，栈上只有一个指针

use List::{Cons, Nil};
let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

// 3. 只转移指针，不复制大数据
let big_data = Box::new([0u8; 1_000_000]);
// 赋值时只复制了 8 字节的指针，不是 1MB 数据

// 4. Trait 对象
let shapes: Vec<Box<dyn Draw>> = vec![
    Box::new(Button { label: String::from("OK") }),
    Box::new(TextField { placeholder: String::from("...") }),
];
```

`Box<T>` 的内存布局和 `&T` 一样——一个指针大小（8 字节在 64 位上）。区别在于 `Box` 拥有数据，离开作用域时释放。

## Rc\<T\> — 引用计数（单线程）

```rust
use std::rc::Rc;

let a = Rc::new(String::from("共享数据"));
let b = Rc::clone(&a);  // 引用计数 +1（不是深拷贝！）
let c = Rc::clone(&a);  // 引用计数 +1

println!("a 的引用计数: {}", Rc::strong_count(&a));  // 3

// 三个 Rc 指向同一块堆内存
// 当最后一个 Rc 离开作用域时，数据被释放
```

`Rc` 只允许不可变引用——因为多个所有者都持有相同数据，任何一个修改都会影响其他的。这就是为什么需要 `RefCell`。

### Weak\<T\> — 打破循环引用

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,     // Weak 不增加强引用计数
    children: RefCell<Vec<Rc<Node>>>,  // Rc 构成强引用
}

let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
});

let branch = Rc::new(Node {
    value: 1,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![Rc::clone(&leaf)]),
});

// 设置父节点（Weak 引用）
*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

// 访问父节点
if let Some(parent) = leaf.parent.borrow().upgrade() {
    println!("父节点值: {}", parent.value);  // 1
}
// branch 被 drop 后，parent 的 upgrade() 返回 None
```

## Arc\<T\> — 原子引用计数（多线程）

`Arc<T>` 是 `Rc<T>` 的线程安全版本。内部使用原子操作（而非普通加减）管理引用计数：

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3, 4, 5]);
let mut handles = vec![];

for i in 0..5 {
    let data = Arc::clone(&data);  // 原子地增加计数
    handles.push(thread::spawn(move || {
        println!("线程 {}: {:?} (长度={})", i, data, data.len());
    }));
}

for h in handles { h.join().unwrap(); }
```

> ⚠️ `Arc<T>` 和 `Rc<T>` 一样只提供不可变引用。需要多线程修改用 `Arc<Mutex<T>>` 或 `Arc<RwLock<T>>`。

## Deref — 像引用一样使用智能指针

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> { fn new(x: T) -> Self { MyBox(x) } }

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

let mb = MyBox::new(String::from("hello"));

// Deref 强制转换链：&MyBox<String> → &String → &str
fn say_hello(name: &str) { println!("Hello, {}!", name); }
say_hello(&mb);  // 自动转换！编译器插入了多次 deref 调用
```

Deref 强制转换发生在：
- `&T` 传给 `&U`（当 `T: Deref<Target=U>`）
- `&mut T` 传给 `&U` 或 `&mut U`（当 `T: DerefMut<Target=U>`）
- 方法调用时（`self` 自动借用）

## Drop — 自动清理

所有智能指针都实现了 `Drop`，在离开作用域时自动做清理：

```rust
struct CustomPointer { data: String }

impl Drop for CustomPointer {
    fn drop(&mut self) {
        println!("释放: {}", self.data);
    }
}

{
    let p = CustomPointer { data: String::from("test") };
}  // 自动打印 "释放: test"

// 手动提前释放
let p = CustomPointer { data: String::from("early") };
drop(p);  // 立即释放
// 注意：不是 p.drop()！编译器禁止直接调用
```

## Cell 和 RefCell — 内部可变性

"内部可变性" 让你即使在持有不可变引用时也能修改数据：

### Cell\<T\>（Copy 类型的内部可变）

```rust
use std::cell::Cell;

let x = Cell::new(42);
x.set(100);                // 整体替换
let val = x.get();         // 获取副本
println!("{}", val);       // 100

// 不需要 mut！x 自身是不可变的
```

### RefCell\<T\>（运行时借用检查）

```rust
use std::cell::RefCell;

let data = RefCell::new(vec![1, 2, 3]);

// 运行时检查借用规则（而非编译期）
{
    let mut v = data.borrow_mut();  // 可变借用
    v.push(4);
    v.push(5);
}  // borrow_mut 在此释放

let v = data.borrow();  // 不可变借用
println!("{:?}", v);    // [1, 2, 3, 4, 5]
```

> ⚠️ `RefCell` 在运行时检查借用规则。违反规则（如同时两个 `borrow_mut`）会导致 **panic**，不是编译错误。

### Rc + RefCell = 共享可变数据

```rust
use std::rc::Rc;
use std::cell::RefCell;

let shared = Rc::new(RefCell::new(5));

let a = Rc::clone(&shared);
let b = Rc::clone(&shared);

*a.borrow_mut() += 10;
*b.borrow_mut() *= 2;

println!("{}", shared.borrow());  // 30
```

## Cow\<T\> — 写时克隆

```rust
use std::borrow::Cow;

fn maybe_modify(input: &str) -> Cow<str> {
    if input.contains(' ') {
        Cow::Owned(input.replace(' ', "_"))  // 需要修改：分配新 String
    } else {
        Cow::Borrowed(input)                  // 不需修改：直接返回借用
    }
}

let s1 = maybe_modify("hello_world");    // Cow::Borrowed（无分配）
let s2 = maybe_modify("hello world");    // Cow::Owned（分配了新 String）
```

## 选择决策树

```
我需要...
├── 在堆上分配一个值？                              → Box<T>
├── 多个所有者（单线程）？                            → Rc<T>
├── 多个所有者（多线程）？                            → Arc<T>
├── 修改不可变结构体的内部数据？                       → Cell<T> 或 RefCell<T>
│   ├── 值是 Copy 类型？                            → Cell<T>
│   └── 值不是 Copy？                               → RefCell<T>
├── 共享可变状态？
│   ├── 单线程？                                     → Rc<RefCell<T>>
│   └── 多线程？                                     → Arc<Mutex<T>> 或 Arc<RwLock<T>>
└── 大部分时间借用、偶尔才拥有？                       → Cow<T>
```

## 练习

1. 用 `Box<T>` 实现一个二叉搜索树的插入和查找
2. 用 `Rc<RefCell<T>>` 实现一个双向链表节点（能获取前驱和后继）
3. 写一个接收 `&str` 并返回 `Cow<str>` 的函数，在字符串中有需要转义的字符时才分配新的 String
4. 实现一个简化版 `MyRc<T>`，理解引用计数的工作原理

---

← [第 12 章：并发编程](./12-concurrency.md) | [返回目录](./README.md) | → [第 14 章：异步编程](./14-async-await.md)
