# 第 12 章：并发编程

## 学习目标

- 使用 `thread::spawn` 创建和管理线程
- 通过 `mpsc::channel` 在线程间传递消息
- 使用 `Mutex` 和 `Arc` 共享状态
- 理解 `Send` 和 `Sync` trait

## Rust 的并发哲学

Rust 以**无畏并发**（Fearless Concurrency）闻名。所有权和借用规则在编译期就能捕获大多数并发 bug（数据竞争、悬垂指针），让并发编程从"调试噩梦"变成"编译通过就跑对"。

> 不要通过共享内存来通信，要通过通信来共享内存。——Go 哲学，Rust 也拥抱这一理念。

## 创建线程

```rust
use std::thread;
use std::time::Duration;

let handle = thread::spawn(|| {
    for i in 1..5 {
        println!("子线程: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
});

// 主线程继续运行
for i in 1..3 {
    println!("主线程: {}", i);
    thread::sleep(Duration::from_millis(1));
}

handle.join().unwrap();  // 等待子线程结束
```

> ⚠️ 主线程结束时，所有子线程都会被强制终止，无论是否完成。务必 `join()` 等待。

### move 闭包与线程

```rust
let v = vec![1, 2, 3];

let handle = thread::spawn(move || {
    println!("子线程: {:?}", v);  // v 的所有权移入线程
});

// println!("{:?}", v);  // 编译错误！v 已被移动
```

`move` 是必需的——编译器无法确定线程是否比捕获的引用活得久。

## 消息传递

Rust 标准库提供了 **多生产者、单消费者**（mpsc）通道：

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();  // tx: 发送端, rx: 接收端

// 发送线程
thread::spawn(move || {
    let messages = vec!["你好", "来自", "子线程"];
    for msg in messages {
        tx.send(msg).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
});

// 接收：rx 作为迭代器
for received in rx {
    println!("收到: {}", received);
}
// 当所有 tx 被 drop 时，迭代自动结束
```

### 多个生产者

```rust
let (tx, rx) = mpsc::channel();
let tx2 = tx.clone();  // clone 发送端

thread::spawn(move || {
    tx.send("来自线程1").unwrap();
});

thread::spawn(move || {
    tx2.send("来自线程2").unwrap();
});

for msg in rx {
    println!("{}", msg);
}
```

## 共享状态

### Mutex — 互斥锁

```rust
use std::sync::Mutex;

let m = Mutex::new(5);

{
    let mut num = m.lock().unwrap();
    *num = 10;
}  // 离开作用域，锁自动释放（RAII）
```

> 💡 `lock()` 返回 `LockResult<MutexGuard<T>>`。`MutexGuard` 实现了 `Deref + Drop`，离开作用域时自动释放锁。

### Arc — 多线程引用计数

`Rc<T>` 不是线程安全的（非 `Send`）。在线程间共享数据需要 `Arc<T>`（原子引用计数）：

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);  // clone Arc（轻量，只增加计数）
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("结果: {}", *counter.lock().unwrap());  // 10
```

## Send 和 Sync

这两个 trait 是 Rust 并发安全的基础（都是自动实现的）：

| Trait | 含义 |
|-------|------|
| `Send` | 类型的所有权可以在线程间转移（几乎所有类型都是 `Send`） |
| `Sync` | 类型的引用可以在线程间共享（`&T` 是 `Send`） |

`Rc<T>` 不是 `Send`（内部使用非原子计数），`RefCell<T>` 不是 `Sync`（运行时借用检查非线程安全）。

## 同步原语速查

| 类型 | 用途 |
|------|------|
| `Mutex<T>` | 互斥锁，一次一个线程访问 |
| `RwLock<T>` | 读写锁，多读单写 |
| `Barrier` | 等待所有线程到达同一点 |
| `Condvar` | 条件变量，配合 Mutex 使用 |
| `Atomic*` | 原子类型（`AtomicBool`, `AtomicUsize` 等） |

## 并发最佳实践

> ⚠️ **优先使用消息传递而非共享内存。** 通道让数据流向更清晰，减少死锁风险。

> ⚠️ **Mutex 内的数据尽量简单。** 锁的粒度越细、持有时间越短，并发性能越好。

> ⚠️ **避免在持有锁时进行阻塞操作。** 这是死锁的常见原因。

## 跨语言对比

| 概念 | Rust | Go | Java | C++ |
|------|------|----|------|-----|
| 线程 | `thread::spawn` | goroutine | `Thread` | `std::thread` |
| 消息 | `mpsc::channel` | channel | `BlockingQueue` | 无标准库支持 |
| 互斥锁 | `Mutex<T>` | `sync.Mutex` | `synchronized` | `std::mutex` |
| 数据竞争 | 编译错误 ✅ | 运行时检测 | 运行时 | 未定义行为 ❌ |

## 练习

1. 创建 5 个线程，每个线程计算一个区间的素数，最后汇总结果
2. 用 mpsc channel 实现一个生产者-消费者模式
3. 用 `Arc<Mutex<Vec<T>>>` 实现一个线程安全的日志收集器

---

← [第 11 章：迭代器](./11-iterators.md) | [返回目录](./README.md) | → [第 13 章：智能指针](./13-smart-pointers.md)
