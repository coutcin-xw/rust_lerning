// ============================================================
// 09_concurrency.rs - 并发编程
// ============================================================
//
// 【核心概念】
// Rust的所有权系统让并发编程更安全
// "Fearless Concurrency" - 无畏并发
//
// 【对比其他语言】
// - Go: goroutine + channel，简单但运行时重
// - Java: Thread + synchronized，容易出错
// - C++: std::thread + mutex，容易数据竞争
// - Erlang: Actor模型，安全但需要学习
// - Rust: 所有权保证线程安全，编译期检查
//
// 【Rust并发模型】
// 1. 线程（thread） - 标准库原生支持
// 2. 消息传递（channel） - 类似Go
// 3. 共享状态（Mutex） - 类似传统并发
// 4. async/await - 异步编程
//
// 【核心保证】
// 编译器防止数据竞争
// Send trait: 类型可以安全地在线程间移动
// Sync trait: 类型可以安全地在线程间共享引用
// ============================================================

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// --- 创建线程 ---
// Rust使用1:1线程模型（一个OS线程对应一个Rust线程）

fn spawn_thread() {
    println!("=== 创建线程 ===");

    // 【thread::spawn】
    // 创建新线程，接收闭包
    let handle = thread::spawn(|| {
        // 闭包在新线程中执行
        for i in 1..5 {
            println!("子线程: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 主线程继续执行
    for i in 1..5 {
        println!("主线程: {}", i);
        thread::sleep(Duration::from_millis(100));
    }

    // 【join】
    // 等待线程完成
    // 不调用join，主线程结束后子线程会被强制终止
    handle.join().unwrap();

    // 【线程的执行顺序】
    // 线程调度由OS决定，执行顺序不确定
    // 每次运行输出可能不同
}

// --- 线程中使用move ---
fn move_closure() {
    println!("=== move闭包 ===");

    let v = vec![1, 2, 3];

    // 【问题】
    // 闭包默认借用环境变量
    // 但线程可能在变量释放后还在运行
    //
    // let handle = thread::spawn(|| {
    //     println!("{:?}", v);  // 错误！v可能已经无效
    // });

    // 【解决：move关键字】
    // move把变量的所有权转移到闭包中
    let handle = thread::spawn(move || {
        // v的所有权已经移动到这个闭包
        println!("子线程中: {:?}", v);
    });

    // v已经移动，这里不能使用
    // println!("{:?}", v);  // 错误！

    handle.join().unwrap();

    // 【何时需要move】
    // 1. 在线程中使用外部变量
    // 2. 闭包生命周期超过变量作用域
    // 3. 需要转移所有权
}

// --- 线程间通信 - 消息传递 ---
// Go名言："不要通过共享内存来通信，要通过通信来共享内存"

fn message_passing() {
    println!("=== 消息传递 ===");

    // 【mpsc::channel】
    // mpsc: multiple producer, single consumer
    // 多生产者，单消费者
    let (tx, rx) = mpsc::channel();

    // 【发送消息】
    // tx 是发送端（transmitter）
    // rx 是接收端（receiver）
    thread::spawn(move || {
        let msg = String::from("hello from thread");
        tx.send(msg).unwrap();
        // send 返回 Result<T, E>
        // 如果接收端已关闭，send 会返回 Err

        // 【注意】发送后 msg 的所有权转移
        // println!("{}", msg);  // 错误！msg已移动
    });

    // 【接收消息】
    // recv: 阻塞等待，返回 Result<T, E>
    let received = rx.recv().unwrap();
    println!("收到: {}", received);

    // 【迭代接收】
    // rx 可以当作迭代器
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        for i in 0..5 {
            tx2.send(i).unwrap();
        }
        // 发送端在作用域结束时关闭
    });

    // 迭代接收，直到发送端关闭
    for received in rx2 {
        println!("收到: {}", received);
    }

    // 【try_recv】
    // 非阻塞接收，立即返回
    // Ok(v) - 有消息
    // Err(mpsc::TryRecvError::Empty) - 无消息
    // Err(mpsc::TryRecvError::Disconnected) - 发送端已关闭
}

// --- 多个生产者 ---
fn multiple_producers() {
    println!("=== 多个生产者 ===");

    let (tx, rx) = mpsc::channel();

    // 【clone发送端】
    // 创建多个发送端
    let tx1 = tx.clone();
    thread::spawn(move || {
        tx1.send("from thread 1").unwrap();
    });

    let tx2 = tx.clone();
    thread::spawn(move || {
        tx2.send("from thread 2").unwrap();
    });

    // 主线程可以保留原始tx
    // 这里我们不用，所以drop掉
    drop(tx);

    // 接收多个消息
    for _ in 0..2 {
        println!("{}", rx.recv().unwrap());
    }

    // 【消息顺序】
    // 消息到达顺序不确定
    // 取决于线程调度
}

// --- 共享状态 - Mutex ---
// Mutex = Mutual Exclusion（互斥锁）

fn shared_state() {
    println!("=== 共享状态（Mutex） ===");

    // 【问题】
    // 多个线程需要修改同一数据
    // 需要同步机制防止数据竞争

    // 【Arc = Atomic Reference Counting】
    // 原子引用计数，线程安全的Rc
    // 必须用Arc，Rc不是线程安全的
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        // 【clone Arc】
        // 增加引用计数
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // 【lock】
            // 获取锁，返回 MutexGuard
            // 如果锁被占用，线程阻塞
            let mut num = counter.lock().unwrap();
            // unwrap 处理锁可能被污染的情况（持有锁的线程panic）

            *num += 1;
            // MutexGuard 在作用域结束时自动释放锁
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 最终结果
    println!("最终计数: {}", *counter.lock().unwrap());

    // 【Mutex的工作原理】
    // 1. lock() 获取锁
    // 2. 同一时间只有一个线程能获取锁
    // 3. MutexGuard 提供 &mut T 访问
    // 4. 离开作用域自动释放锁（RAII）

    // 【为什么需要Arc？】
    // Mutex需要被多个线程共享
    // Arc提供线程安全的共享所有权
}

// --- 更多同步原语 ---
fn more_sync_primitives() {
    println!("=== 更多同步原语 ===");

    use std::sync::Barrier;
    use std::sync::RwLock;

    // 【RwLock - 读写锁】
    // 允许多个读者或一个写者
    let rw = RwLock::new(5);

    {
        // 多个读者可以同时持有读锁
        let r = rw.read().unwrap();
        println!("读取: {}", *r);
    }

    {
        // 写锁是排他的
        let mut w = rw.write().unwrap();
        *w = 10;
    }

    // 【Barrier - 屏障】
    // 让所有线程等待，直到都到达某一点
    let barrier = Arc::new(Barrier::new(3)); // 等待3个线程
    let mut handles = vec![];

    for i in 0..3 {
        let b = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("线程 {} 准备就绪", i);
            b.wait(); // 等待所有线程
            println!("线程 {} 开始执行", i);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    // 【其他同步原语】
    // - Condvar: 条件变量
    // - Atomic类型: 原子操作
    // - Once: 单次执行
    // - parking_lot crate: 更高效的锁实现
}

// --- Send和Sync Trait ---
// 这两个trait是Rust线程安全的基石

// 【Send Trait】
// 表示类型可以安全地在线程间转移所有权
// 几乎所有类型都自动实现Send
//
// 例外：
// - Rc<T> 不是 Send（引用计数不是原子的）
// - Raw pointer 不是 Send
//
// 手动标记为非Send：
// struct NotSend(*mut i32);
// impl !Send for NotSend {}

// 【Sync Trait】
// 表示类型可以安全地在线程间共享引用（&T是Send）
// 等价于：&T 是 Send
//
// 大多数类型自动实现Sync
//
// 例外：
// - Cell<T>, RefCell<T> 不是 Sync（内部可变性）
// - Rc<T> 不是 Sync

// 【手动实现】
// 通常不需要手动实现，编译器自动推导
// 如果需要手动实现，使用unsafe
//
// unsafe impl<T: Send> Send for MyWrapper<T> {}
// unsafe impl<T: Sync> Sync for MyWrapper<T> {}

// --- RefCell和Rc（单线程）---
fn refcell_rc() {
    println!("=== RefCell和Rc（单线程） ===");

    use std::cell::RefCell;
    use std::rc::Rc;

    // 【Rc - Reference Counting】
    // 引用计数的智能指针
    // 只能用于单线程！
    let data = Rc::new(RefCell::new(5));

    // clone增加引用计数
    let d1 = Rc::clone(&data);
    let d2 = Rc::clone(&data);

    // 【RefCell - 内部可变性】
    // 允许通过不可变引用修改数据
    // 运行时检查借用规则（而不是编译期）
    *d1.borrow_mut() += 10;
    *d2.borrow_mut() += 10;

    println!("结果: {:?}", data); // 25

    // 【Rc vs Arc】
    // Rc: 单线程，性能更好
    // Arc: 多线程，原子操作有开销

    // 【RefCell vs Mutex】
    // RefCell: 单线程，运行时检查
    // Mutex: 多线程，线程安全

    // 【组合使用】
    // Rc<RefCell<T>> - 单线程共享可变数据
    // Arc<Mutex<T>> - 多线程共享可变数据
}

// --- 练习：生产者-消费者 ---
fn producer_consumer() {
    println!("=== 生产者-消费者模式 ===");

    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    // 【生产者1】
    thread::spawn(move || {
        for i in 0..5 {
            let msg = format!("生产者A-{}", i);
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    // 【生产者2】
    thread::spawn(move || {
        for i in 0..5 {
            let msg = format!("生产者B-{}", i);
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_millis(30));
        }
    });

    // 【消费者】
    // 接收所有消息
    for _ in 0..10 {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(msg) => println!("消费: {}", msg),
            Err(_) => {
                println!("超时或通道关闭");
                break;
            }
        }
    }

    // 【设计要点】
    // 1. 生产者和消费者解耦
    // 2. 缓冲区（channel）平衡生产/消费速度差异
    // 3. 错误处理：超时、通道关闭
}

// --- 并发最佳实践 ---
// 【总结】
// 1. 优先使用消息传递（channel）
// 2. 避免共享状态
// 3. 如果必须共享，使用Mutex保护
// 4. 记得join线程
// 5. 处理错误（unwrap不是好的选择）

// 【避免的问题】
// 1. 死锁 - 两个线程互相等待
// 2. 竞态条件 - 依赖执行顺序
// 3. 饥饿 - 线程永远无法获取资源
// 4. 活锁 - 线程持续响应但无法推进

pub fn run() {
    println!("\n========== 09_concurrency ==========");
    spawn_thread();
    move_closure();
    message_passing();
    multiple_producers();
    shared_state();
    more_sync_primitives();
    refcell_rc();
    producer_consumer();
}
