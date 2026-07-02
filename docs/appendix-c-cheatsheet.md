# 附录 C：速查表

## 所有权规则

| 规则 | 代码示例 |
|------|---------|
| 值有唯一所有者 | `let s = String::from("hi");` — s 是所有者 |
| Move 后原变量失效 | `let s2 = s1;` — s1 不可再用 |
| Copy 类型复制 | `let y = x;` — x, y 都可用（`i32`, `f64`, `bool`, `char` 等） |
| 离开作用域自动 drop | `}` — 自动调用 `drop()` |
| Clone 显式深拷贝 | `let s2 = s1.clone();` |

## 借用规则

| 规则 | 代码 |
|------|------|
| `&T` 多个共存 | `let r1 = &s; let r2 = &s;` ✅ |
| `&mut T` 独占 | `let r = &mut s;` 同时只能有一个 |
| 不能同时有 `&T` 和 `&mut T` | `let r1 = &s; let r2 = &mut s;` ❌ |
| NLL：引用用到最后一行为止 | 之后可以创建新引用 |
| 切片 `&[T]` `&str` | `&s[..5]`, `&arr[1..3]` |

## 常用 Trait

| Trait | 作用 | derive? |
|-------|------|---------|
| `Debug` | `{:?}` 调试输出 | ✅ `#[derive(Debug)]` |
| `Display` | `{}` 用户输出 | ❌ 手动 |
| `Clone` | `.clone()` 深拷贝 | ✅ |
| `Copy` | 赋值时复制 | ✅ (仅栈类型) |
| `PartialEq` | `==` `!=` | ✅ |
| `Eq` | 等价关系 | ✅ |
| `PartialOrd` | `<` `>` `<=` | ✅ |
| `Ord` | 全序 | ✅ |
| `Default` | `Default::default()` | ✅ |
| `Hash` | 可哈希 | ✅ |
| `Drop` | 离开作用域清理 | ❌ 手动 |
| `Deref` | `*` 解引用 | ❌ 手动 |
| `From` / `Into` | 类型转换 | ❌ 手动 |
| `Iterator` | `.next()` → 获得所有方法 | ❌ 手动 |

## 智能指针选择

```
需要堆分配                              → Box<T>
共享所有权（单线程）                     → Rc<T>
共享所有权（多线程）                     → Arc<T>
内部可变（Copy 类型）                    → Cell<T>
内部可变（任意类型，单线程）             → RefCell<T>
共享 + 修改（单线程）                    → Rc<RefCell<T>>
共享 + 修改（多线程）                    → Arc<Mutex<T>>
读多写少                                → Arc<RwLock<T>>
惰性分配 / 借用或拥有                    → Cow<T>
```

## 迭代器速查

```rust
// 适配器（惰性）
iter.map(|x| ...)        // 映射         iter.take(n)      // 前 n 个
iter.filter(|x| ...)     // 过滤         iter.skip(n)      // 跳过 n 个
iter.filter_map(|x| ...) // 滤+转        iter.enumerate()  // 带索引
iter.flatten()           // 展平         iter.chain(b)     // 连接
iter.flat_map(|x| ...)   // 映射+展平    iter.zip(b)       // 配对

// 消费者（触发计算）
iter.collect::<Vec<_>>() // 收集到 Vec   iter.sum::<i32>() // 求和
iter.fold(init, |a,x|)   // 折叠         iter.any(|x| ...) // 存在?
iter.reduce(|a,x|)       // 无初值折叠   iter.all(|x| ...) // 全满足?
iter.count()             // 计数         iter.find(|x| ...) // 查找
iter.for_each(|x| ...)   // 每个执行     iter.max()        // 最大
```

## 错误处理速查

```rust
// 传播（推荐）                → let x = fallible()?;
// 快速获取（原型）             → let x = fallible().unwrap();
// 带消息的快速获取             → let x = fallible().expect("原因");
// 默认值                      → let x = fallible().unwrap_or(default);
// 惰性默认值                  → let x = fallible().unwrap_or_else(|| compute());
// 匹配处理                    → match fallible() { Ok(x) => ..., Err(e) => ... }
// panic                      → panic!("消息");
// debug 断言                  → debug_assert!(condition);
```

## 生命周期省略规则

```
1. 每个引用参数各有自己的生命周期
2. 只有 1 个输入 → 赋给所有输出
3. 有 &self → self 的生命周期赋给所有输出
```

## 闭包 Trait 层次

```
        FnOnce (调用 ≥ 1 次，可能消耗捕获值)
          ↑
        FnMut (可调用多次，可变借用)
          ↑
        Fn    (可调用多次，不可变借用)
```

## Cargo 常用命令

```bash
cargo new <name>           # 创建项目
cargo build                # 编译 (debug)
cargo build --release      # 编译 (release)
cargo run                  # 编译 + 运行
cargo check                # 快速检查（推荐开发时）
cargo test                 # 运行测试
cargo test <name>          # 运行匹配的测试
cargo fmt                  # 格式化
cargo clippy               # lint
cargo doc --open           # 生成 + 打开文档
cargo add <crate>          # 添加依赖
cargo update               # 更新依赖
cargo clean                # 清理 target/
```

## Rust 2024 Edition 新语法速查

```rust
// let chains (2024 Edition 独占, Rust ≥ 1.88.0)
if let Some(a) = opt1 && let Some(b) = opt2 && condition {
    // 使用 a, b
}

// unsafe extern (2024 Edition 必写)
unsafe extern "C" {
    fn sqrt(x: f64) -> f64;
}

// unsafe 属性 (2024 Edition 必写)
#[unsafe(no_mangle)]
pub extern "C" fn my_func() { }

// unsafe fn 内部显式 unsafe 块 (2024 默认 warn)
unsafe fn process(ptr: *const u8) {
    unsafe { println!("{}", *ptr); }
}

// RPIT 自动捕获生命周期 (2024 默认行为)
fn foo(x: &str) -> impl Display { x }

// 精确控制: use<..>
fn bar<T>(x: &T) -> impl Display + use<T> { ... }

// Future/IntoFuture 入 prelude (无需 import)
async fn demo() { }

// async closures
let f = async |x: i32| -> i32 { x * 2 };

// 禁止 static mut 引用 (改用 &raw mut)
static mut COUNTER: u32 = 0;
let ptr = &raw mut COUNTER;  // 不是 &COUNTER
```

---

← [附录 B：学习资源](./appendix-b-resources.md) | [返回目录](./README.md)
