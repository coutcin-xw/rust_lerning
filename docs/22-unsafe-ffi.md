# 第 22 章：unsafe Rust 与 FFI

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

### `&raw` 操作符 — 安全地创建原始指针

Rust 2024 Edition 引入了 `&raw const` 和 `&raw mut` 操作符，替代旧的 `addr_of!`/`addr_of_mut!` 宏：

```rust
let mut x = 42;
let raw: *const i32 = &raw const x;   // 创建 *const
let raw_mut: *mut i32 = &raw mut x;   // 创建 *mut
```

`&raw` 不会触发借用——即使变量有未对齐、悬垂等情况也能安全使用。旧版 `&x as *const T` 在对齐错误时是 UB。

### NonNull\<T\> — 非空指针抽象

`NonNull<T>` 是一个保证非 null 的 `*mut T` 包装，编译器可以据此优化：

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

`NonNull` 是 `Option` 友好的——`Option<NonNull<T>>` 和 `*mut T` 大小相同（8 字节），没有额外开销。

### 指针运算与读写 — `*const T` / `*mut T` 的方法

原始指针本身没有借用检查，但提供了一套精细的 unsafe 操作 API 来读写和移动内存：

#### 偏移（算术运算）

```rust
use std::ptr;

let mut data = [10i32, 20, 30, 40, 50];
let base: *mut i32 = data.as_mut_ptr();

unsafe {
    // .add(n)：前进 n 个元素（不是字节！），达到第 n 个元素
    let third = base.add(2);           // 指向 data[2]
    println!("{}", *third);             // 30

    // .sub(n)：后退 n 个元素
    let first = base.add(4).sub(3);    // 指向 data[1]
    println!("{}", *first);             // 20

    // .offset(n)：与 add 相同，但 n 是 isize（可为负）
    let last = base.offset(4);         // 指向 data[4]
    println!("{}", *last);             // 50

    // .offset_from(other)：计算两个指针之间的元素数（isize）
    let dist = base.offset_from(base.add(3));
    println!("距离: {}", dist);        // -3 (base 在 add(3) 之前 3 个元素)

    // .byte_add(n) / .byte_offset(n)：按字节偏移
    let raw_bytes: *const u8 = base as *const u8;
    let second_byte = raw_bytes.byte_add(4);  // i32 占 4 字节，第 2 个 i32 的第一个字节
}
```

#### 读与写

```rust
let mut value = 42i32;
let p: *mut i32 = &mut value;

unsafe {
    p.write(100);             // 写入（不调用 drop，直接覆盖）
    let v = p.read();         // 读取值（不移动，产生一个副本）
    println!("{}", v);        // 100

    // as_ref / as_mut：安全地转为引用（需保证指针有效）
    let r: &i32 = p.as_ref().unwrap();  // Option<&T>
    let r: &mut i32 = p.as_mut().unwrap();
}
```

> ⚠️ `.read()` 读取值时是"位级拷贝"——不调用 `Copy`/`Clone`，也不会 drop 原位置的值。如果 T 有 Drop 实现，你需要自己管理：先 `read` 走值，再用不可丢弃的值（如 `MaybeUninit::uninit()`）覆盖原位，或者后续处理所有权。

#### 替换与交换

```rust
let mut x = 10i32;
let mut y = 20i32;
let px: *mut i32 = &mut x;
let py: *mut i32 = &mut y;

unsafe {
    ptr::swap(px, py);                 // 交换两个位置的值（x=20, y=10）

    let old = ptr::replace(px, 99);    // 将 *px 设为 99，返回旧值
    println!("旧值: {}", old);          // 20
    println!("新值: {}", *px);          // 99
}
```

#### 批量拷贝

```rust
let src = [1i32, 2, 3, 4];
let mut dst = [0i32; 4];

unsafe {
    // copy_nonoverlapping：源和目标不重叠（更快，不检查重叠）
    ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), 4);

    // copy：源和目标可以重叠（类似 C 的 memmove）
    // 用于需要支持重叠的场景，性能稍低
    ptr::copy(dst.as_ptr().add(1), dst.as_mut_ptr(), 3);
    // dst 现在 = [0, 1, 1, 2] （复制从 index=1 开始的 3 个元素到 index=0）
}

println!("{:?}", dst);
```

#### 指针转换

```rust
let value: u32 = 0xDEAD_BEEF;
let p: *const u32 = &value;

unsafe {
    // cast：改变指针的类型（不改变地址）
    let byte_ptr: *const u8 = p.cast::<u8>();

    // 读取第一个字节
    println!("{:02x}", *byte_ptr);  // 在小端序机器上是 EF

    // 直接转换：*const T as *const U（效果同 cast）
    let byte_ptr2 = p as *const u8;
}
```

#### 常用方法速查

| 方法 | 签名 | 作用 |
|------|------|------|
| `.add(n)` | `*const/mut T -> *const/mut T` | 前进 n 个元素 |
| `.sub(n)` | `*const/mut T -> *const/mut T` | 后退 n 个元素 |
| `.offset(n)` | `*const/mut T -> *const/mut T` | 偏移 n 个元素（可为负） |
| `.byte_add(n)` | `*const/mut T -> *const/mut T` | 按字节偏移 |
| `.read()` | `*const T -> T` | 位级拷贝读取 |
| `.write(val)` | `*mut T` | 位级写入（不 drop） |
| `.as_ref()` | `*const T -> Option<&T>` | 安全引用（需验证有效性） |
| `.as_mut()` | `*mut T -> Option<&mut T>` | 安全可变引用 |
| `.cast::<U>()` | `*const T -> *const U` | 类型转换 |
| `ptr::swap(x, y)` | `*mut T, *mut T` | 交换两指针的值 |
| `ptr::replace(dst, val)` | `*mut T, T -> T` | 替换并返回旧值 |
| `ptr::copy(src, dst, n)` | `*const T, *mut T, usize` | 允许重叠的批量拷贝 |
| `ptr::copy_nonoverlapping` | 同上 | 禁止重叠（更快） |
| `ptr::drop_in_place(p)` | `*mut T` | 手动 drop（不释放内存） |
| `ptr::null()` | `-> *const T` | 创建空指针 |
| `ptr::null_mut()` | `-> *mut T` | 创建可变空指针 |
| `p.is_null()` | `-> bool` | 判断是否空指针 |

### MaybeUninit\<T\> — 延迟初始化

标准库中处理未初始化/部分初始化数据的基础设施：

```rust
use std::mem::MaybeUninit;

// 场景1：在栈上分批初始化
let mut buf: [MaybeUninit<u8>; 4] = [MaybeUninit::uninit(); 4];
buf[0].write(0xDE);
buf[1].write(0xAD);
buf[2].write(0xBE);
buf[3].write(0xEF);
// 现在安全地转为 [u8; 4]
let buf: [u8; 4] = unsafe { std::mem::transmute(buf) };

// 场景2：Vec 内部使用——提前分配未初始化空间
let mut vec = Vec::with_capacity(10);
let spare = vec.spare_capacity_mut();  // &mut [MaybeUninit<T>]
for slot in spare.iter_mut() {
    slot.write(42);
}
unsafe { vec.set_len(10); }

// 场景3：从 MaybeUninit<Vec<T>> 取出 Vec<T>（不触发 Drop）
let mut v: MaybeUninit<Vec<u32>> = MaybeUninit::uninit();
v.write(vec![1, 2, 3]);
let v = unsafe { v.assume_init() };  // 必须确保已初始化
```

> ⚠️ `assume_init()` 调用前，**必须**保证值已正确初始化，否则是未定义行为（UB）。

### UnsafeCell\<T\> — 内部可变性的基石

`UnsafeCell<T>` 是 `Cell<T>` 和 `RefCell<T>` 的底层基础。它告诉编译器："这里的值可能通过共享引用被修改"：

```rust
use std::cell::UnsafeCell;

struct MyCell<T> {
    value: UnsafeCell<T>,
}

impl<T> MyCell<T> {
    pub fn new(v: T) -> Self {
        MyCell { value: UnsafeCell::new(v) }
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.value.get() }
    }

    pub fn set(&self, v: T) {
        unsafe { *self.value.get() = v; }  // 通过 &self 修改内部值
    }
}
```

> 💡 除非你在实现 `Cell`/`RefCell`/`Mutex` 这样的底层同步原语，否则**不应该直接使用 `UnsafeCell`**——用 `Cell` 或 `RefCell` 代替。

### union — C 风格的联合体

union 的所有字段共享同一块内存，访问字段必须在 `unsafe` 块中：

```rust
#[repr(C)]
union FloatOrInt {
    f: f32,
    i: u32,
}

let u = FloatOrInt { f: 3.14 };
unsafe {
    println!("作为 f32: {}", u.f);     // 3.14
    println!("作为 u32: {:#x}", u.i);  // 内部位表示
}

// 常见场景：FFI 中处理 C union
#[repr(C)]
union CValue {
    int_val: i32,
    float_val: f32,
}

#[repr(C)]
struct TaggedValue {
    tag: u8,       // 0 = int, 1 = float
    value: CValue,
}
```

> ⚠️ union 没有运行时标签来标识"当前活跃的是哪个字段"——你需要自己维护一个标记（如上例的 `tag`），否则读取非活跃字段是 UB。

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

### #[repr(C)] 深入 — 与 C 内存布局一致

FFI 的核心规则：**Rust 的默认布局不保证与 C 兼容**。`#[repr(C)]` 强制 struct/enum/union 按 C ABI 排列：

```rust
// Rust 默认布局（编译器可重排字段以优化内存）
struct RustLayout {
    a: u8,    // 1 byte
    b: u32,   // 4 bytes
    c: u16,   // 2 bytes
}  // 可能只有 8 bytes（编译器优化）

// C 布局（严格按声明顺序，带对齐填充）
#[repr(C)]
struct CLayout {
    a: u8,    // offset 0
    _pad1: [u8; 3],  // 隐含填充
    b: u32,   // offset 4
    c: u16,   // offset 8
    _pad2: [u8; 2],  // 隐含填充
}  // sizeof = 12 bytes（末尾对齐到 4）

// 枚举带判别值
#[repr(C)]
enum Status {
    Ok = 0,
    Error = -1,
    Timeout = 2,
}
```

**FFI 中必须用 `#[repr(C)]` 的场景：**
- 传给 C 函数的 struct 参数或返回值
- 通过 `extern "C"` 跨边界传递的任何复合类型
- `union`（用于和 C union 对应）

> ⚠️ 不要用 `#[repr(Rust)]` 的 `enum` 传给 C——Rust 枚举的内存布局（判别值 + 变体数据）不与任何 C 模式对应。

### 回调函数 — C 调用 Rust

```rust
// 声明一个接受回调的 C 函数
unsafe extern "C" {
    fn set_callback(cb: unsafe extern "C" fn(i32));
}

// Rust 中定义的函数，可以被 C 调用
unsafe extern "C" fn my_handler(code: i32) {
    println!("收到事件码: {}", code);
}

fn main() {
    unsafe {
        set_callback(my_handler);
    }
}
```

**传递 context / user_data：**

C 回调通常允许传递一个 `void*` 作为上下文：

```rust
use std::ffi::c_void;

unsafe extern "C" {
    fn register_handler(
        cb: unsafe extern "C" fn(*mut c_void, i32),
        user_data: *mut c_void,
    );
}

struct Handler {
    name: String,
    count: u32,
}

unsafe extern "C" fn rust_handler(user_data: *mut c_void, event: i32) {
    let handler = &mut *(user_data as *mut Handler);
    handler.count += 1;
    println!("[{}] 事件 {} (第 {} 次)", handler.name, event, handler.count);
}

// 使用：
let mut handler = Box::new(Handler { name: "主处理器".into(), count: 0 });
unsafe {
    register_handler(rust_handler, &mut *handler as *mut Handler as *mut c_void);
}
```

### cbindgen — 自动生成 C 头文件

当 Rust 库需要被 C/C++ 调用时，手动编写头文件容易出错。[`cbindgen`](https://github.com/mozilla/cbindgen) 可以从 Rust 源码自动生成 C 头文件：

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

它会自动处理 `#[repr(C)]` struct、`extern "C"` fn、enum 等，生成标准的 C 声明。

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

### 进阶示例：安全的动态数组

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
        if self.len == self.cap {
            self.grow();
        }
        unsafe {
            self.ptr.add(self.len).write(value);
        }
        self.len += 1;
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

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.ptr.add(index)) }
        } else {
            None
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                // 先 drop 所有元素
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

> 这就是 `Vec<T>` 的简化版——外部的 `push`、`get`、`Drop` 都是安全的，但内部用了 `alloc`、`copy_nonoverlapping`、`drop_in_place` 等 unsafe 操作。

## 用 miri 检测未定义行为

[miri](https://github.com/rust-lang/miri) 是 Rust 的 MIR 解释器，能检测 unsafe 代码中的**未定义行为（UB）**——这些错误在正常编译下可能不会立即崩溃，但会导致难以调试的 bug：

```bash
rustup +nightly component add miri
cargo +nightly miri test       # 用 miri 运行测试
cargo +nightly miri run        # 用 miri 运行程序
```

**miri 能检测的问题：**
- 悬垂指针/use-after-free
- 对已释放内存的读写
- 未对齐的指针解引用
- 数据竞争（data races）
- 读取未初始化的内存
- 无效的 enum 判别值
- 违反指针别名规则

```rust,ignore
// 这段代码在正常编译下可能正常工作，但 miri 会报告错误：
let mut v = vec![1, 2, 3];
let ptr = v.as_mut_ptr();
v.push(4);  // 可能触发 realloc，ptr 变成悬垂指针
unsafe {
    *ptr = 42;  // ← miri 会捕获：use-after-free
}
```

> 💡 **CI 集成建议：** 如果你的项目包含 unsafe 代码，在 CI 中添加 `cargo +nightly miri test` 步骤。这是零成本的保险，能捕获绝大多数 unsafe 的 UB。

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

← [第 21 章：测试](./21-testing.md) | [返回目录](./README.md) | → [附录 A：工具链](./appendix-a-tools.md)
