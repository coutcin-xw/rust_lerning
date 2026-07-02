# 第 19 章：unsafe Rust 与 FFI

## 学习目标

- 理解 unsafe 的五个能力及其使用场景
- 安全地使用原始指针
- 使用 FFI 调用 C 函数
- 掌握"用安全 API 包装 unsafe 代码"的设计模式
- 适应 2024 Edition 的 unsafe 新规范

## 为什么需要 unsafe？

Rust 编译器不能验证所有操作的合法性。`unsafe` 是一种"信任程序员"的机制：

> 💡 `unsafe` **不关闭借用检查器**，不削弱其他安全保证。它只是让你能做五件编译器无法验证的事。

## unsafe 的五个超级能力

```rust
unsafe {
    // 1. 解引用原始指针（*const T, *mut T）
    // 2. 调用 unsafe 函数或方法
    // 3. 访问或修改可变静态变量（static mut）
    // 4. 实现 unsafe trait
    // 5. 访问 union 的字段
}
```

## 原始指针

```rust
let mut num = 5;

// 创建原始指针（safe 操作）
let r1: *const i32 = &num;      // *const T
let r2: *mut i32 = &mut num;    // *mut T

// 解引用原始指针（必须在 unsafe 块中）
unsafe {
    println!("r1: {}", *r1);  // 5
    *r2 += 1;
    println!("r2: {}", *r2);  // 6
}
```

原始指针 vs 引用（`&T`）：

| | `&T` / `&mut T` | `*const T` / `*mut T` |
|---|---|---|
| 借用规则 | 编译期强制 | 无 |
| null | 不允许 | 可以为 null |
| 自动释放 | 不适用（借用） | 无 |
| 创建 | `&x` | `&raw const x`, `std::ptr::addr_of!` |

## 调用 unsafe 函数

```rust
unsafe fn dangerous() { /* ... */ }

// 必须在 unsafe 块中调用
unsafe {
    dangerous();
}
```

> 📘 *在 Rust 2024 Edition 中，`unsafe_op_in_unsafe_fn` 默认 warn。即使在 `unsafe fn` 内部，unsafe 操作也必须明确包在 `unsafe {}` 块中：*

```rust
// 2024 Edition 推荐写法
unsafe fn process(ptr: *const u8) {
    // safe 操作可以直接写
    println!("开始处理...");

    // unsafe 操作需要显式 unsafe 块
    unsafe {
        println!("值: {}", *ptr);
    }
}
```

## FFI — 调用 C 函数

> 📘 *在 Rust 2024 Edition 中，`extern` 块必须加 `unsafe` 关键字。`#[unsafe(no_mangle)]` 替代旧 `#[no_mangle]`。*

```rust
// 声明外部 C 函数
unsafe extern "C" {
    fn abs(input: i32) -> i32;
    fn sqrt(x: f64) -> f64;
}

fn main() {
    let x = -42;
    unsafe {
        println!("abs({}) = {}", x, abs(x));
        println!("sqrt(2.0) = {:.6}", sqrt(2.0));
    }
}
```

### 让 C 调用 Rust 函数

```rust
// 2024 Edition
#[unsafe(no_mangle)]  // 禁止名称修饰（mangling）
pub unsafe extern "C" fn my_callback(x: i32) -> i32 {
    x * 2
}
```

### 链接 C 库

```rust
// 链接系统库
#[link(name = "m")]     // libm (math library)
unsafe extern "C" {
    fn cos(x: f64) -> f64;
}

// 链接自定义 C 代码
#[link(name = "myclib")]
unsafe extern "C" {
    fn my_c_function();
}
```

### 编译为 C 可用的库

```toml
# Cargo.toml
[lib]
crate-type = ["cdylib", "staticlib"]
```

- `cdylib` — 动态库（`.so` / `.dylib` / `.dll`）
- `staticlib` — 静态库（`.a` / `.lib`）

## 安全抽象——最重要的模式

**把 unsafe 代码封装在安全的 API 后面：**

```rust
// 暴露安全的 API
pub fn set_element(slice: &mut [i32], index: usize, value: i32) -> bool {
    if index >= slice.len() {
        return false;  // 安全的边界检查
    }
    unsafe {
        // 内部用 unsafe 操作，但调用者获得的是安全的 API
        *slice.as_mut_ptr().add(index) = value;
    }
    true
}
```

这个原则是 Rust 标准库的基础——标准库内部使用了大量 unsafe 代码，但向用户提供的 API 都是安全的。

## 禁止 `static mut` 引用

> 📘 *Rust 2024 Edition 禁止直接引用 `static mut`，必须通过原始指针访问：*

```rust
// 2024 Edition
static mut COUNTER: u32 = 0;

fn increment() {
    unsafe {
        // ✅ 使用 &raw mut 获取原始指针
        let ptr = &raw mut COUNTER;
        *ptr += 1;
        // ❌ 不能写 let r = &COUNTER;
    }
}
```

## 2024 Edition unsafe 变化速查

| 项目 | 旧写法 | 2024 Edition 写法 |
|------|--------|------------------|
| extern 块 | `extern "C" { }` | `unsafe extern "C" { }` |
| no_mangle | `#[no_mangle]` | `#[unsafe(no_mangle)]` |
| export_name | `#[export_name = "x"]` | `#[unsafe(export_name = "x")]` |
| unsafe fn 内操作 | 隐式 unsafe | 推荐显式 `unsafe {}` |
| static mut 引用 | `&COUNTER` (warning) | 必须用 `&raw mut COUNTER` |

## 何时使用 unsafe

| 场景 | 是否用 unsafe |
|------|-------------|
| 调用 C 函数（FFI） | ✅ 需要 |
| 直接操作硬件/内存映射 I/O | ✅ 需要 |
| 实现底层数据结构（如 `Vec` 内部） | ✅ 需要 |
| 性能优化（绕过边界检查） | ⚠️ 衡量后再用 |
| 规避借用检查器 | ❌ 几乎总是坏主意 |
| "求编译器别报错了" | ❌ 重新设计你的代码 |

## 练习

1. 写一个函数，用原始指针交换两个 `i32` 的值
2. 调用 C 标准库的 `malloc` 和 `free`，手动管理一块内存
3. 实现一个安全的 `SliceBuf` 封装：内部用 unsafe 的原始指针操作，对外提供安全的 `read(&self, index) -> Option<T>` 和 `write(&mut self, index, value)` 方法

---

← [第 18 章：测试](./18-testing.md) | [返回目录](./README.md) | → [附录 A：工具链](./appendix-a-tools.md)
