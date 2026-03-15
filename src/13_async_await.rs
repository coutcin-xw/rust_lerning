// ============================================================
// 13_async_await.rs - 异步编程
// ============================================================
//
// 【核心概念】
// 异步编程 = 非阻塞 + 协作式多任务
// 用同步的代码风格实现异步操作
//
// 【对比其他语言】
// - JavaScript: async/await + Promise
// - Python: async/await + asyncio
// - C#: async/await + Task
// - Go: goroutine（协程，但语法不同）
// - Rust: async/await + Future + 运行时
//
// 【Rust异步编程的特点】
// 1. 零成本抽象 - 无运行时开销（需引入运行时库）
// 2. 编译期状态机 - async函数编译为状态机
// 3. 无内置运行时 - 需要选择tokio/async-std等
// 4. 所有权安全 - 异步代码也受所有权规则约束
//
// 【关键组件】
// - async fn: 定义异步函数
// - .await: 等待异步操作完成
// - Future trait: 异步计算的抽象
// - 运行时: 执行Future
// ============================================================

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

// --- 异步基础概念 ---
fn async_basics() {
    println!("=== 异步基础概念 ===");

    // 【同步 vs 异步】
    // 同步：操作阻塞当前线程
    // 异步：操作不阻塞，可以并发执行

    // 【async 关键字】
    // 定义异步函数
    async fn hello_async() {
        println!("Hello, async!");
    }

    // 【async 块】
    let future = async {
        println!("Hello from async block!");
        42
    };

    // 【Future trait】
    // async 函数返回 Future
    // Future 是惰性的，不会自动执行
    println!("Future 已创建，但未执行");

    // 【执行Future】
    // 需要 runtime 来执行
    // 这里用 block_on 模拟（实际项目用 tokio）
    println!("async fn 返回 Future，需要运行时执行");

    // 【.await 关键字】
    // 等待 Future 完成
    // 只能在 async 函数或块中使用

    println!("\n核心概念：async创建Future，await执行Future");
}

// --- 简单的Future实现 ---
// 手动实现 Future trait，理解其工作原理

struct SimpleFuture {
    done: bool,
}

impl Future for SimpleFuture {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 【poll 方法】
        // 运行时调用 poll 检查 Future 是否完成
        // Poll::Ready(result) - 完成
        // Poll::Pending - 未完成，稍后再问

        if self.done {
            Poll::Ready("Future 完成!")
        } else {
            self.done = true;
            println!("第一次poll，返回Pending");
            Poll::Pending
        }
    }
}

fn future_trait() {
    println!("=== Future trait ===");

    // 【Future 定义】
    // trait Future {
    //     type Output;
    //     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
    // }

    // 【状态机模型】
    // async 函数被编译为状态机
    // 每次 poll 根据当前状态执行相应代码

    // 【示例：模拟异步操作】
    struct Delay {
        duration: Duration,
        started: Option<std::time::Instant>,
    }

    impl Future for Delay {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if let Some(started) = self.started {
                if started.elapsed() >= self.duration {
                    Poll::Ready(())
                } else {
                    // 安排稍后再次 poll
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            } else {
                self.started = Some(std::time::Instant::now());
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    println!("Future 是惰性的，poll 方法驱动其执行");
}

// --- async/await 语法 ---
fn async_await_syntax() {
    println!("=== async/await 语法 ===");

    // 【async fn】
    // 定义异步函数
    async fn fetch_data(id: u32) -> String {
        // 模拟异步操作
        format!("数据 {}", id)
    }

    // 【async 块】
    let future = async {
        let data = fetch_data(1).await;  // await 等待
        println!("获取到: {}", data);
        data
    };

    // 【异步函数调用】
    // 调用 async fn 返回 Future，不立即执行
    let future1 = fetch_data(1);
    let future2 = fetch_data(2);
    println!("两个Future已创建");

    // 【并发执行】
    // 在实际项目中使用 tokio::join! 或 futures::join!
    println!("并发执行：使用 join! 宏");

    // 【顺序执行】
    async fn sequential() {
        let data1 = fetch_data(1).await;
        let data2 = fetch_data(2).await;
        println!("顺序结果: {}, {}", data1, data2);
    }

    println!("\nasync 定义异步，await 等待结果");
}

// --- 异步运行时 ---
fn runtime_intro() {
    println!("=== 异步运行时 ===");

    // 【为什么需要运行时？】
    // Rust 的 Future 是惰性的
    // 需要运行时来执行它们

    // 【主流运行时】
    // 1. tokio - 最流行，功能全面
    // 2. async-std - 标准库风格的API
    // 3. smol - 轻量级

    // 【Tokio 示例】
    // 需要在 Cargo.toml 添加:
    // [dependencies]
    // tokio = { version = "1", features = ["full"] }

    println!("主流运行时：tokio, async-std, smol");

    println!("\n运行时示例（伪代码）：");
    println!("#[tokio::main]");
    println!("async fn main() {{");
    println!("    // 异步代码");
    println!("}}");

    println!("\n或者手动创建运行时：");
    println!("let rt = tokio::runtime::Runtime::new().unwrap();");
    println!("rt.block_on(async {{");
    println!("    // 异步代码");
    println!("}});");
}

// --- 异步函数详解 ---
fn async_functions() {
    println!("=== 异步函数详解 ===");

    // 【async fn 的返回类型】
    // async fn foo() -> T
    // 实际返回: impl Future<Output = T>

    // 【异步方法】
    struct Service {
        name: String,
    }

    impl Service {
        async fn fetch(&self) -> String {
            format!("{} 的数据", self.name)
        }

        async fn process(&self, data: &str) -> String {
            format!("处理: {}", data)
        }
    }

    // 【异步闭包】
    println!("异步闭包示例：");
    println!("  let async_closure = |x: i32| async move {{ x * 2 }};");

    println!("async fn 编译为返回 impl Future 的函数");

    // 【异步函数的生命周期】
    // async fn borrow_data(&self, data: &str) -> String
    // 编译为复杂的 Future，包含引用的生命周期

    println!("\nasync fn 的 Output 就是返回类型");
}

// --- 并发模式 ---
fn concurrency_patterns() {
    println!("=== 异步并发模式 ===");

    // 【join! - 并发等待多个】
    // 所有 Future 都完成后继续
    println!("join! 宏：并发执行多个Future，等待全部完成");

    // 【select! - 竞争执行】
    // 任一 Future 完成后继续
    println!("select! 宏：竞争执行，任一完成即继续");

    // 【spawn - 创建任务】
    // 后台执行，不等待
    println!("spawn：创建后台任务");

    // 【FuturesUnordered - 动态并发】
    println!("FuturesUnordered：动态管理多个Future");

    println!("\n并发模式：join!, select!, spawn, FuturesUnordered");

    // 模拟异步操作
    fn async_operation(id: i32) -> impl std::future::Future<Output = i32> {
        async move { id * 2 }
    }

    let _ = async_operation(1);
}

// --- 异步错误处理 ---
fn async_error_handling() {
    println!("=== 异步错误处理 ===");

    // 【Result 与异步】
    async fn may_fail(id: u32) -> Result<String, &'static str> {
        if id == 0 {
            Err("ID不能为0")
        } else {
            Ok(format!("数据 {}", id))
        }
    }

    // 【使用 ? 操作符】
    async fn process() -> Result<String, &'static str> {
        let data = may_fail(1).await?;
        let data2 = may_fail(2).await?;
        Ok(format!("{} + {}", data, data2))
    }

    // 【处理异步错误】
    async fn handle_errors() {
        match may_fail(1).await {
            Ok(data) => println!("成功: {}", data),
            Err(e) => println!("错误: {}", e),
        }
    }

    // 【try_join! - 全部成功或第一个错误】
    println!("try_join! 宏：全部成功或返回第一个错误");

    println!("异步函数可以使用 Result 和 ? 操作符");

    let _ = may_fail(1);
    let _ = process();
    let _ = handle_errors();
}

// --- 异步流 ---
fn async_streams() {
    println!("=== 异步流 ===");

    // 【Stream trait】
    // 类似 Iterator，但是异步的
    // trait Stream {
    //     type Item;
    //     fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
    // }

    println!("Stream 是异步版本的 Iterator");

    // 【使用 Stream】
    println!("\nwhile let Some(item) = stream.next().await {{");
    println!("    println!(\"{{:?}}\", item);");
    println!("}}");

    // 【常见 Stream】
    // - TcpListener::incoming() - 接受连接
    // - Lines - 按行读取
    // - Interval - 定时器

    println!("\nStream 用 while let 循环消费");
}

// --- 异步网络编程 ---
fn async_networking() {
    println!("=== 异步网络编程 ===");

    // 【TCP 服务器示例】
    println!("TCP 服务器示例（伪代码）：");
    println!("#[tokio::main]");
    println!("async fn main() {{");
    println!("    let listener = TcpListener::bind(\"127.0.0.1:8080\").await.unwrap();");
    println!("    loop {{");
    println!("        let (socket, addr) = listener.accept().await.unwrap();");
    println!("        tokio::spawn(async move {{");
    println!("            // 处理连接");
    println!("        }});");
    println!("    }}");
    println!("}}");

    // 【HTTP 客户端示例】
    println!("\nHTTP 客户端示例（需要 reqwest 库）：");
    println!("let body = reqwest::get(\"https://example.com\")");
    println!("    .await?");
    println!("    .text()");
    println!("    .await?;");

    println!("\n异步 I/O 不阻塞线程，可以处理大量并发连接");
}

// --- 异步与同步的选择 ---
fn async_vs_sync() {
    println!("=== 异步 vs 同步 ===");

    println!("【使用异步的场景】");
    println!("  - 高并发网络服务（Web服务器、API）");
    println!("  - 大量 I/O 操作（数据库查询、文件读写）");
    println!("  - 需要同时处理多个任务");
    println!("  - 微服务架构");

    println!("\n【使用同步的场景】");
    println!("  - CPU 密集型计算");
    println!("  - 简单的命令行工具");
    println!("  - 不需要并发的场景");
    println!("  - 学习和原型开发");

    println!("\n【混合使用】");
    println!("  - 异步主循环 + 同步计算任务");
    println!("  - 使用 spawn_blocking 处理 CPU 密集任务");
    println!("  - tokio::task::spawn_blocking(|| {{ ... }})");
}

// --- 异步最佳实践 ---
fn async_best_practices() {
    println!("=== 异步最佳实践 ===");

    println!("【1. 避免阻塞异步运行时】");
    println!("  - 不要在 async 代码中调用阻塞函数");
    println!("  - 使用异步版本的库（tokio::fs 而不是 std::fs）");
    println!("  - CPU 密集任务用 spawn_blocking");

    println!("\n【2. 正确处理取消】");
    println!("  - select! 可以取消未完成的 Future");
    println!("  - 使用 CancellationToken 进行优雅关闭");

    println!("\n【3. 避免过度并发】");
    println!("  - 使用 Semaphore 限制并发数");
    println!("  - 使用 buffer_unordered 控制并发度");

    println!("\n【4. 错误处理】");
    println!("  - 使用 Result 而不是 panic");
    println!("  - 实现良好的错误类型");

    println!("\n【5. 资源管理】");
    println!("  - 注意异步代码中的资源泄漏");
    println!("  - 使用 RAII 模式管理资源");

    println!("\n【6. 性能优化】");
    println!("  - 避免不必要的 clone");
    println!("  - 使用 Arc 共享只读数据");
    println!("  - 合理设置任务数量");
}

// --- 模拟异步运行（演示用）---
// 实际项目中使用 tokio 或 async-std
mod mini_runtime {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    // 简单的 block_on 实现（仅用于演示）
    pub fn block_on<F: Future>(mut future: F) -> F::Output {
        // 创建一个简单的 waker
        fn dummy_raw_waker() -> RawWaker {
            fn no_op(_: *const ()) {}
            fn clone(_: *const ()) -> RawWaker {
                dummy_raw_waker()
            }

            static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
            RawWaker::new(std::ptr::null(), &VTABLE)
        }

        let waker = unsafe { Waker::from_raw(dummy_raw_waker()) };
        let mut cx = Context::from_waker(&waker);

        // 不断 poll 直到完成
        let mut future = unsafe { Pin::new_unchecked(&mut future) };
        loop {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(output) => return output,
                Poll::Pending => {
                    // 在真实运行时中，这里会 yield 让出线程
                    std::thread::yield_now();
                }
            }
        }
    }
}

// 演示用的异步函数
async fn demo_async_function() -> i32 {
    println!("异步函数执行中...");
    42
}

pub fn run() {
    println!("\n========== 13_async_await ==========");
    async_basics();
    future_trait();
    async_await_syntax();
    runtime_intro();
    async_functions();
    concurrency_patterns();
    async_error_handling();
    async_streams();
    async_networking();
    async_vs_sync();
    async_best_practices();

    // 演示运行
    println!("\n=== 演示：运行异步函数 ===");
    let result = mini_runtime::block_on(demo_async_function());
    println!("异步函数返回: {}", result);
}