# 第 12 章：并发编程

## 学习目标

- 使用 `thread::spawn` 创建线程，`join` 等待完成
- 通过 `mpsc::channel` 实现线程间消息传递
- 使用 `Arc<Mutex<T>>` 安全共享状态
- 理解 `Send` 和 `Sync` trait
- 树立"消息传递优先于共享内存"的并发理念

## Rust 的并发——编译期保证安全

在大多数语言中，并发 bug（数据竞争、死锁）是运行时调试的噩梦。Rust 的类型系统和借用规则**在编译期就捕获了数据竞争**。

> "无畏并发"（Fearless Concurrency）不是说并发容易写，而是说——一旦编译通过，就没有隐晦的并发 bug。

## 创建线程

```rust
use std::thread;
use std::time::Duration;

let handle = thread::spawn(|| {
    for i in 1..=5 {
        println!("子线程: {}", i);
        thread::sleep(Duration::from_millis(10));
    }
});

// 主线程继续执行
for i in 1..=3 {
    println!("主线程: {}", i);
    thread::sleep(Duration::from_millis(10));
}

handle.join().unwrap();  // 阻塞等待子线程完成
```

> ⚠️ 如果主线程结束，所有子线程被强制终止。**务必 join**。

### move 关键字——所有权转移进线程

```rust
let data = vec![1, 2, 3, 4, 5];

let handle = thread::spawn(move || {
    // data 的所有权移入此线程
    println!("子线程: {:?}", data);
});

// println!("{:?}", data);  // ❌ data 已移动
```

为什么必须 `move`？线程可能比创建它的作用域活得更久。如果闭包只借用了局部变量，当局部变量被释放时线程还在引用它——悬垂引用。`move` 强制闭包获取所有权，消除这种可能性。

## 消息传递——mpsc::channel

Rust 标准库提供**多生产者、单消费者**（mpsc）通道：

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();  // tx: 发送端(Sender), rx: 接收端(Receiver)

thread::spawn(move || {
    tx.send("来自子线程的消息").unwrap();
    tx.send("第二条消息").unwrap();
    // tx 在此离开作用域 → 通道关闭
});

// rx 作为迭代器——自动在通道关闭时结束
for msg in rx {
    println!("收到: {}", msg);
}
// 输出：
// 收到: 来自子线程的消息
// 收到: 第二条消息
```

### 多个生产者

```rust
let (tx, rx) = mpsc::channel();
let tx2 = tx.clone();  // 克隆发送端

thread::spawn(move || {
    for i in 1..=3 {
        tx.send(format!("线程1-消息{}", i)).unwrap();
    }
});

thread::spawn(move || {
    for i in 1..=3 {
        tx2.send(format!("线程2-消息{}", i)).unwrap();
    }
});

// 注意：原始的 tx 也还在！需要 drop 它让通道在合适时关闭
drop(tx);

for msg in rx {
    println!("{}", msg);
}
// 两个线程共发送 6 条消息
```

> 💡 `mpsc` = Multiple Producer, Single Consumer。只有一个接收端，但有多个发送端（通过 `clone`）。

## 共享状态——Mutex + Arc

### Mutex\<T\> — 互斥锁

```rust
use std::sync::Mutex;

let m = Mutex::new(5);

{
    let mut num = m.lock().unwrap();   // 获取锁
    *num += 1;
}  // MutexGuard 离开作用域 → 自动释放锁（RAII）
```

> ⚠️ `lock()` 返回 `LockResult<MutexGuard<T>>`。`unwrap()` 只在**另一个线程 panic 导致锁中毒**时失败——这种情况很少。

### 多线程共享——Arc\<Mutex\<T\>\>

`Mutex` 本身不能在线程间共享（所有权问题）。`Rc` 可以共享所有权但不线程安全。解决方案：`Arc<Mutex<T>>`。

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);  // clone Arc（只增加引用计数）
    handles.push(thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    }));
}

for handle in handles {
    handle.join().unwrap();
}

println!("最终计数: {}", *counter.lock().unwrap());  // 10
```

### 死锁示例（注意避免）

```rust
// ❌ 死锁风险：两个锁的获取顺序相反
let lock1 = Arc::new(Mutex::new(0));
let lock2 = Arc::new(Mutex::new(0));

// 线程 A：先 lock1 后 lock2
// 线程 B：先 lock2 后 lock1
// 如果交错执行 → 死锁
```

防范死锁：**始终按相同顺序获取锁**；尽量缩小锁的持有范围。

## Send 和 Sync Trait

这两个 trait 是 Rust 并发安全的基础——编译器自动为类型实现：

| Trait | 含义 | 反例 |
|-------|------|------|
| `Send` | 类型的**所有权**可以安全地转移到其他线程 | `Rc<T>`（非原子引用计数） |
| `Sync` | 类型的**共享引用**可以安全地在线程间传递 | `RefCell<T>`（运行时借用检查） |

```rust
// 大部分类型是 Send + Sync
fn is_send_sync<T: Send + Sync>() {}

is_send_sync::<i32>();       // ✅
is_send_sync::<String>();    // ✅
// is_send_sync::<Rc<i32>>();   // ❌ Rc 不是 Send
// is_send_sync::<RefCell<i32>>();  // ❌ RefCell 不是 Sync
```

## 其他同步原语

| 类型 | 用途 | 使用场景 |
|------|------|---------|
| `RwLock<T>` | 读写锁：多读单写 | 读多写少的场景 |
| `Barrier` | 所有线程到达同一点才继续 | 分阶段并行计算 |
| `Condvar` | 条件变量：等待某条件成立 | 生产者-消费者复杂场景 |
| `OnceLock<T>` | 只初始化一次的值 | 全局延迟初始化的配置 |
| `Atomic*` | 无锁原子操作 | 简单计数器、状态标志 |

## 并发模式选择指南

```
需要并行计算独立任务？      → thread::spawn + join
需要线程间通信？            → mpsc::channel（消息传递）
需要多个线程读写共享数据？   → Arc<Mutex<T>> 或 Arc<RwLock<T>>
只需要简单计数？            → AtomicU32 等（无锁，最快）
多读少写？                  → Arc<RwLock<T>>
分阶段并行（所有人到齐再继续）→ Barrier
```

## 练习

1. 创建 N 个线程，每个线程计算一个区间的素数个数，最后汇总
2. 用 mpsc channel 实现生产者-消费者：一个线程生成随机数，另一个线程接收并求和打印
3. 用 `Arc<Mutex<Vec<T>>>` 实现一个线程安全的日志记录器
4. 阅读 `std::sync::atomic` 文档，用 `AtomicUsize` 实现无锁计数器

---

← [第 11 章：迭代器](./11-iterators.md) | [返回目录](./README.md) | → [第 13 章：智能指针](./13-smart-pointers.md)
