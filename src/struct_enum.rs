// ============================================================
// 04_struct_enum.rs - 结构体和枚举
// 对比: C++ struct / Java class -> Rust struct
// 对比: Java enum -> Rust enum（更强大）
// ============================================================

// --- 结构体 ---
#[derive(Debug, Clone)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn struct_basics() {
    // 创建实例
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1: {:?}", user1);

    // 可变实例
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("newemail@example.com");

    // 结构体更新语法（类似JS ...spread）
    let user3: User = User {
        email: String::from("third@example.com"),
        ..user1.clone() // 其余字段从user1复制
    };

    // 元组结构体
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);
    println!("Color: {} {} {}", black.0, black.1, black.2);

    // 单元结构体（类似Go）
    struct AlwaysEqual;
    let _always_equal = AlwaysEqual;
}

// --- 方法语法 ---
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 第一个参数是 &self（不可变引用）
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 第一个参数是 &mut self（可变引用）
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // 关联函数（类似静态方法）
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // 方法可以同时使用 &self 和 &mut self 的重载吗？
    // 不行，会冲突。这里用不同的名字
}

fn method_syntax() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("面积: {}", rect.area());

    let mut rect2 = Rectangle {
        width: 10,
        height: 20,
    };
    rect2.scale(2);
    println!("放大后面积: {}", rect2.area());

    let sq = Rectangle::square(5);
    println!("正方形: {:?}", sq);
}

// --- 枚举 ---
#[derive(Debug)]
enum Direction {
    Up, // 类似其他语言的枚举
    Down,
    Left,
    Right,
}

fn enum_basics() {
    let dir = Direction::Up;

    // match必须处理所有情况
    match dir {
        Direction::Up => println!("向上"),
        Direction::Down => println!("向下"),
        Direction::Left => println!("向左"),
        Direction::Right => println!("向右"),
    }

    // if let简化单分支
    if let Direction::Up = dir {
        println!("正在向上!");
    }
}

// --- 枚举携带数据 ---
#[derive(Debug)]
enum Message {
    Quit,                    // 无数据
    Move { x: i32, y: i32 }, // 匿名结构体
    Write(String),           // 单个字符串
    ChangeColor(u8, u8, u8), // 多个值
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("移动到({}, {})", x, y),
            Message::Write(s) => println!("写入: {}", s),
            Message::ChangeColor(r, g, b) => println!("颜色: {} {} {}", r, g, b),
        }
    }
}

fn enum_with_data() {
    let m = Message::Move { x: 10, y: 20 };
    m.call();

    let m = Message::Write(String::from("hello"));
    m.call();
}

// --- Option枚举（替代null）---
// Rust没有null，但有Option<T>
fn option_demo() {
    // Option<T> 是标准库定义的
    // enum Option<T> { Some(T), None }

    let some_number: Option<i32> = Some(5);
    let absent_number: Option<i32> = None;

    // 使用match处理
    match some_number {
        Some(n) => println!("有值: {}", n),
        None => println!("无值"),
    }

    // 使用if let
    if let Some(n) = absent_number {
        println!("有值: {}", n);
    } else {
        println!("无值（None）");
    }

    // 常用方法
    let x = Some(5);
    let y = x.unwrap_or(0); // 如果是None返回0，否则返回Some中的值
    println!("y = {}", y);
}

// --- 练习：枚举和模式匹配 ---
#[derive(Debug)]
enum Shape {
    Circle(f64),         // 半径
    Rectangle(f64, f64), // 宽、高
    Triangle(f64, f64),  // 底、高，简化计算
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle(b, h) => b * h / 2.0,
    }
}

fn shape_demo() {
    let circle = Shape::Circle(2.0);
    let rect = Shape::Rectangle(3.0, 4.0);

    println!("圆面积: {:.2}", area(&circle));
    println!("矩形面积: {:.2}", area(&rect));
}

pub fn run() {
    println!("\n========== 04_struct_enum ==========");
    struct_basics();
    method_syntax();
    enum_basics();
    enum_with_data();
    option_demo();
    shape_demo();
}
