# 第 22 章：unsafe Rust 与 FFI

## 学习目标

- 理解 unsafe Rust 的设计哲学和五个能力
- 掌握原始指针的创建、运算、读写语义
- 用 `MaybeUninit`、`NonNull`、`UnsafeCell` 处理底层内存
- 通过 FFI 双向调用 Rust 和 C
- 掌握"安全抽象"模式——用 safe API 封装 unsafe 实现
- 用 miri 检测未定义行为

---

## 一、概念层：什么是 unsafe Rust

### 为什么需要 unsafe？

Rust 的安全保证基于一套严格的编译期规则——所有权、借用、生命周期。但在以下场景中，这些规则**本就不适用或无法验证**：

| 场景 | 为什么编译器帮不了你 |
|------|---------------------|
| 调用操作系统 / C 库 | 编译器看不到 C 代码的实现，无法验证安全性 |
| 直接操作硬件（内存映射 I/O） | 硬件地址不在 Rust 的内存模型中 |
| 实现 `Vec` / `Arc` 等底层容器 | 需要手动管理内存分配/释放，超出了借用检查的范围 |
| 性能关键路径中跳过边界检查 | 你已经用其他方式保证了安全性，但编译器不知道 |

`unsafe` 关键字就是为这些场景设计的——**它不削弱任何现有的安全检查，只是额外打开五扇"编译器无法验证，你说了算"的窗。**

> 💡 `unsafe` **不关闭借用检查器**。在 `unsafe` 块内，引用仍然有生命周期约束，`&mut` 仍然独占。你只是获得了五件安全 Rust 不让你做的事。

### 核心哲学：安全抽象

**整个 unsafe Rust 的设计围绕一条原则：**

```
┌──────────────────────────────────────┐
│  用 safe API 封装 unsafe 实现        │
│                                      │
│  调用者（safe 代码）                  │
│  └── 调用安全 API ←── 编译器保证安全  │
│       └── 内部用 unsafe ←── 你保证安全 │
└──────────────────────────────────────┘
```

标准库的 `Vec<T>`、`Arc<T>`、`Mutex<T>` 全部遵循这个模式——内部大量 unsafe，但暴露的 API 100% 安全。**你也应该这样写。**

### unsafe 的五个超级能力

```rust
unsafe {
    // 1. 解引用原始指针（*const T, *mut T）
    //    与 &T 不同：无生命周期、可为 null、可指向未初始化内存
    //
    // 2. 调用 unsafe 函数
    //    FFI 调用 C 函数、操作系统 API
    //
    // 3. 访问或修改可变静态变量（static mut）
    //    全局可变状态，编译器无法追踪多线程访问
    //
    // 4. 实现 unsafe trait
    //    如 Send、Sync——你承诺你的类型满足线程安全条件
    //
    // 5. 访问 union 的字段
    //    读取非活跃字段是 UB，由你保证正确
}
```

---

## 二、机制层：每个 unsafe 能力深入

### 原始指针 vs 引用 — 为什么需要原始指针？

安全 Rust 的 `&T` / `&mut T` 有三条铁律：
- 不能为空
- 必须始终指向有效数据（有生命周期约束）
- `&mut T` 在作用域内独占

但有些场景你**必须**突破这些约束：

| 你需要… | `&T` / `&mut T` | `*const T` / `*mut T` |
|---------|:--:|:--:|
| 指向未初始化的内存块 | ❌ | ✅ |
| 空指针（表示"不存在"） | ❌ | ✅ |
| 比数据本身活得更久的指针 | ❌（生命周期约束） | ✅（无生命周期） |
| 多个可变指针同时指向同一块内存 | ❌ | ✅ |
| 指向内存映射的硬件地址 | ❌ | ✅ |
| 编译器帮忙检查 | ✅ | ❌（全靠你） |

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

### `&raw` 操作符 — 安全地创建原始指针

Rust 2024 Edition 引入了 `&raw const` 和 `&raw mut`，替代旧的 `addr_of!`/`addr_of_mut!` 宏：

```rust
let mut x = 42;
let raw: *const i32 = &raw const x;   // 创建 *const
let raw_mut: *mut i32 = &raw mut x;   // 创建 *mut
```

`&raw` 不会触发借用——即使变量有未对齐、悬垂等情况也能安全使用。旧版 `&x as *const T` 在对齐错误时是 UB。

### NonNull\<T\> — 非空指针抽象

`NonNull<T>` 是一个保证非 null 的 `*mut T` 包装，编译器可以据此优化（如 `Option<NonNull<T>>` 与 `*mut T` 同大小）：

```rust
use std::ptr::NonNull;

let mut value = 42;
let ptr = NonNull::new(&mut value as *mut i32).unwrap();

unsafe {
    println!("值: {}", ptr.as_ref());  // 安全的引用，因为非 null + 对齐正确
    *ptr.as_ptr() = 100;               // 通过原始指针写
}
// ptr.as_ref() 返回 &T（安全），因为 NonNull 保证非空
// 但调用者仍需保证值的生命周期覆盖 ptr 的使用
```

### 指针运算 — 偏移、读写与语义

原始指针提供了一套精细的 unsafe 操作 API。**它们的核心差异在于是否要求目标内存已初始化、如何处理所有权。**

#### 偏移

```rust
use std::ptr;

let mut data = [10i32, 20, 30, 40, 50];
let base: *mut i32 = data.as_mut_ptr();

unsafe {
    let third = base.add(2);           // 前进 2 个元素，指向 data[2]
    let prev = base.add(4).sub(3);     // 后退，指向 data[1]
    let last = base.offset(4);         // 同 add，但 n 是 isize（可为负）
    
    // 按字节偏移
    let bytes: *const u8 = base as *const u8;
    let _ = bytes.byte_add(4);         // i32 占 4 字节，前进一个元素
}
```

#### 写：`write()` vs `*p = val`

对于 `i32` 这类 Copy 类型，两者没有区别。但对于非 Copy 类型和未初始化内存，区别极大：

```rust
use std::alloc::{alloc, Layout};

unsafe {
    let ptr = alloc(Layout::new::<String>()) as *mut String;

    // ❌ 错误！*ptr = 要求 *ptr 是"合法位置（place）"，合法位置要求内存已初始化
    // *ptr = String::from("hello");  // 未定义行为！

    // ✅ 正确：write 不要求目标已初始化，直接覆盖字节
    ptr.write(String::from("hello"));
}
```

| | `p.write(val)` | `*p = val` |
|---|---|---|
| 语义 | 位级覆盖，不关心原来有什么 | 赋值到 `*p` 这个 place |
| 要求目标已初始化？ | ❌ 不要求 | ⚠️ 要求（否则是 UB） |
| 会 drop 旧值吗？ | ❌ 不会 | ❌ 不会 |

**规则：初始化未初始化内存时，必须用 `write`。**

> ⚠️ `write` 也不自动 drop 旧值。覆盖一个已有的 `String` 会造成旧值泄漏——此时应先 `ptr::drop_in_place(p)` 手动析构。

#### 读：`read()` vs `*p`

```rust
// Copy 类型：read 和 *p 效果相同
let mut x = 42i32;
let p: *mut i32 = &mut x;
unsafe { let a = p.read(); let b = *p; }  // 都是位级拷贝

// 非 Copy 类型：*p 根本不能用！
let mut s = String::from("hello");
let p: *mut String = &mut s;
unsafe {
    // ❌ 编译错误：不能通过裸指针移动所有权
    // let moved = *p;

    // ✅ read 可以"移动"出来（你获得所有权）
    let moved = p.read();
    // 原位置逻辑上已"空"——不能再被 drop，s 也不能再使用（否则 double-free）
}
```

#### `as_ref()` / `as_mut()` — 裸指针到安全引用的桥梁

```rust
let p: *mut i32 = std::ptr::null_mut();
assert!(unsafe { p.as_ref() }.is_none());  // ✅ 空指针 → None，不崩

let value = 42;
let p: *const i32 = &value;
if let Some(r) = unsafe { p.as_ref() } {
    // r 是安全的 &T，可离开 unsafe 块正常使用
}
```

| | `*p` / `read()` / `write()` | `as_ref()` / `as_mut()` |
|---|---|---|
| 安全层级 | 始终在 `unsafe` 块内 | 得到安全的 `&T`，可离开 unsafe |
| 返回值 | 值本身 | 一个引用 |
| 空指针 | segfault | 返回 `None`（优雅处理） |

#### 替换、拷贝与转换

```rust
let mut x = 10i32; let mut y = 20i32;
let px: *mut i32 = &mut x; let py: *mut i32 = &mut y;

unsafe {
    ptr::swap(px, py);                   // 交换两位置值
    let old = ptr::replace(px, 99);      // 替换，返回旧值
    
    // 批量拷贝
    let src = [1i32, 2, 3, 4];
    let mut dst = [0i32; 4];
    ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), 4);  // 不重叠（快）
    ptr::copy(dst.as_ptr().add(1), dst.as_mut_ptr(), 3);          // 可重叠（慢）
    
    // 类型转换
    let p: *const u32 = &0xDEAD_BEEFu32;
    let byte: *const u8 = p.cast::<u8>();  // 同 as *const u8
    println!("{:02x}", *byte);             // 在小端序机上是 EF
}
```

#### 常用方法速查

| 方法 | 签名 | 作用 |
|------|------|------|
| `.add(n)` | `*const/mut T -> *const/mut T` | 前进 n 个元素 |
| `.sub(n)` | `*const/mut T -> *const/mut T` | 后退 n 个元素 |
| `.offset(n)` | `*const/mut T -> *const/mut T` | 偏移（可为负） |
| `.byte_add(n)` | `*const/mut T -> *const/mut T` | 按字节偏移 |
| `.read()` | `*const T -> T` | 位级拷贝（非 Copy 也可用） |
| `.write(val)` | `*mut T` | 位级写，不要求已初始化 |
| `.as_ref()` | `*const T -> Option<&T>` | 转安全引用，空指针安全 |
| `.as_mut()` | `*mut T -> Option<&mut T>` | 转安全可变引用 |
| `.cast::<U>()` | `*const T -> *const U` | 类型转换 |
| `ptr::swap(x, y)` | `*mut T, *mut T` | 交换两位置值 |
| `ptr::replace(dst, v)` | `*mut T, T -> T` | 替换并返回旧值 |
| `ptr::copy(s, d, n)` | `*const T, *mut T, usize` | 可重叠批量拷贝 |
| `ptr::copy_nonoverlapping` | 同上 | 禁止重叠（快） |
| `ptr::drop_in_place(p)` | `*mut T` | 手动 drop（不释放内存） |
| `ptr::null()` | `-> *const T` | 创建空指针 |
| `ptr::null_mut()` | `-> *mut T` | 创建可变空指针 |
| `p.is_null()` | `-> bool` | 判空 |

### MaybeUninit\<T\> — 延迟初始化

标准库中处理未初始化/部分初始化数据的基础设施。**为什么需要它？** 因为你不能声明一个未初始化的 `String` 或 `Vec`——Rust 要求所有值在使用前必须初始化。但手动管理内存时，你常常需要"先分配，后初始化"：

```rust
use std::mem::MaybeUninit;

// 场景1：栈上分批初始化
let mut buf: [MaybeUninit<u8>; 4] = [MaybeUninit::uninit(); 4];
buf[0].write(0xDE);
buf[1].write(0xAD);
buf[2].write(0xBE);
buf[3].write(0xEF);
let buf: [u8; 4] = unsafe { std::mem::transmute(buf) };

// 场景2：Vec 预分配（Vec 内部也这样）
let mut vec = Vec::with_capacity(10);
let spare = vec.spare_capacity_mut();  // &mut [MaybeUninit<T>]
for slot in spare.iter_mut() {
    slot.write(42);
}
unsafe { vec.set_len(10); }

// 场景3：避免重复 Drop
let mut v: MaybeUninit<Vec<u32>> = MaybeUninit::uninit();
v.write(vec![1, 2, 3]);
let v = unsafe { v.assume_init() };  // 必须保证已初始化
```

> ⚠️ `assume_init()` 调用前**必须**保证值已正确初始化，否则是 UB。

### UnsafeCell\<T\> — 内部可变性的基石

`UnsafeCell<T>` 是 `Cell<T>` 和 `RefCell<T>` 的底层原语。**它解决的问题：** 编译器默认假设 `&T` 指向的数据不会被修改（这允许它做别名分析优化）。但有时你需要通过 `&self` 修改内部状态——UnsafeCell 就是告诉编译器"别优化这里"的标记：

```rust
use std::cell::UnsafeCell;

struct MyCell<T> {
    value: UnsafeCell<T>,
}

impl<T> MyCell<T> {
    pub fn new(v: T) -> Self { MyCell { value: UnsafeCell::new(v) } }

    pub fn get(&self) -> &T       { unsafe { &*self.value.get() } }
    pub fn set(&self, v: T)      { unsafe { *self.value.get() = v; } }
}
```

> 💡 除非你在实现 `Cell`/`RefCell`/`Mutex` 这样的底层同步原语，否则**不应该直接使用 `UnsafeCell`**——用 `Cell` 或 `RefCell` 代替。

### union — C 风格的联合体

**为什么需要 union？** Rust 的 `enum` 自带判别值（discriminant），足够大多数场景。但两种情况下必须用 union：
1. FFI 中对应 C 的 union（C union 没有判别值）
2. 做位级别的 reinterpret cast（如在 `f32` 和 `u32` 之间转换位模式）

```rust
#[repr(C)]
union FloatOrInt {
    f: f32,
    i: u32,
}

let u = FloatOrInt { f: 3.14 };
unsafe {
    println!("作为 f32: {}", u.f);     // 3.14
    println!("内部位: {:#x}", u.i);    // 不安全的位模式读取
}

// FFI 中的 tagged union 模式
#[repr(C)]
union CValue { int_val: i32, float_val: f32 }

#[repr(C)]
struct TaggedValue {
    tag: u8,       // 0 = int, 1 = float
    value: CValue,
}
```

> ⚠️ union 没有运行时标签——你需要自己维护一个标记（如上例的 `tag`），否则读取非活跃字段是 UB。

---

## 三、代码层：具体怎么写

### 调用 unsafe 函数

```rust
unsafe fn dangerous() { /* ... */ }

unsafe {
    dangerous();  // 必须在 unsafe 块中调用
}
```

> 📘 *Rust 2024 Edition 中，`unsafe_op_in_unsafe_fn` 默认 warn——即使在 `unsafe fn` 内部，unsafe 操作也必须明确包在 `unsafe {}` 块中：*

```rust
unsafe fn process(ptr: *const u8) {
    println!("开始处理...");           // safe 操作可直接写
    unsafe { println!("值: {}", *ptr); } // unsafe 操作需要显式块
}
```

### 禁止 `static mut` 引用

> 📘 *Rust 2024 Edition 禁止直接引用 `static mut`，必须通过原始指针：*

```rust
static mut COUNTER: u32 = 0;

fn increment() {
    unsafe {
        let ptr = &raw mut COUNTER;  // ✅ 用 &raw mut 获取原始指针
        *ptr += 1;
        // ❌ let r = &COUNTER;  // 2024 Edition 不再允许
    }
}
```

### FFI 实战

### 场景说明

什么时候需要 FFI？

| 场景 | 示例 |
|------|------|
| 调用已有的 C/C++ 库 | 调用 SQLite、OpenSSL、ffmpeg |
| 调用操作系统 API | Linux syscall、Windows WinAPI |
| 需要极致性能，用 C 写热点 | Rust 调用手写 SIMD 的 C 函数 |
| 把 Rust 嵌入现有 C/C++ 项目 | 用 Rust 重写性能瓶颈模块 |
| 跨语言插件系统 | 游戏引擎的脚本/插件接口 |

### 调用 C 函数

> 📘 *Rust 2024 Edition：`extern` 块必须加 `unsafe`。*

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

### C 类型映射表

Rust 的 `std::ffi` 提供了与 C 类型精确对应的类型别名：

| C 类型 | Rust 类型 | 说明 |
|--------|----------|------|
| `int` | `c_int` | 至少 16 位，通常 32 位 |
| `unsigned int` | `c_uint` | |
| `char` | `c_char` | C 的 char（不是 Rust 的 char！） |
| `signed char` | `c_schar` | |
| `unsigned char` | `c_uchar` | |
| `short` | `c_short` | |
| `long` | `c_long` | 32/64 位，平台相关 |
| `long long` | `c_longlong` | 至少 64 位 |
| `float` | `c_float` | |
| `double` | `c_double` | |
| `void` | `c_void` | |
| `size_t` | `usize` | |
| `ptrdiff_t` | `isize` | |
| `NULL` | `std::ptr::null()` / `null_mut()` | |
| `void*` | `*mut c_void` | |
| `const char*` | `*const c_char` | C 字符串，需转换 |
| `bool` (C99 `_Bool`) | `bool` | Rust 的 bool 与 C 的 _Bool 兼容 |

### 字符串：CString / CStr

C 字符串是以 `\0` 结尾的字节序列，与 Rust 的 `&str`（UTF-8，带长度）完全不同：

```rust
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

// Rust → C：创建以 \0 结尾的字符串
let rust_str = "hello";
let c_str = CString::new(rust_str).unwrap();      // 检查没有内部 \0
let ptr: *const c_char = c_str.as_ptr();           // 传给 C 函数

// C → Rust：解析 C 返回的字符串
unsafe extern "C" { fn get_version() -> *const c_char; }

unsafe {
    let ptr = get_version();
    if !ptr.is_null() {
        let c_str = CStr::from_ptr(ptr);            // 不拷贝，只包装
        let rust_str = c_str.to_str().unwrap();     // 验证 UTF-8
        println!("版本: {}", rust_str);
    }
}

// 如果 C 端分配了内存，你需要对应的释放函数
unsafe extern "C" { fn free_string(s: *mut c_char); }
unsafe {
    let s = get_version();
    println!("{}", CStr::from_ptr(s).to_str().unwrap());
    free_string(s as *mut c_char);  // C 分配，C 释放
}
```

### 链接 C 库

```rust
// 链接系统库
#[link(name = "m")]     // libm (math library)
unsafe extern "C" {
    fn cos(x: f64) -> f64;
}

// 链接自定义 C 代码（放在 src/ 同目录或用 build.rs 编译）
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

### #[repr(C)] 深入 — 与 C 内存布局一致

**核心规则：Rust 默认布局不保证与 C 兼容。** `#[repr(C)]` 强制按 C ABI 排列：

```rust
// Rust 默认布局（编译器可重排字段优化内存）
struct RustLayout { a: u8, b: u32, c: u16 }
// 可能只有 8 bytes（编译器优化了填充）

// C 布局（严格按声明顺序，带对齐填充）
#[repr(C)]
struct CLayout {
    a: u8,    // offset 0
    // _pad1: [u8; 3]  // 隐含填充
    b: u32,   // offset 4
    c: u16,   // offset 8
    // _pad2: [u8; 2]  // 隐含填充（末尾对齐到 4）
}  // sizeof = 12 bytes

// 枚举带判别值
#[repr(C)]
enum Status { Ok = 0, Error = -1, Timeout = 2 }
```

**FFI 中必须用 `#[repr(C)]` 的场景：**
- 传给 C 的 struct / union
- 通过 `extern "C"` 跨边界传递的任何复合类型

> ⚠️ 不要用 `#[repr(Rust)]` 的 `enum` 传给 C——Rust 枚举的判别值+变体数据布局不与任何 C 模式对应。

### 回调函数 — C 调用 Rust

```rust
unsafe extern "C" {
    fn set_callback(cb: unsafe extern "C" fn(i32));
}

unsafe extern "C" fn my_handler(code: i32) {
    println!("收到事件码: {}", code);
}

fn main() {
    unsafe { set_callback(my_handler); }
}
```

**带 context 的回调（模仿 C 的 `void* user_data` 模式）：**

```rust
use std::ffi::c_void;

unsafe extern "C" {
    fn register_handler(
        cb: unsafe extern "C" fn(*mut c_void, i32),
        user_data: *mut c_void,
    );
}

struct Handler { name: String, count: u32 }

unsafe extern "C" fn rust_handler(user_data: *mut c_void, event: i32) {
    let handler = &mut *(user_data as *mut Handler);
    handler.count += 1;
    println!("[{}] 事件 {} (第 {} 次)", handler.name, event, handler.count);
}

let mut handler = Box::new(Handler { name: "主处理器".into(), count: 0 });
unsafe {
    register_handler(rust_handler, &mut *handler as *mut Handler as *mut c_void);
}
```

### cbindgen — 自动生成 C 头文件

当 Rust 库需要被 C/C++ 调用时，[`cbindgen`](https://github.com/mozilla/cbindgen) 从 Rust 源码自动生成 C 头文件：

```bash
cargo install cbindgen
cbindgen --config cbindgen.toml --crate mylib --output mylib.h
```

```toml
# cbindgen.toml
language = "C"
include_guard = "MYLIB_H"
autogen_warning = "/* 自动生成，请勿手工编辑 */"
```

---

## 四、实践层：怎么用好

### 用 miri 检测未定义行为

[miri](https://github.com/rust-lang/miri) 是 Rust 的 MIR 解释器，能检测 unsafe 代码中的 UB：

```bash
rustup +nightly component add miri
cargo +nightly miri test       # 用 miri 运行测试
cargo +nightly miri run        # 用 miri 运行程序
```

**miri 能检测：** 悬垂指针、use-after-free、读取未初始化内存、数据竞争、未对齐解引用、无效枚举值、违反别名规则。

```rust,ignore
let mut v = vec![1, 2, 3];
let ptr = v.as_mut_ptr();
v.push(4);  // 可能 realloc，ptr 悬垂
unsafe { *ptr = 42; }  // ← miri 捕获：use-after-free
```

> 💡 如果你的项目包含 unsafe 代码，在 CI 中添加 `cargo +nightly miri test`。

### 2024 Edition unsafe 变化速查

| 项目 | 旧写法 | 2024 Edition 写法 |
|------|--------|------------------|
| extern 块 | `extern "C" { }` | `unsafe extern "C" { }` |
| no_mangle | `#[no_mangle]` | `#[unsafe(no_mangle)]` |
| export_name | `#[export_name = "x"]` | `#[unsafe(export_name = "x")]` |
| unsafe fn 内操作 | 隐式 unsafe | 推荐显式 `unsafe {}` |
| static mut 引用 | `&COUNTER` (warning) | 必须用 `&raw mut COUNTER` |

### 何时使用 unsafe

| 场景 | 是否用 unsafe |
|------|-------------|
| 调用 C 函数（FFI） | ✅ 需要 |
| 直接操作硬件/内存映射 I/O | ✅ 需要 |
| 实现底层数据结构（如 `Vec` 内部） | ✅ 需要 |
| 性能优化（绕过边界检查） | ⚠️ 衡量后再用 |
| 规避借用检查器 | ❌ 几乎总是坏主意 |
| "求编译器别报错了" | ❌ 重新设计你的代码 |

---

## 五、示例层

### 简单安全抽象：边界检查的 set_element

```rust
pub fn set_element(slice: &mut [i32], index: usize, value: i32) -> bool {
    if index >= slice.len() {
        return false;  // 安全的边界检查
    }
    unsafe {
        *slice.as_mut_ptr().add(index) = value;
    }
    true
}
```

调用者不接触任何 unsafe，但内部用裸指针绕过了标准索引检查——因为你自己已经做过了。

### 进阶：安全的动态数组（RawVec）

这是 `Vec<T>` 的骨架——外部 API 全部安全，内部用 `alloc`、`copy_nonoverlapping`、`drop_in_place` 等 unsafe 操作：

```rust
use std::alloc::{alloc, dealloc, Layout};

pub struct RawVec<T> {
    ptr: *mut T,
    cap: usize,
    len: usize,
}

impl<T> RawVec<T> {
    pub fn new() -> Self {
        RawVec { ptr: std::ptr::null_mut(), cap: 0, len: 0 }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.cap { self.grow(); }
        unsafe { self.ptr.add(self.len).write(value); }
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.ptr.add(index)) }
        } else { None }
    }

    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 4 } else { self.cap * 2 };
        let layout = Layout::array::<T>(new_cap).unwrap();
        unsafe {
            let new_ptr = alloc(layout) as *mut T;
            if !self.ptr.is_null() {
                std::ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
                let old_layout = Layout::array::<T>(self.cap).unwrap();
                dealloc(self.ptr as *mut u8, old_layout);
            }
            self.ptr = new_ptr;
            self.cap = new_cap;
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                for i in 0..self.len {
                    std::ptr::drop_in_place(self.ptr.add(i));
                }
                let layout = Layout::array::<T>(self.cap).unwrap();
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}
```

## 练习

1. 写一个函数，用原始指针交换两个 `i32` 的值
2. 用 `alloc` 和 `dealloc` 手动管理一块内存，写入值再释放
3. 实现一个安全的 `SliceBuf` 封装：内部用 unsafe 原始指针操作，对外提供安全的 `read(&self, index) -> Option<T>` 和 `write(&mut self, index, value)`
4. 调用 C 标准库的 `malloc` 和 `free`，注意类型映射和字符串转换

---

← [第 21 章：测试](./21-testing.md) | [返回目录](./README.md) | → [附录 A：工具链](./appendix-a-tools.md)
