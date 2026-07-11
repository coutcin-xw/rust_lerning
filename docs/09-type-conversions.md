# 第 9 章：类型转换与运算符重载

## 学习目标

- 掌握 Rust 的类型转换 trait：`From`/`Into`、`TryFrom`/`TryInto`、`AsRef`/`AsMut`
- 理解 `Borrow<T>` 与 `AsRef<T>` 的区别
- 学会用 `std::ops` 中的 trait 实现运算符重载
- 实现 `Add`、`Sub`、`Index` 等常见运算符
- 理解运算符重载的最佳实践和陷阱

## 类型转换 — From / Into / TryFrom / AsRef

在 Rust 中，类型转换不是隐式的——你必须显式声明。这避免了 C/C++ 中隐式类型转换带来的 bug。Rust 通过 trait 来标准化类型转换。

### From\<T\> / Into\<T\> — 不会失败的转换

```rust
// 实现 From，自动获得 Into
impl From<u32> for UserId {
    fn from(id: u32) -> Self {
        UserId(id)
    }
}

// 使用 From
let id = UserId::from(42);

// Into 自动可用（From 的反向）
let id: UserId = 42.into();       // Rust 推断目标类型
let id = <UserId as From<u32>>::from(42);  // 完全限定语法（很少用）
```

> 💡 **标准模式：为你的类型实现 `From<T>`，调用者用 `.into()`。** `?` 操作符就依赖 `From` trait 来自动转换错误类型——当你写 `let f = File::open("a.txt")?;` 时，`io::Error` 通过 `From<io::Error>` 自动转成你的错误类型。

**在哪里 `From` 被自动调用：**
- `?` 操作符转换错误类型
- `String::from("hello")`、`Vec::from([1, 2, 3])`
- 函数参数（`fn f<T: Into<String>>(s: T)` 这样写接受 `&str`、`String` 等）
- `collect()` 时从迭代器构建集合

### TryFrom\<T\> / TryInto\<T\> — 可失败的转换

```rust
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
struct UserId(u32);

impl TryFrom<i32> for UserId {
    type Error = &'static str;

    fn try_from(id: i32) -> Result<Self, Self::Error> {
        if id > 0 {
            Ok(UserId(id as u32))
        } else {
            Err("用户 ID 必须为正数")
        }
    }
}

// 使用
assert_eq!(UserId::try_from(42), Ok(UserId(42)));
assert!(UserId::try_from(-1).is_err());

// TryInto 自动可用
let id: Result<UserId, _> = 42_i32.try_into();
let id: Result<UserId, _> = (-5_i32).try_into();  // Err("用户 ID 必须为正数")

// 典型用法：
fn create_user(raw_id: i32) -> Result<User, &'static str> {
    let id = UserId::try_from(raw_id)?;  // ? 在这里传播错误
    Ok(User { id, /* ... */ })
}
```

### AsRef\<T\> / AsMut\<T\> — 零成本的引用转换

`AsRef` 不是"转换"——它是获取一个指向内部数据的引用。成本为零（编译器优化后就是一个指针拷贝）。

```rust
// 让函数接受任意能转为 &str 的参数
fn is_hello<T: AsRef<str>>(s: T) -> bool {
    s.as_ref() == "hello"
}

is_hello("hello");                    // &str -> &str（无操作）
is_hello(String::from("hello"));      // String -> &str
is_hello("hello".to_owned());         // String -> &str

// 典型用法：包装类型实现 AsRef
struct Config {
    path: String,
    timeout: u64,
}

impl AsRef<str> for Config {
    fn as_ref(&self) -> &str {
        &self.path
    }
}

// 好处：函数签名更灵活，不需要知道具体类型
fn load_config(path: impl AsRef<str>) -> Config {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path).unwrap();
    // 解析 content...
    todo!()
}

// 调用者传什么都行
load_config("config.toml");
load_config(String::from("config.toml"));
load_config(&my_config);  // Config 本身实现了 AsRef<str>
```

### Borrow\<T\> — 与 AsRef 的区别

| | `AsRef<T>` | `Borrow<T>` |
|---|---|---|
| 用途 | 通用引用转换 | 用于 HashMap 查找（等价性保证） |
| 约束 | 任意转换 | `x.borrow() == y.borrow()` 必须等价于 `x == y` |
| 使用场景 | 函数参数灵活化 | `HashMap::get` 用 `&str` 查找 `String` 的 key |

```rust
use std::collections::HashMap;

let mut map: HashMap<String, u32> = HashMap::new();
map.insert("alice".to_string(), 100);

// 用 &str 查找，因为 String: Borrow<str>
let score = map.get("alice");  // 不需要先创建 String！
```

### 转换 trait 完整速查

| Trait | 方向 | 成本 | 使用场景 |
|-------|------|------|---------|
| `From<T>` | T → Self | 可能昂贵 | 所有权转换，`?` 依赖 |
| `Into<T>` | Self → T | 同 From | `.into()` 语法糖 |
| `TryFrom<T>` | T → Result\<Self\> | 可能昂贵 | 含校验的所有权转换 |
| `TryInto<T>` | Self → Result\<T\> | 同 TryFrom | `.try_into()` 语法糖 |
| `AsRef<T>` | &Self → &T | 零成本 | 获取内部引用 |
| `AsMut<T>` | &mut Self → &mut T | 零成本 | 获取内部可变引用 |
| `Borrow<T>` | &Self → &T | 零成本 | HashMap 等价查找 |
| `Deref` | &Self → &T | 零成本（自动） | 智能指针自动解引用 |

## 运算符重载

Rust 通过 `std::ops` 中的 trait 实现运算符重载。每个运算符对应一个 trait，实现 trait 即可使用运算符。

### 算术运算符

```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point { x: i32, y: i32 }

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

let p = Point { x: 1, y: 2 } + Point { x: 3, y: 4 };
assert_eq!(p, Point { x: 4, y: 6 });
```

**支持跨类型运算：**

```rust
// Point + i32（在 Point 右侧）
impl Add<i32> for Point {
    type Output = Point;
    fn add(self, rhs: i32) -> Point {
        Point { x: self.x + rhs, y: self.y + rhs }
    }
}

let shifted = Point { x: 1, y: 2 } + 10;  // { x: 11, y: 12 }

// &Point + &Point（借用版本，不消费操作数）
impl Add for &Point {
    type Output = Point;
    fn add(self, other: &Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 3, y: 4 };
let p3 = &p1 + &p2;  // p1 和 p2 仍然可用！
```

### 常用运算符对应的 trait

| 运算符 | Trait | 方法 | 备注 |
|--------|-------|------|------|
| `+` | `Add` | `add(self, rhs)` | |
| `-` | `Sub` | `sub(self, rhs)` | |
| `*` | `Mul` | `mul(self, rhs)` | |
| `/` | `Div` | `div(self, rhs)` | |
| `%` | `Rem` | `rem(self, rhs)` | |
| `-` (负号) | `Neg` | `neg(self)` | 一元运算符 |
| `!` (取反) | `Not` | `not(self)` | 一元运算符 |
| `+=` `-=` `*=` `/=` `%=` | `AddAssign` 等 | `add_assign(&mut self, rhs)` | 不需要返回类型 |
| `[]` (索引) | `Index` | `index(&self, index)` | 返回引用 |
| `[] =` (可变索引) | `IndexMut` | `index_mut(&mut self, index)` | 返回可变引用 |
| `==` `!=` | `PartialEq` | `eq`, `ne` | 可用 `#[derive]` |
| `<` `>` `<=` `>=` | `PartialOrd` | `partial_cmp` | 可用 `#[derive]` |

### 索引运算符 — 自定义 `[]` 语法

```rust
use std::ops::{Index, IndexMut};

struct Grid { cells: Vec<Vec<char>> }

impl Index<(usize, usize)> for Grid {
    type Output = char;
    fn index(&self, (row, col): (usize, usize)) -> &char {
        &self.cells[row][col]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut char {
        &mut self.cells[row][col]
    }
}

let mut grid = Grid { cells: vec![vec!['a', 'b'], vec!['c', 'd']] };
println!("{}", grid[(0, 1)]);  // 'b'
grid[(1, 0)] = 'X';
```

### 运算符的最佳实践

> ⚠️ **不要滥用运算符重载。** 只在运算语义明确时使用。例如 `Point + Point` 有意义，但 `User + User` 没有。如果加法语义不明确，用普通方法（如 `user.merge(other)`）。

> ⚠️ **实现 `AddAssign` 时通常也要实现 `Add`。** 标准库的 blanket impl 会自动从 `Add + Clone` 推导 `AddAssign`，但手动实现的顺序反过来不会。

> ⚠️ **注意 `self` vs `&self`。** 算术运算符通常消费 `self`（`fn add(self, rhs)`），这意味着操作后原值被移动。如果要保留原值，要么用 `Copy` 类型，要么为 `&T` 实现运算符。

## 练习

1. 为你的自定义类型实现 `From<u32>` 和 `TryFrom<i32>`
2. 设计一个 `Temperature` 结构体，实现摄氏度和华氏度的双向 `From` 转换
3. 为 `Point` 结构体实现 `Add`、`Sub`、`Index` trait
4. 尝试为一个自定义类型实现 `Display` trait，用 `{}` 格式化输出

---

← [第 8 章：Trait 与泛型](./08-trait-generics.md) | [返回目录](./README.md) | → [第 10 章：生命周期](./10-lifetime.md)
