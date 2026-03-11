// ============================================================
// 07_trait_generics.rs - Trait和泛型
// Trait类似Java接口 + C++抽象类
// 泛型类似C++模板 / Java泛型
// ============================================================

// --- Trait定义 ---
trait Summary {
    fn summarize(&self) -> String;

    // 默认实现
    fn summarize_author(&self) -> String {
        String::from("(Unknown)")
    }
}

trait Display {
    fn display(&self) -> String;
}

// --- 实现Trait ---
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

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
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound语法
fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// --- 多个Trait bound ---
fn notify3(item: &(impl Summary + Display)) {
    println!("{}", item.display());
    println!("Summary: {}", item.summarize());
}

// --- Trait返回类型 ---
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("rustlang"),
        content: String::from("Hello world!"),
        reply: false,
        retweet: false,
    }
}

// --- 泛型函数 ---
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn generic_functions() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("最大的数: {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("最大的字符: {}", result);
}

// --- 泛型结构体 ---
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn generic_structs() {
    let p1 = Point { x: 5, y: 10.0 };
    let p2 = Point { x: "hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3: x={}, y={}", p3.x, p3.y);
}

// --- 泛型方法 ---
impl<T> Point<T, f64> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &f64 {
        &self.y
    }
}

// --- where子句 ---
fn some_function<T, U>(t: &T, u: &U) -> String
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    // ...
    String::from("result")
}

// --- Trait对象（动态分发）---
// 类似C++虚函数/Java接口
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

// 返回trait对象
fn draw_shape(shape: &dyn Shape) {
    println!("面积: {:.2}", shape.area());
}

// 练习
fn shapes_demo() {
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
}

// --- 关联类型 ---
trait Iterator2 {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// impl Iterator2 for Counter {
//     type Item = u32;
//
//     fn next(&mut self) -> Option<Self::Item> { ... }
// }

// --- 默认泛型参数 ---
struct Wrapper<T = Vec<String>> {
    value: T,
}

impl Default for Wrapper<Vec<String>> {
    fn default() -> Self {
        Wrapper {
            value: vec![String::from("default")],
        }
    }
}

// --- 练习：实现trait ---
#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("最大的值是 x = {}", self.x);
        } else {
            println!("最大的值是 y = {}", self.y);
        }
    }
}

fn trait_exercise() {
    let p = Pair::new(5, 10);
    p.cmp_display();
}

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
