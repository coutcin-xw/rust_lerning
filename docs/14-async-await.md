# 第 14 章：异步编程

## 学习目标

- 理解 async / await 的基本语法
- 理解 Future trait 和 poll 模型
- 使用 Tokio 作为异步运行时
- 掌握 `join!` / `select!` 并发模式
- 使用 **async fn in traits** 和 async closures

## 同步 vs 异步

```
同步：线程等待 I/O 完成，期间被阻塞
异步：线程在等待 I/O 时切换到其他任务，不浪费 CPU
```

```rust
// 同步：一步步等待
fn fetch_data() -> String {
    let resp = request("https://api.example.com");  // 阻塞线程
    resp
}

// 异步：标记可等待的操作
async fn fetch_data() -> String {
    let resp = request("https://api.example.com").await;  // 挂起，不阻塞
    resp
}
```

> 📘 *在 Rust 2024 Edition 中，`Future` 和 `IntoFuture` 已加入 prelude，无需手动 `use std::future::Future`。*

## async / await 语法

```rust
// async 函数
async fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// async 代码块
let future = async {
    let result = do_something().await;
    println!("{}", result);
};

// .await 等待 Future 完成
// 只能在 async 函数或 async 块内部使用
greet("Rust").await;
```

> 💡 `async fn` 本质上是语法糖。`async fn foo() -> T` 编译后等价于 `fn foo() -> impl Future<Output = T>`。

## Future Trait

```rust
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),    // 已完成
    Pending,     // 未完成，稍后再试
}
```

Future 是**惰性**的：创建时不执行，只有 `.await` 或被提交给运行时才开始运行。

## 异步运行时

Rust 标准库只提供 Future trait，不包含运行时。你需要选择第三方运行时：

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

### 使用 tokio

```rust
#[tokio::main]
async fn main() {
    // 串行执行
    let a = fetch_url("https://example.com/a").await;
    let b = fetch_url("https://example.com/b").await;

    // 并发执行
    let (a, b) = tokio::join!(
        fetch_url("https://example.com/a"),
        fetch_url("https://example.com/b"),
    );

    println!("{}, {}", a, b);
}
```

## 并发模式

### join! — 同时等待多个 Future

```rust
let (result1, result2, result3) = tokio::join!(
    task1(),
    task2(),
    task3(),
);
// 三个任务并发执行，全部完成后返回
```

### select! — 竞速等待

```rust
tokio::select! {
    result = task1() => println!("task1 先完成: {}", result),
    result = task2() => println!("task2 先完成: {}", result),
    _ = tokio::time::sleep(Duration::from_secs(5)) => println!("超时"),
}
```

### spawn — 创建异步任务

```rust
let handle = tokio::spawn(async {
    // 在独立的异步任务中运行
    heavy_computation().await
});

let result = handle.await.unwrap();  // 等待任务完成
```

## Async fn in traits

> 📘 *`async fn` in traits 自 Rust 1.75 起稳定（跨 Edition），在 Rust 2024 Edition 中 RPIT 行为与其对齐。*

```rust
trait Database {
    async fn query(&self, sql: &str) -> Result<Vec<Row>, DbError>;
}

struct PostgresDb;

impl Database for PostgresDb {
    async fn query(&self, sql: &str) -> Result<Vec<Row>, DbError> {
        // 异步数据库查询
        todo!()
    }
}
```

## 异步错误处理

与同步代码一样使用 `Result` + `?`：

```rust
async fn process() -> Result<String, Error> {
    let data = fetch_data().await?;      // ? 传播错误
    let result = transform(data).await?;
    Ok(result)
}
```

## 异步与同步的选择

| 适合异步 | 适合同步 |
|----------|---------|
| 网络服务（Web 服务器） | CPU 密集型计算 |
| 大量并发连接 | 简单的 CLI 工具 |
| 数据库查询 | 本地文件操作（小量） |
| 代理 / 网关 | 嵌入式 / 实时系统 |

## 跨语言对比

| 概念 | Rust | JS/TS | Python | Go |
|------|------|-------|--------|-----|
| 语法 | `async fn` / `.await` | `async` / `await` | `async def` / `await` | goroutine（无 await） |
| 运行时 | 第三方（tokio 等） | 内置 | 内置 | 内置 |
| Future 惰性 | ✅ | ❌（eager） | ❌（eager） | N/A |
| 零成本 | ✅ | ❌ | ❌ | ✅ |

## 练习

1. 用 tokio 写一个并发请求两个 URL 并汇总结果的程序
2. 用 `tokio::select!` 实现带超时的异步操作
3. 定义一个包含 `async fn` 的 trait，为其实现两个不同的类型

---

← [第 13 章：智能指针](./13-smart-pointers.md) | [返回目录](./README.md) | → [第 15 章：模块系统](./15-modules.md)
