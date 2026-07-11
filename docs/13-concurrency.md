# 第 13 章：并发编程

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

## Send 和 Sync — 编译期线程安全

这两个 trait 是 Rust 并发安全的基石，由编译器自动推导：

| Trait | 含义 | 自动实现条件 |
|-------|------|------------|
| `Send` | 类型的所有权可安全转移到其他线程 | 类型的**所有字段**都是 `Send` |
| `Sync` | 类型的共享引用 `&T` 可在线程间安全传递 | 类型的**所有字段**都是 `Sync` |

```rust
// 大部分标准库类型是 Send + Sync
fn assert_send_sync<T: Send + Sync>() {}

assert_send_sync::<i32>();         // ✅ 简单类型
assert_send_sync::<String>();      // ✅

// 不是 Send 的类型
// assert_send_sync::<Rc<i32>>();     // ❌ Rc 用非原子操作
// assert_send_sync::<*const i32>();  // ❌ 原始指针

// 不是 Sync 的类型
// assert_send_sync::<RefCell<i32>>(); // ❌ 运行时借用检查非线程安全
// assert_send_sync::<Cell<i32>>();    // ❌ 同上
```

**为什么 Rc 不是 Send？** `Rc` 的引用计数用的是普通 `+1/-1`，不是原子操作。如果两个线程同时 clone，计数可能错乱。`Arc` 用原子操作替代，所以 `Arc` 是 `Send`。

## 机制层：并发原语如何工作

### mpsc::channel 的内部

```rust
use std::sync::mpsc;

// 默认：无缓冲通道（同步通道）
let (tx, rx) = mpsc::channel();
// tx.send() 会阻塞直到 rx 接收数据——"会合"语义

// 有缓冲通道
let (tx, rx) = mpsc::sync_channel(10);
// tx.send() 只在缓冲区满时才阻塞

// 通道关闭机制：
// - 所有 tx（包括 clone）被 drop → 通道关闭
// - rx.recv() 返回 Err(RecvError)
// - rx 作为迭代器自然结束
```

### Mutex 内部和锁中毒

```rust
use std::sync::Mutex;

let m = Mutex::new(42);

// lock() 阻塞当前线程直到获取锁
let mut guard = m.lock().unwrap();

// MutexGuard<T> 实现了 Deref<Target=T> 和 DerefMut
*guard += 1;  // 直接修改内部值

// MutexGuard 实现了 Drop → 离开作用域自动释放锁
drop(guard);

// "锁中毒"（lock poisoning）：
// 如果另一个线程在持有锁时 panic，锁被标记为"中毒"
// 之后的 lock() 返回 Err(PoisonError)，你需要决定是否继续
match m.lock() {
    Ok(guard) => { /* 正常使用 */ }
    Err(poisoned) => {
        // 锁中毒了——数据可能处于不一致状态
        let guard = poisoned.into_inner();  // 仍然可以获取数据
        // 但你要小心使用
    }
}
```

### Arc 的原子引用计数

```rust
use std::sync::Arc;

let a = Arc::new(String::from("data"));
// 堆上： [ref_count: 1 (atomic)] [String 数据]
let b = Arc::clone(&a);
// 堆上： [ref_count: 2 (atomic)] — 原子递增
// a 和 b 各占 8 字节（一个指针）
drop(a);
// 堆上： [ref_count: 1 (atomic)] — 原子递减
drop(b);
// ref_count 归零 → 释放堆内存
```

## 代码层：更多并发原语示例

### Scoped Threads — 安全借用局部变量

```rust
use std::thread;

let data = vec![1, 2, 3, 4, 5];

thread::scope(|s| {
    // scope 内的所有线程保证在 scope 结束前全部 join

    s.spawn(|| {
        println!("线程1: {:?}", data);  // 直接借用！不需要 move
    });

    s.spawn(|| {
        println!("线程2: 长度={}", data.len());
    });

    // scope 结束 → 自动 join 所有线程
});

println!("所有线程完成，data 仍可用: {:?}", data);
// scope 保证：所有子线程结束时，data 还活着
```

> 💡 `thread::scope` (Rust 1.63+) 是 `thread::spawn` 的增强版。在 scope 内 spawn 的线程可以安全借用局部变量，编译器保证线程在 scope 结束前全部 join。

### RwLock — 读多写少

```rust
use std::sync::RwLock;

let cache = RwLock::new(HashMap::new());

// 多个读可以同时
let data = cache.read().unwrap();
let val = data.get("key").copied();
drop(data);  // 释放读锁

// 写独占
let mut data = cache.write().unwrap();
data.insert("key".to_string(), 42);
```

### Barrier — 同步点

```rust
use std::sync::{Arc, Barrier};
use std::thread;

let barrier = Arc::new(Barrier::new(3));  // 等待 3 个线程
let mut handles = vec![];

for i in 0..3 {
    let b = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        println!("线程 {}: 阶段1", i);
        b.wait();  // 阻塞，直到 3 个线程都到达
        println!("线程 {}: 阶段2（所有人到齐后继续）", i);
    }));
}
```

### Atomic — 无锁计数

```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

let counter = Arc::new(AtomicU64::new(0));
let mut handles = vec![];

for _ in 0..100 {
    let c = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        for _ in 0..1000 {
            c.fetch_add(1, Ordering::SeqCst);  // 原子操作，无需 Mutex
        }
    }));
}

for h in handles { h.join().unwrap(); }
println!("{}", counter.load(Ordering::SeqCst));  // 100000
```

`Ordering` 控制内存顺序保证：`SeqCst` 最严格（默认选择），`Relaxed` 最宽松（仅保证原子性，不保证顺序）。初期用 `SeqCst`，性能敏感时再调整。

## 实践层：并发模式

### Worker Pool 模式

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            let rx = Arc::clone(&rx);
            workers.push(Worker {
                id,
                thread: Some(thread::spawn(move || loop {
                    let job = rx.lock().unwrap().recv();
                    match job {
                        Ok(job) => job(),
                        Err(_) => break,  // 通道关闭，退出
                    }
                })),
            });
        }

        ThreadPool { workers, sender: Some(tx) }
    }

    fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());  // 关闭通道 → worker 线程退出
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

### Fan-out / Fan-in 模式

多个 worker 并行处理数据，结果收集到一个通道：

```rust
let (result_tx, result_rx) = mpsc::channel();

// Fan-out: 分发任务到多个线程
for chunk in data.chunks(100) {
    let chunk = chunk.to_vec();
    let tx = result_tx.clone();
    thread::spawn(move || {
        let result = process_chunk(&chunk);
        tx.send(result).unwrap();
    });
}
drop(result_tx);  // 关闭发送端

// Fan-in: 收集结果
let results: Vec<_> = result_rx.iter().collect();
```

## 并发模式选择指南

```
模式                                    适用场景                原语
──────────────────────────────────────────────────────────────────────
独立任务并行                           计算密集型              thread::spawn + join
线程间发送消息                         解耦的生产者-消费者      mpsc::channel
共享可变状态                           缓存/配置               Arc<Mutex<T>>
读多写少                               缓存读取                Arc<RwLock<T>>
简单计数/标志                          统计/退出信号           Atomic*
分阶段并行                             所有线程到齐再继续      Barrier
等待条件通知                           等待资源就绪            Condvar
延迟初始化                             全局单例               OnceLock / Once
安全借用局部变量到线程中               临时多线程              thread::scope
任务队列                               工作池                 channel + worker loop
```

## 练习

1. 创建 N 个线程并行计算不同区间的素数，汇总到 `Arc<Mutex<Vec<u64>>>`
2. 用 mpsc channel 实现生产者-消费者：3 个生产者生成随机数，1 个消费者汇总
3. 实现一个简化版 `ThreadPool`，支持 `pool.execute(|| { ... })`
4. 用 `AtomicU64` 实现无锁计数器，benchmark 对比 `Arc<Mutex<u64>>` 的性能差异

---

← [第 12 章：迭代器](./12-iterators.md) | [返回目录](./README.md) | → [第 14 章：智能指针](./14-smart-pointers.md)
