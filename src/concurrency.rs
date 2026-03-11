// ============================================================
// 09_concurrency.rs - 并发编程
// Rust的所有权系统让并发编程更安全
// 对比: Go goroutine / Java Thread -> Rust threads/async
// ============================================================

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// --- 创建线程 ---
fn spawn_thread() {
    // spawn创建新线程
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("子线程: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 主线程
    for i in 1..5 {
        println!("主线程: {}", i);
        thread::sleep(Duration::from_millis(100));
    }

    // 等待子线程结束
    handle.join().unwrap();
}

// --- 线程中使用move ---
fn move_closure() {
    let v = vec![1, 2, 3];

    // move关键字：把v的所有权转移到闭包中
    let handle = thread::spawn(move || {
        println!("子线程中: {:?}", v);
    });

    // 这里v已经无效
    // println!("{:?}", v);  // 错误!

    handle.join().unwrap();
}

// --- 线程间通信 - 消息传递 ---
fn message_passing() {
    // 创建通道
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("hello from thread");
        tx.send(msg).unwrap();
        // msg在这里已经move走了
    });

    // 接收消息
    let received = rx.recv().unwrap();
    println!("收到: {}", received);

    // 多个消息
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        for i in 0..5 {
            tx2.send(i).unwrap();
        }
        // 发送端关闭后，recv会返回None
    });

    for received in rx2 {
        println!("收到: {}", received);
    }
}

// --- 多个生产者 ---
fn multiple_producers() {
    let (tx, rx) = mpsc::channel();

    // spawn多个生产者
    let tx1 = tx.clone();
    thread::spawn(move || {
        tx1.send("from 1").unwrap();
    });

    let tx2 = tx.clone();
    thread::spawn(move || {
        tx2.send("from 2").unwrap();
    });

    // 主线程接收
    for _ in 0..2 {
        println!("{}", rx.recv().unwrap());
    }
}

// --- 共享状态 - Mutex ---
fn shared_state() {
    // Arc: 原子引用计数，可安全共享
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // lock()返回MutexGuard
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数: {}", *counter.lock().unwrap());
}

// --- 更多同步原语 ---
fn more_sync_primitives() {
    use std::sync::Barrier;
    use std::sync::RwLock;

    // RwLock - 读写锁
    let rw = RwLock::new(5);

    {
        let r = rw.read().unwrap();
        println!("读取: {}", *r);
    }

    {
        let mut w = rw.write().unwrap();
        *w = 10;
    }

    // Barrier - 等待所有线程到达某点
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];

    for i in 0..3 {
        let b = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("线程 {} 等待中...", i);
            b.wait();
            println!("线程 {} 继续", i);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}

// --- Send和Sync ---
// Send: 可以在线程间转移所有权
// Sync: 可以安全共享引用（&T是Send）
// 大多数类型自动实现这两个trait

// 手动实现（不常见）
// impl<T> Send for Container where T: Send {}

// --- RefCell和Rc（单线程）---
fn refcell_rc() {
    use std::cell::RefCell;
    use std::rc::Rc;

    // Rc: 引用计数
    // RefCell: 内部可变性
    let data = Rc::new(RefCell::new(5));

    let d1 = Rc::clone(&data);
    let d2 = Rc::clone(&data);

    *d1.borrow_mut() += 10;
    *d2.borrow_mut() += 10;

    println!("{:?}", data); // 25
}

// --- 练习：生产者-消费者 ---
fn producer_consumer() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    // 生产者1
    thread::spawn(move || {
        for i in 0..5 {
            tx.send(format!("A{}", i)).unwrap();
        }
    });

    // 生产者2
    thread::spawn(move || {
        for i in 0..5 {
            tx2.send(format!("B{}", i)).unwrap();
        }
    });

    // 消费者
    for _ in 0..10 {
        println!("消费: {}", rx.recv().unwrap());
    }
}

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
