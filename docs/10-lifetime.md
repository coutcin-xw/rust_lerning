# 第 10 章：生命周期

## 学习目标

- 理解生命周期标注的**真实含义**（描述关系，不是延长时间）
- 掌握 `'a` 在函数、结构体、方法中的使用
- 熟记三条省略规则，知道为什么多数场景不需要手动标注
- 理解 `'static`、`dyn Trait + 'a`、HRTB `for<'a>` 的含义和使用场景
- 能识别并修复常见生命周期编译错误

---

## 一、概念层：为什么有生命周期问题

看这个函数——编译器报错：

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
// 编译错误：expected named lifetime parameter
```

**编译器在困惑什么？**

返回的 `&str` 是从 `x` 来的还是从 `y` 来的？调用者拿到了一个引用，但这个引用到底能活多久？取决于它是从谁返回的。

```
调用者视角:
                          
  fn longest(x: &str, y: &str) -> &str
              ↑来自哪？  ↑来自哪？       ↑是谁的引用？
                                         编译器不知道！
```

**生命周期的本质不是"让变量活多久"，而是向编译器描述引用参数和返回值之间的关系。** 编译器用你的标注来验证这种关系是否安全——如果成立就通过，否则报错。

> 💡 生命周期标注不延长任何变量的实际存活时间。它只是把**引用之间已经存在的关系**明确写出来，交给编译器验证。标注是给编译器看的，不生成任何运行时代码。

---

## 二、语法层：`'a` 标注语法

```rust
// 基本形式：fn 函数名<'a>(参数: &'a T) -> &'a T
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

**`'a` 怎么读：**

```
'x 是 x 的生命周期，'y 是 y 的生命周期
'a 是 x 和 y 的"交集"——两个引用同时有效的区间
返回值必须活得不短于 'a
```

**配套的语法要点：**

```rust
// 函数参数 —— &'a T
fn foo<'a>(r: &'a str) -> &'a str { r }

// 结构体字段 —— 包含引用时，结构体自身也需要 'a
struct Excerpt<'a> { part: &'a str }

// impl 块 —— 声明和传递 'a
impl<'a> Excerpt<'a> {
    fn part(&self) -> &'a str { self.part }
}

// 多个生命周期参数 —— 各自声明
fn context<'a, 'b>(config: &'a str, request: &'b str) { }
```

### 生命周期 + 泛型

泛型和生命周期并列写在尖括号中，顺序任意：

```rust
use std::fmt::Display;

// 生命周期 + 单个泛型
fn first<'a, T>(slice: &'a [T]) -> &'a T { &slice[0] }

// 泛型带 trait bound（where 子句）
fn longest_with_msg<'a, T>(x: &'a str, y: &'a str, msg: T) -> &'a str
where
    T: Display,
{
    println!("{}", msg);
    if x.len() > y.len() { x } else { y }
}
```

### 生命周期 + Trait（直接写在 bound 里）

```rust
// T: 'a        —— 类型 T 包含的所有引用都活得 ≥ 'a
// T: Trait + 'a —— T 实现 Trait 且内部引用 ≥ 'a
// 'a: 'b       —— 'a 至少和 'b 一样长（生命周期子类型关系）

use std::fmt::Debug;

// 例子：T 是 Debug + 包含的引用 ≥ 'a
fn debug_ref<'a, T: Debug + 'a>(r: &'a T) {
    println!("{:?}", r);
}

// 例子：生命周期约束 —— 'a 必须比 'b 长
fn pick<'a, 'b: 'a>(first: &'a str, second: &'b str) -> &'a str {
    first  // 'a 是 'b 的子集，安全
}
```

### 生命周期 + 泛型 + Trait（完整组合）

```rust
use std::fmt::Display;

// 方式 1：where 子句（推荐，可读性最好）
fn announce<'a, T>(x: &'a str, info: T) -> &'a str
where
    T: Display + 'a,          // T 实现 Display，且含有的引用 ≥ 'a
{
    println!("{}", info);
    x
}

// 方式 2：直接写在尖括号里（简单场景）
fn announce2<'a, T: Display + 'a>(x: &'a str, info: T) -> &'a str {
    println!("{}", info);
    x
}
```

### trait 定义带生命周期 + where Self

`Self` 是 trait 内部表示"实现了这个 trait 的那个类型"的特殊标识——它不是泛型参数，但行为类似：

```rust
// trait 自身带生命周期参数
pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>;
}

// 等价写法——把 Self: Sized 移到 where 子句
pub trait Deserialize<'de>
where
    Self: Sized,     // Self = "实现这个 trait 的类型"
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>;
}
```

**`Self` 在这里是什么？**

```
当你写:  impl<'de> Deserialize<'de> for User
编译器:  在这个 impl 块中, Self = User
         where Self: Sized  → where User: Sized ✓

当你写:  impl<'de> Deserialize<'de> for str
编译器:  在这个 impl 块中, Self = str
         where Self: Sized  → where str: Sized ✗ (str 是 DST，不满足 Sized)
```

> 💡 `Self: Sized` 是 trait 中最常见的 where 约束——它说"只有 Sized 类型才能实现这个 trait"（排除 `str`、`dyn Trait` 等动态大小类型）。如果去掉这个约束，trait 就可以为 `str` 等类型实现，但方法签名中需要处理 `?Sized`。

### impl 块中的生命周期 + where Self

```rust
struct MyParser<'a, T> {
    data: &'a [T],
    pos: usize,
}

impl<'a, T> MyParser<'a, T>
where
    T: PartialEq + 'a,     // T 可比较 + 包含的引用 ≥ 'a
{
    fn next(&mut self) -> Option<&'a T> {
        if self.pos < self.data.len() {
            let r = &self.data[self.pos];
            self.pos += 1;
            Some(r)
        } else {
            None
        }
    }
}
```

**组合速查：**

| 写法 | 含义 | 常用场景 |
|------|------|---------|
| `<'a, T>` | 生命周期 `'a` + 泛型 `T` | 泛型函数返回引用 |
| `trait Trait<'a>` | trait 自身带生命周期参数 | `Deserialize<'de>`、`Borrow<'a>` |
| `where Self: Sized` | 限制实现者类型大小已知 | trait 定义中排除 DST |
| `T: 'a` | T 包含的引用都 ≥ `'a` | 保证 T 不包含悬垂引用 |
| `T: Trait + 'a` | T 实现 Trait 且引用 ≥ `'a` | 泛型 + trait + 生命周期组合 |
| `'a: 'b` | `'a` 至少和 `'b` 一样长 | 多生命周期约束 |
| `where T: Display + 'a` | Where 子句形式 | 复杂约束时推荐 |

---

## 三、机制层：生命周期如何工作

### 省略规则 — 为什么多数场景不需要手动标注

编译器大多数时候能自动推断，规则只有三条：

1. **每个引用参数各自获得一个生命周期**：`fn foo(x: &str, y: &str)` → `fn foo<'a, 'b>(x: &'a str, y: &'b str)`
2. **如果只有一个输入生命周期，所有输出都继承它**：`fn foo(x: &str) -> &str` → `fn foo<'a>(x: &'a str) -> &'a str`
3. **如果有 `&self`，`self` 的生命周期赋给所有输出**（方法中最常见）

这三条覆盖了 90% 的日常场景。当规则都失效时（如 `longest` 有两个输入参数），编译器才要求你手动标注。

### `'static` — 存活到程序结束

```rust
let s: &'static str = "我是静态字符串";  // 字符串字面量天生 'static

// 合法使用场景：
// - 字符串字面量
// - 全局常量
// - Box::leak 泄露的值
// - move 进线程的闭包（fn() + Send + 'static）
```

> ⚠️ **不要用 `'static` 来"修复"生命周期错误。** 编译器提示加 `'static` 时，99% 是你的设计有问题——重新思考所有权链条。

### `dyn Trait + 'a` — trait 对象上的生命周期

```rust
trait Draw { fn draw(&self); }

struct View<'a> { label: &'a str }
impl<'a> Draw for View<'a> { fn draw(&self) { println!("{}", self.label); } }

let name = String::from("主界面");

// ❌ Box<dyn Draw> 默认是 Box<dyn Draw + 'static> — View<'a> 不满足
// let v: Box<dyn Draw> = Box::new(View { label: &name });

// ✅ 显式放宽约束
let v: Box<dyn Draw + '_> = Box::new(View { label: &name });
//                             ^^ '_ = 编译器自动推断最短可行的生命周期
```

规则：
- `Box<dyn Trait>` 默认 `Box<dyn Trait + 'static>`
- `Box<dyn Trait + 'a>` 允许包含生命周期 ≥ `'a` 的引用
- `Box<dyn Trait + '_>` 是常用简写

### HRTB `for<'a>` — 当"所有生命周期"都需要时

**问题场景：**

```rust
// Deserialize<'a> 的含义：从存活期为 'a 的数据中反序列化
// 所以 T: Deserialize<'a> 中的 'a 必须是一个具体的生命周期

// 但是这个函数签名有问题：
fn get_json<'de, T: Deserialize<'de>>(&self, data: &'de str) -> Result<T> {
    // 'de 被 data 参数绑定——返回的 T 只能活和 data 一样长
    // 但实际上 serde 可能从临时缓冲区反序列化，data 很快就没了
}
```

`Deserialize` 的 `'de` 参数绑定到了函数的输入引用。但如果 T 不需要借用 data 中的数据（即 `T` 不包含引用字段），你希望 `Deserialize<'de>` 对**任意** `'de` 都成立——这就是 `for<'a>` 的用武之地：

```rust
// for<'a> 读作"对所有生命周期 'a"
async fn get_json<T: for<'a> Deserialize<'a>>(&self, path: &str) -> Result<T>
//                    ^^^^^^^^^^^^^^^^^^^^
//                    T 必须满足：不管 'a 是什么，Deserialize<'a> 都成立
```

**`for<'a>` 的含义：**

```
T: Deserialize<'a>          对于某个具体的 'a（和调用者绑定）
T: for<'a> Deserialize<'a>  对于所有可能的 'a（'a 可以是任意值）
                            等价于：T 不依赖于数据源的存活期
```

**HRTB 常见的位置：**

| 场景 | 写法 | 为什么需要 |
|------|------|-----------|
| serde 反序列化 | `T: for<'de> Deserialize<'de>` | T 不从输入数据借用，生命周期应该解耦 |
| 闭包类型 | `F: for<'a> Fn(&'a u32) -> &'a u32` | 闭包能处理任意存活期的引用 |
| 回调函数 | `cb: for<'a> Fn(&'a str)` | 回调要能接收所有传入的引用 |
| `DeserializeOwned` | `T: DeserializeOwned` | serde 提供的别名 = `for<'de> Deserialize<'de>` |

> 💡 **实践中的简化：** serde 提供了 `DeserializeOwned` trait，它就是 `for<'de> Deserialize<'de>` 的别称。大多数时候你不需要手写 `for<'a>`，遇到编译器建议 `T: DeserializeOwned` 时，知道它等价于 `for<'de> Deserialize<'de>` 就行。

---

## 四、代码层：实际使用模式

### 结构体中的生命周期

```rust
struct Excerpt<'a> {
    part: &'a str,  // part 引用的有效期 ≥ 结构体实例的生命周期
}

let novel = String::from("从前有座山。山上有座庙。");
let first = &novel[..9];  // "从前有座山。"
let excerpt = Excerpt { part: first };
// excerpt 不能比 novel 活得久——否则 part 就变成悬垂引用了
```

```
novel:     |═════════════|  (String 的作用域)
first:        |═══════|      (&str 引用，必须在 novel 内)
excerpt:      |═══════|      (Excerpt<'a>, part 的有效期≥它的有效期)
```

### 方法中的生命周期

```rust
impl<'a> Excerpt<'a> {
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("{}", announcement);
        self.part
    }
    // 省略规则第 3 条：&self 的生命周期自动赋给返回值
}
```

### 多个生命周期参数

```rust
// 哪些场景会需要多个 'a？
struct Context<'a, 'b> {
    config: &'a str,   // config 来自全局，活很久
    request: &'b str,   // request 来自网络，活很短
}
// 'a 和 'b 分别标注——request 不会把 config 的生命周期"拉短"

fn first_word<'a, 'b>(full: &'a str, delimiter: &'b str) -> &'a str {
    full.split(delimiter).next().unwrap_or(full)
    // 返回值只和 full 有关，和 delimiter 无关
    // 显式标出 'a 和 'b 防止编译器把返回值限制为 &delimiter 的生命周期
}
```

### 泛型 + Trait Bound + 生命周期

```rust
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,  // T 不含引用，不需要给它生命周期
{
    println!("公告: {}", ann);
    if x.len() > y.len() { x } else { y }
}
```

---

## 五、实践层

### 常见错误及修复

**错误 1：返回局部变量的引用**
```rust,ignore
fn bad() -> &str {
    let s = String::from("hello");
    &s  // ❌ s 在函数结束时被释放
}
```
修复：返回所有权（`String` 而非 `&str`）。

**错误 2：结构体引用比数据活得久**
```rust,ignore
fn create_holder() -> Holder {
    let s = String::from("data");
    Holder { data: &s }  // ❌ s 释放后 Holder.data 悬垂
}
```
修复：结构体持有所有权（用 `String` 而非 `&str`）。

**错误 3：trait 对象生命周期不匹配**
```rust,ignore
fn make_view(label: &str) -> Box<dyn Draw> {
    Box::new(View { label })  // ❌ View 不满足 dyn Draw 的 'static 默认
}
fn make_view<'a>(label: &'a str) -> Box<dyn Draw + 'a> {
    Box::new(View { label })  // ✅
}
```

### RPIT 生命周期捕获（2024 Edition）

> 📘 *2024 Edition 中 `impl Trait` 返回值自动捕获所有生命周期参数。*

```rust
// 2024 Edition：'a 被自动捕获
fn process<'a>(data: &'a str) -> impl Display {
    format!("处理: {}", data)
}

// 精确控制：use<..>
fn get_iter<'a, 'b>(x: &'a str, y: &'b str) -> impl Iterator<Item = char> + use<'a> {
    x.chars()  // 只捕获 'a，y 不能在迭代器中使用
}
```

### 常见陷阱

> ⚠️ **以为标注能让变量活得更久。** 标注只是描述关系，不改变任何实际存活时间。

> ⚠️ **错误信息 "does not live long enough"。** 检查：谁拥有这个数据？引用从哪来的？它们的作用域如何嵌套？

> ⚠️ **滥用 `'static`。** `'static` 不是"修复器"——如果是编译器建议，99% 的情况是你的设计有问题。

> ⚠️ **在结构体中不加生命周期。** 任何包含引用的结构体都需要声明 `'a`。但问问自己：真的需要引用吗？用所有权类型（`String` 而非 `&str`）通常更简单。

---

## 练习

1. 实现 `fn shortest<'a>(x: &'a str, y: &'a str) -> &'a str`——返回较短的那个
2. 定义一个 `Config<'a>` 结构体，包含一个 `path: &'a str` 字段，实现方法和关联函数
3. 写一个 RPIT 返回函数：接收 `&str`，返回 `impl Display`。尝试在 2024 Edition 下编译，验证生命周期自动捕获
4. 分析以下代码为什么不编译，并修复：

```rust
fn return_ref() -> &str {
    let s = String::from("hello");
    &s
}
```

---

← [第 9 章：类型转换与运算符重载](./09-type-conversions.md) | [返回目录](./README.md) | → [第 11 章：闭包](./11-closures.md)
