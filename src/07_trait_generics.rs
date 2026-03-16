// ============================================================
// 07_trait_generics.rs - Trait和泛型
// ============================================================
//
// 【核心概念】
// Trait: 定义共享行为（类似接口）
// 泛型: 参数化类型（类似模板）
//
// 【对比其他语言】
//
// Trait:
// - Java: 接口（interface），但Rust可以有默认实现
// - C++: 抽象类 + 纯虚函数，但trait更灵活
// - Go: 接口（隐式实现），Rust需要显式声明
// - Haskell: Typeclass（Rust trait的设计灵感）
//
// 泛型:
// - C++: 模板（template），编译期泛型
// - Java: 泛型（类型擦除）
// - C#: 泛型（运行时保留）
// - Rust: 单态化（编译期展开，零运行时开销）
//
// 【设计哲学】
// 1. 组合优于继承（Rust没有继承）
// 2. 零成本抽象
// 3. 编译期类型检查
// ============================================================

// --- Trait定义 ---
// Trait定义一组方法签名，表示某种能力

trait Summary {
    // 【抽象方法】
    // 没有实现，实现者必须提供
    fn summarize(&self) -> String;

    // 【默认实现】
    // 提供默认行为，实现者可以覆盖
    fn summarize_author(&self) -> String {
        String::from("(Unknown)")
    }

    // 默认方法可以调用抽象方法
    fn summarize_with_author(&self) -> String {
        format!("作者: {}", self.summarize_author())
    }
}

// 自定义的Display trait
trait Display {
    fn display(&self) -> String;
}

// --- 实现Trait ---
// 使用 impl Trait for Type 语法

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// 为NewsArticle实现Summary trait
impl Summary for NewsArticle {
    // 必须实现抽象方法
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    // 可以覆盖默认实现
    fn summarize_author(&self) -> String {
        format!("作者: {}", self.author)
    }
}

// 可以实现多个trait
impl Display for NewsArticle {
    fn display(&self) -> String {
        format!("[News] {}", self.headline)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// --- Trait作为参数 ---
// Trait可以作为函数参数类型

fn notify(item: &impl Summary) {
    // 【impl Trait语法】
    // 参数可以是任何实现了Summary的类型
    // 
    println!("Br这是语法糖，等价于下面的泛型写法eaking news! {}", item.summarize());
}

// 【Trait bound语法】
// 泛型参数约束
fn notify2<T: Summary>(item: &T) {
    // T: Summary 表示T必须实现Summary trait
    // 更灵活，可以用于多个参数
    println!("Breaking news! {}", item.summarize());
}

// 多个参数使用相同类型
fn notify_same<T: Summary>(item1: &T, item2: &T) {
    // item1和item2必须是相同类型
    println!("{} | {}", item1.summarize(), item2.summarize());
}

// --- 多个Trait bound ---
// 要求同时实现多个trait

fn notify3(item: &(impl Summary + Display)) {
    // + 语法：要求同时实现多个trait
    println!("{}", item.display());
    println!("Summary: {}", item.summarize());
}

// 泛型版本
fn notify4<T: Summary + Display>(item: &T) {
    println!("{} - {}", item.display(), item.summarize());
}

// --- Trait返回类型 ---
// 函数可以返回实现了某个trait的类型

fn returns_summarizable() -> impl Summary {
    // 【impl Trait返回值】
    // 返回类型只需要实现trait，调用者不需要知道具体类型
    //
    // 限制：只能返回单一类型
    // 以下代码错误：
    // if condition {
    //     return NewsArticle { ... }  // 错误！类型不同
    // } else {
    //     return Tweet { ... }
    // }

    Tweet {
        username: String::from("rustlang"),
        content: String::from("Hello world!"),
        reply: false,
        retweet: false,
    }
}

// 【何时使用impl Trait返回】
// 1. 闭包返回类型
// 2. 迭代器链
// 3. 隐藏复杂类型

// --- 泛型函数 ---
// 泛型让函数可以处理多种类型

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    // 【泛型约束】
    // T: PartialOrd 要求T可以比较大小
    // 如果不加约束，> 操作符无法使用

    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn generic_functions() {
    println!("=== 泛型函数 ===");

    // 【单态化】
    // 编译器为每种使用的类型生成专门的函数
    // largest::<i32> 和 largest::<char> 是两个不同的函数

    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("最大的数: {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("最大的字符: {}", result);

    // 【零成本抽象】
    // 泛型在编译期展开，运行时没有额外开销
    // 类似C++模板
}

// --- 泛型结构体 ---
// 结构体可以包含泛型类型参数

struct Point<T, U> {
    // T和U是类型参数
    // 可以有不同的类型
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // 【泛型方法】
    // 在impl块中声明类型参数

    // 消费self并创建新Point
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        // V和W是新的类型参数
        Point {
            x: self.x,  // T类型
            y: other.y, // W类型
        }
    }
}

fn generic_structs() {
    println!("=== 泛型结构体 ===");

    // 不同类型参数
    let p1 = Point { x: 5, y: 10.0 }; // Point<i32, f64>
    let p2 = Point { x: "hello", y: 'c' }; // Point<&str, char>

    let p3 = p1.mixup(p2);
    println!("p3: x={}, y={}", p3.x, p3.y);
}

// --- 泛型方法 ---
// 可以为特定类型参数组合实现方法

impl<T> Point<T, f64> {
    // 只为 y 是 f64 的 Point 实现
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &f64 {
        &self.y
    }
}

// --- where子句 ---
// 当约束太多时，使用where更清晰

fn some_function<T, U>(t: &T, u: &U) -> String
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    // 【where子句】
    // 将约束移到函数签名后面
    // 等价于：
    // fn some_function<T: Display + Clone, U: Clone + Debug>(...)
    //
    // 好处：
    // 1. 函数签名更清晰
    // 2. 约束可以换行
    String::from("result")
}

// --- Trait对象（动态分发）---
// 使用dyn关键字创建trait对象

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn draw_shape(shape: &dyn Shape) {
    // 【dyn Trait】
    // 动态分发的trait对象
    // 运行时确定调用哪个方法
    println!("面积: {:.2}", shape.area());
}

// 【静态分发 vs 动态分发】
// 静态分发（泛型）：
// - 编译期确定具体类型
// - 零运行时开销
// - 生成多份代码（代码膨胀）
//
// 动态分发（trait对象）：
// - 运行时确定具体类型
// - 有虚表查找开销
// - 只生成一份代码

fn shapes_demo() {
    println!("=== Trait对象 ===");

    // 存储不同类型的trait对象
    let shapes: Vec<&dyn Shape> = vec![
        &Circle { radius: 2.0 },
        &Rectangle {
            width: 3.0,
            height: 4.0,
        },
    ];

    for shape in shapes {
        draw_shape(shape);
    }

    // 【使用场景】
    // - 需要存储不同类型的集合
    // - 运行时确定类型
    // - 类型数量在编译期不确定
}

// --- 关联类型 ---
// Trait可以有关联类型，避免泛型参数膨胀

trait Iterator2 {
    // 【关联类型】
    // 每个实现者只需要指定一次Item类型
    // 而不是 trait Iterator<T> 需要在每个使用处指定
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// 【对比泛型trait】
// trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }
// 问题：每次使用都要指定T，如 Iterator<i32>

// 【关联类型的优势】
// 1. 代码更简洁
// 2. 避免重复指定类型
// 3. 一个类型只能有一个impl

// 示例实现
struct Counter {
    count: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator2 for Counter {
    type Item = usize; // 指定关联类型

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// --- 默认泛型参数 ---
// 可以为泛型参数提供默认值

struct Wrapper<T = Vec<String>> {
    // T 默认是 Vec<String>
    value: T,
}

impl Default for Wrapper<Vec<String>> {
    fn default() -> Self {
        Wrapper {
            value: vec![String::from("default")],
        }
    }
}

// --- Trait Bound示例 ---
// 实现特定trait bound的方法

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    // 为所有 Pair<T> 实现
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    // 只为实现了 Display 和 PartialOrd 的 T 实现
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("最大的值是 x = {}", self.x);
        } else {
            println!("最大的值是 y = {}", self.y);
        }
    }
}

// 【空白trait bound】
// 可以用trait bound有条件地实现方法
// 只有满足条件的类型才有这个方法

fn trait_exercise() {
    println!("=== Trait练习 ===");

    let p = Pair::new(5, 10);
    p.cmp_display();

    // Pair<String> 没有cmp_display方法（String实现了Display但需要PartialOrd）
    // 编译器会在编译期检查
}

// --- 高级Trait特性 ---
// 【Trait继承】
// trait Child: Parent { ... }
// 实现Child必须先实现Parent

// 【Supertrait】
// trait Widget: Draw + Clone { ... }
// 要求实现多个trait

// 【孤儿规则】
// 只能为当前crate的类型实现当前crate的trait
// 不能为外部类型实现外部trait
// 这保证了trait实现的一致性

//不是所有特征都能拥有特征对象，只有对象安全的特征才行。当一个特征的所有方法都有如下属性时，它的对象才是安全的：
//  - 方法的返回类型不能是 Self
//  - 方法没有任何泛型参数

pub fn run() {
    println!("\n========== 07_trait_generics ==========");

    let article = NewsArticle {
        headline: String::from("Rust发布新版本"),
        location: String::from("北京"),
        author: String::from("张三"),
        content: String::from("..."),
    };

    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Exciting news!"),
        reply: false,
        retweet: false,
    };

    notify(&article);
    notify(&tweet);

    generic_functions();
    generic_structs();
    shapes_demo();
    trait_exercise();
}
