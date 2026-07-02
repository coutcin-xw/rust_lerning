# 第 14 章：异步编程

## 学习目标

- 理解 async / await 语法和执行模型
- 理解 Future trait 的 poll 机制
- 使用 Tokio 作为异步运行时
- 掌握 `join!` / `select!` / `spawn` 并发模式
- 使用 **async fn in traits** 定义异步接口
- 了解何时用异步，何时用同步

## 为什么需要异步？

```rust
// 同步：线程被阻塞，等待网络响应
fn fetch_url(url: &str) -> String {
    let response = blocking_request(url);  // 线程卡在这，浪费 CPU
    response
}
// 1000 个请求 = 1000 个线程 × 每个线程的栈内存 = 大量开销

// 异步：线程不等待，切换到其他任务
async fn fetch_url(url: &str) -> String {
    let response = async_request(url).await;  // 挂起此任务，线程去干别的
    response
}
// 1000 个请求 = 几个线程，操作系统只调度活跃的任务
```

> 📘 *在 Rust 2024 Edition 中，`Future` 和 `IntoFuture` 已加入 prelude，无需 `use std::future::Future`。*

## async / await 语法

```rust
// async 函数：返回 impl Future<Output = T>
async fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// async 代码块
let future = async {
    let data = fetch_data().await;
    process(data)
};

// .await 等待 Future 完成
// 只能在 async fn 或 async 块中使用
let greeting = greet("Rust").await;
```

> 💡 `async fn` 是语法糖。`async fn foo() -> T` 编译为 `fn foo() -> impl Future<Output = T>`。Future 是**惰性**的——创建时不执行，只有 `.await` 时才开始运行。

## Future Trait 内部原理

```rust
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),    // 完成了，包含结果
    Pending,     // 没完成，运行时下次再 poll
}
```

**理解 poll 模型：**
1. 运行时调用 `poll()` 问 "你完成了吗？"
2. 如果完成 → 返回 `Ready(result)`
3. 如果没完成 → 返回 `Pending`，并告诉运行时 "数据到了叫我"
4. 数据到达时运行时再次调用 `poll()`，直到 `Ready`

这就是"协作式多任务"——每个 Future 主动告诉调度器自己是否需要等待。

## 异步运行时

Rust 标准库只定义了 `Future` trait，不包含运行时。你需要选择一个：

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

> `tokio` 是最流行的异步运行时。其他选择：`async-std`（API 贴近标准库）、`smol`（轻量级）。

### 使用 tokio

```rust
#[tokio::main]
async fn main() {
    // 你的异步代码
}
```

`#[tokio::main]` 展开后的等价写法：

```rust
fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            // 你的异步代码
        });
}
```

### 串行 vs 并发

```rust
// 串行：a 完成后再开始 b（总时间 = a + b）
let a = fetch_url(url_a).await;
let b = fetch_url(url_b).await;

// 并发：a 和 b 同时进行（总时间 = max(a, b)）
let (a, b) = tokio::join!(
    fetch_url(url_a),
    fetch_url(url_b),
);
```

## 并发模式

### join! — 等待所有完成

```rust
let (user, posts, settings) = tokio::join!(
    fetch_user(user_id),
    fetch_posts(user_id),
    fetch_settings(user_id),
);
// 三个请求并发执行，全部完成后返回
```

### select! — 竞速，只取最先完成的

```rust
tokio::select! {
    result = task1() => println!("task1 先完成: {}", result),
    result = task2() => println!("task2 先完成: {}", result),
    _ = tokio::time::sleep(Duration::from_secs(5)) => {
        println!("超时！两个任务都没在 5 秒内完成");
    }
}
```

### spawn — 创建独立异步任务

```rust
let handle = tokio::spawn(async {
    // 这个任务在后台运行
    heavy_computation().await
});

// 主任务继续做别的事...
do_something_else().await;

// 等待后台任务完成
let result = handle.await.unwrap();
```

## Async fn in traits

> 📘 *`async fn` in traits 自 Rust 1.75 起稳定（跨 Edition）。在 Rust 2024 Edition 中，其 RPIT 行为与普通 `async fn` 对齐。*

```rust
trait Database {
    async fn query(&self, sql: &str) -> Result<Vec<Row>, DbError>;
    async fn execute(&self, sql: &str) -> Result<u64, DbError>;
}

struct PostgresDb { /* ... */ }

impl Database for PostgresDb {
    async fn query(&self, sql: &str) -> Result<Vec<Row>, DbError> {
        // 异步数据库查询
        todo!()
    }

    async fn execute(&self, sql: &str) -> Result<u64, DbError> {
        todo!()
    }
}
```

## 异步错误处理

和同步代码一样——`Result` + `?`：

```rust
async fn fetch_and_process(url: &str) -> Result<ProcessedData, Error> {
    let response = reqwest::get(url).await?;        // ? 传播网络错误
    let text = response.text().await?;              // ? 传播读取错误
    let data: Data = serde_json::from_str(&text)?;  // ? 传播解析错误
    Ok(process(data))
}
```

## 异步 vs 同步：选择指南

| 场景 | 用异步 | 用同步 |
|------|--------|--------|
| Web 服务器处理数千并发连接 | ✅ | ❌ 线程开销太大 |
| CPU 密集型计算 | ❌（反而增加开销） | ✅ |
| 简单的 CLI 工具 | ❌（不需要） | ✅ |
| 数据库查询密集型 | ✅ | ❌ 等待阻塞线程 |
| 本地文件操作（少量） | ❌（OS 不一定支持真异步 IO） | ✅ |
| 网络代理/网关 | ✅ | ❌ 必须异步 |
| 嵌入式实时系统 | ❌ | ✅ |

> 💡 异步不是"更快"，而是能**更高效地处理大量并发 I/O**。单个异步任务并不比同步快——优势在于任务间切换无需线程上下文切换。

## Pin / Unpin — 自引用类型的内存保证

当你深入使用 async 时，会遇到 `Pin`。它的核心问题：**Future 可能在内存中被移动，但 Future 内部的自引用指针不能跟着更新。**

### 问题场景

```rust
async fn demo() {
    let s = String::from("hello");
    let r = &s;  // 自引用：r 指向同一个 Future 中的 s
    do_something(r).await;
    // 如果 Future 在 .await 之间被移动了内存位置
    // r 就变成了悬垂指针！
}
```

### Pin 做了什么

`Pin<Box<T>>` 或 `Pin<&mut T>` 保证被包装的值**不再被移动**：

```rust
use std::pin::Pin;

// Box::pin 创建固定在堆上的 Future
let future = Box::pin(async {
    // 这个 Future 内部可以有自引用，因为它在堆上的位置固定了
    let s = String::from("hello");
    let r = &s;
    do_something(r).await;
});
```

### Unpin — "可以安全移动" 的标记

大多数类型是 `Unpin`（自动实现），可以被安全移动。自引用类型不是 `Unpin`——这就是 `Pin` 发挥作用的地方。

> 💡 **你不需要手动处理 `Pin` 来使用 async。** `tokio::spawn`、`Box::pin`、`#[tokio::main]` 已经帮你处理好了。这里介绍它是为了帮你理解——为什么有些 async 相关 API 签名中会看到 `Pin<Box<dyn Future>>`。

```rust
// tokio::spawn 内部帮你 Pin 了
tokio::spawn(async { /* ... */ });  // ✅ 不用手动 Pin

// 手动创建固定的 Future（罕见场景）
let pinned_future = Box::pin(my_async_fn());

// 用 poll 手动推进（只在实现自己的运行时时需要）
use std::future::Future;
use std::task::{Context, Poll};
// pinned_future.as_mut().poll(cx);
```

理解即可：`Pin` 是 Rust 解决"自引用结构被移动"这一底层问题的方式，最终用户级别很少直接接触。

## 练习

1. 用 tokio 写一个程序，并发请求两个 URL，汇总响应内容长度
2. 用 `tokio::select!` 实现带超时的 HTTP 请求（如 3 秒未响应则放弃）
3. 定义一个 `async fn` 的 trait（如 `Cache` 有 `async fn get(&self, key: &str) -> Option<String>`），为两个类型实现
4. 理解 Future 的惰性：写一个 async 函数，创建 Future 但不 await，观察它是否执行
5. 用 `Box::pin` 手动创建一个 Pinned Future，理解为什么 Pin 对自引用结构是必要的

---

← [第 13 章：智能指针](./13-smart-pointers.md) | [返回目录](./README.md) | → [第 15 章：模块系统](./15-modules.md)
