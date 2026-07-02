# 附录 C：速查表

## 所有权与借用

| 规则 | 说明 |
|------|------|
| 每个值有唯一所有者 | `let s = String::from("hi")` — `s` 是所有者 |
| 离开作用域自动释放 | `}` — 自动调用 `drop()` |
| Move 语义 | `let s2 = s1` — `s1` 失效（除 `Copy` 类型） |
| 不可变引用 `&T` | 可以有多个 |
| 可变引用 `&mut T` | 同时只能有一个 |
| 同作用域内不可混用 | 不可变引用和可变引用不能同时活跃 |

## 常用 Trait

| Trait | 作用 | 自动实现？ |
|-------|------|----------|
| `Copy` | 赋值时复制而不断交 | 简单栈类型 |
| `Clone` | 显式深拷贝 | 需手动实现 |
| `Drop` | 离开作用域时自动清理 | 需手动实现 |
| `Deref` | 解引用 `*` 操作 | 需手动实现 |
| `Debug` | `{:?}` 调试格式化 | 可用 `#[derive(Debug)]` |
| `Display` | `{}` 用户友好格式化 | 需手动实现 |
| `PartialEq` | `==` 和 `!=` | 可用 `#[derive(PartialEq)]` |
| `PartialOrd` | `<` `>` 等比较 | 可用 `#[derive(PartialOrd)]` |
| `Send` | 可在线程间转移 | 大多数类型自动 |
| `Sync` | 可在多线程间共享引用 | 大多数类型自动 |
| `Default` | 默认值 | 可用 `#[derive(Default)]` |
| `From` / `Into` | 类型转换 | 需手动实现 |
| `Iterator` | 迭代器 | 需手动实现 `next()` |

## 智能指针选择

```
唯一所有权 + 堆分配         → Box<T>
共享所有权（单线程）        → Rc<T>
共享所有权（多线程）        → Arc<T>
内部可变（Copy 类型）      → Cell<T>
内部可变（任意类型、单线程）→ RefCell<T>
共享 + 内部可变（单线程）   → Rc<RefCell<T>>
共享 + 内部可变（多线程）   → Arc<Mutex<T>>
读写锁                     → Arc<RwLock<T>>
惰性/复用（借 or 拥有）     → Cow<T>
```

## ? 操作符链

```
fn read_config() -> Result<Config, Error> {
    let content = std::fs::read_to_string("config.toml")?;   // io::Error → Error
    let config: Config = toml::from_str(&content)?;           // toml::Error → Error
    Ok(config)
}
```

## 常见派生宏

```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct MyType { /* ... */ }
```

## 生命周期省略规则

| 规则 | 说明 |
|------|------|
| 规则 1 | 每个引用参数都有自己的生命周期参数 |
| 规则 2 | 只有 1 个输入生命周期 → 赋给所有输出 |
| 规则 3 | 有 `&self` → `self` 的生命周期赋给所有输出 |

## 闭包 Trait 层次

```
FnOnce ← FnMut ← Fn
 (一次)  (可改)  (只读)
```

## 常用迭代器模式

```rust
// 转换
iter.map(|x| x * 2)                 // 映射
iter.filter(|x| x > 0)              // 过滤
iter.filter_map(|x| x.parse().ok()) // 过滤+转换
iter.take(5)                        // 取前 5 个
iter.skip(3)                        // 跳过前 3 个
iter.flatten()                      // 展平

// 收集
iter.collect::<Vec<_>>()            // 收集到 Vec
iter.collect::<HashMap<_, _>>()     // 收集到 HashMap
iter.fold(0, |acc, x| acc + x)     // 折叠
iter.sum::<i32>()                   // 求和
iter.any(|x| x > 0)                // 是否存在
iter.all(|x| x > 0)                // 全部满足
iter.find(|x| x > 0)               // 查找
iter.count()                        // 计数
```

## 错误处理模式

```rust
// 传播错误（推荐）
fn f() -> Result<T, E> { let x = g()?; Ok(x) }

// 直接获取（原型/测试）
let x = g().unwrap();

// 带消息的直接获取
let x = g().expect("g 不应该失败");

// 提供默认值
let x = g().unwrap_or(default);

// match 处理
match g() {
    Ok(x) => use(x),
    Err(e) => handle(e),
}
```

## 2024 Edition 关键语法

```rust
// let chains（2024 Edition 独占，1.88.0 稳定）
if let Some(x) = opt1 && let Some(y) = opt2 { }

// unsafe extern（2024 强制）
unsafe extern "C" { fn sqrt(x: f64) -> f64; }

// unsafe 属性（2024 强制）
#[unsafe(no_mangle)]
pub extern "C" fn callback() { }

// Future/IntoFuture 已入 prelude（无需 import）
async fn foo() -> impl Future<Output = i32> { 42 }

// RPIT 自动捕获所有生命周期（2024 默认行为）
fn bar<'a>(x: &'a str) -> impl Display + use<'a> { x }
```

## Cargo 常用命令

```bash
cargo new <name>         # 创建新项目
cargo build              # 编译
cargo run                # 编译+运行
cargo check              # 快速检查（推荐开发时用）
cargo test               # 运行测试
cargo fmt                # 格式化代码
cargo clippy             # 运行 linter
cargo doc --open         # 生成并打开文档
cargo add <crate>        # 添加依赖（需 cargo-edit）
cargo update             # 更新依赖
cargo clean              # 清除编译缓存
cargo build --release    # 发布构建
```

---

← [附录 B：学习资源](./appendix-b-resources.md) | [返回目录](./README.md)
