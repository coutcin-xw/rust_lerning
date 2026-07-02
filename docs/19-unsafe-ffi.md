# 第 19 章：unsafe Rust 与 FFI

## 学习目标

- 理解 `unsafe` 的五个超级能力
- 安全地使用原始指针
- 调用 C 函数（FFI 基础）
- 了解 `unsafe` 的安全抽象包装原则
- 掌握 2024 Edition 的 unsafe 新规范

## 为什么需要 unsafe？

Rust 编译器不能保证所有操作都是安全的。`unsafe` 提供了一种"信任程序员"的机制，让你可以做编译器无法验证的事情：

> 💡 `unsafe` 不是绕过 Rust 安全规则的方式，而是**你自己承担编译器无法保证的那部分责任**。

## unsafe 的五个能力

```rust
unsafe {
    // 1. 解引用原始指针
    // 2. 调用 unsafe 函数或方法
    // 3. 访问或修改可变静态变量
    // 4. 实现 unsafe trait
    // 5. 访问 union 的字段
}
```

注意：`unsafe` 不关闭借用检查器，也不会禁用其他安全检查。它只是开启了这五个额外能力。

## 原始指针

```rust
let mut num = 5;

// 创建原始指针（safe 操作）
let r1: *const i32 = &num;      // 不可变原始指针
let r2: *mut i32 = &mut num;    // 可变原始指针

// 解引用原始指针（必须在 unsafe 块中）
unsafe {
    *r2 = 10;
    println!("r1: {}", *r1);  // 10
}
```

原始指针 vs 引用：
- 原始指针可以忽略借用规则（同时有可变和不可变指针）
- 原始指针不保证指向有效内存
- 原始指针可以为 null
- 原始指针无自动清理

## 调用 unsafe 函数

```rust
unsafe fn dangerous() {
    // 这个函数需要调用者来保证安全条件
}

// 调用 unsafe 函数必须在 unsafe 块中
unsafe {
    dangerous();
}
```

> 📘 *在 Rust 2024 Edition 中，`unsafe_op_in_unsafe_fn` 默认启用 warn。即使在 `unsafe fn` 内部，unsafe 操作也必须包裹在 `unsafe {}` 块中。这使得"安全"和"不安全"的边界更加清晰。*

```rust
// 2024 Edition 推荐写法
unsafe fn process(ptr: *const u8) {
    unsafe {                    // 显式标记 unsafe 操作
        println!("{}", *ptr);
    }
    // 非 unsafe 的操作不需要标记
    println!("done");
}
```

## FFI：调用 C 函数

> 📘 *在 Rust 2024 Edition 中，`extern` 块必须加 `unsafe` 关键字。`#[unsafe(no_mangle)]` 替代了旧的 `#[no_mangle]`。*

### 调用 C 标准库函数

```rust
// 2024 Edition 写法
unsafe extern "C" {
    fn abs(input: i32) -> i32;
    fn sqrt(x: f64) -> f64;
}

fn main() {
    unsafe {
        println!("abs(-3) = {}", abs(-3));
        println!("sqrt(2.0) = {}", sqrt(2.0));
    }
}
```

### 从 C 调用 Rust 函数

```rust
// 2024 Edition 写法
#[unsafe(no_mangle)]
pub unsafe extern "C" fn callback(x: i32) -> i32 {
    x * 2
}
```

> 💡 将 Rust 编译为 C 可链接的静态库/动态库：
> ```toml
> [lib]
> crate-type = ["cdylib", "staticlib"]
> ```

## unsafe trait

```rust
// 实现者必须保证安全性条件
unsafe trait TrustedLen { }

// 实现 unsafe trait 必须在 unsafe impl 中
unsafe impl TrustedLen for Vec<i32> { }
```

## 安全抽象包装原则

unsafe 代码的最佳实践：**把 unsafe 代码封装在安全的 API 后面**：

```rust
// ❌ 差：让调用者操心 unsafe
pub unsafe fn set_value(ptr: *mut i32, val: i32) {
    *ptr = val;
}

// ✅ 好：safe API 封装 unsafe 操作
pub fn set_first(list: &mut [i32], val: i32) {
    if list.is_empty() {
        return;
    }
    // 内部使用 unsafe，但 API 是安全的
    unsafe {
        *list.as_mut_ptr() = val;
    }
}
```

## 禁止 `static mut` 引用

> 📘 *Rust 2024 Edition 禁止直接引用 `static mut`——这是 deny-by-default 错误。改用 `&raw mut` 获取原始指针：*

```rust
// 2024 Edition
static mut COUNTER: u32 = 0;

fn increment() {
    unsafe {
        let ptr: *mut u32 = &raw mut COUNTER;  // ✅ 使用 &raw mut
        *ptr += 1;
    }
}
```

## 2024 Edition unsafe 变化速查

| 变化 | 2021 Edition | 2024 Edition |
|------|-------------|-------------|
| extern 块 | `extern "C" { }` | `unsafe extern "C" { }` |
| no_mangle | `#[no_mangle]` | `#[unsafe(no_mangle)]` |
| unsafe fn 内 unsafe 操作 | 隐式允许 | 需显式 `unsafe {}` 块 |
| static mut 引用 | warning | deny-by-default 错误 |

## 练习

1. 写一个函数，用 `unsafe` 交换两个 `*mut i32` 指向的值
2. 调用 C 标准库的 `malloc` 和 `free`，分配和释放内存
3. 写一个安全的 `SliceBuf` 封装，内部用 unsafe 操作原始指针，对外提供安全的读写 API

---

← [第 18 章：测试](./18-testing.md) | [返回目录](./README.md) | → [附录 A：工具链](./appendix-a-tools.md)
