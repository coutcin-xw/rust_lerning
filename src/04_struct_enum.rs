// ============================================================
// 04_struct_enum.rs - 结构体和枚举
// ============================================================
//
// 【核心概念】
// 结构体（struct）：组合相关数据
// 枚举（enum）：表示一组可能的值
//
// 【对比其他语言】
//
// 结构体：
// - C/C++: struct类似，但Rust没有继承
// - Java: class类似，但Rust的struct只有数据
// - Go: struct几乎一样
//
// 枚举：
// - C/C++: 枚举只是整数常量
// - Java: 枚举是类，可以携带数据（类似Rust）
// - Scala: case class和sealed trait
//
// 【Rust的特点】
// 1. 结构体和方法分离（impl块）
// 2. 枚举可以携带任意数据
// 3. 没有继承，用trait实现多态
// 4. 模式匹配与枚举完美配合
// ============================================================

// --- 结构体 ---
// 【derive属性】
// 自动实现常用的trait
// Debug: 可以用{:?}格式化打印
// Clone: 可以.clone()复制
#[derive(Debug, Clone)]
struct User {
    // 字段默认是私有的（模块外不可访问）
    // 加 pub 可以公开
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn struct_basics() {
    println!("=== 结构体基础 ===");

    // 【创建实例】
    // 必须为所有字段赋值（没有部分初始化）
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 【Debug打印】
    // {:?} 使用Debug trait
    // {:#?} 美化打印（多行）
    println!("user1: {:?}", user1);

    // 【可变实例】
    // 整个实例是可变或不可变，不能只让部分字段可变
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("newemail@example.com");
    // 修改字段需要实例本身是mut

    // 【结构体更新语法】
    // 类似JS的 ...spread 操作符
    let user3: User = User {
        email: String::from("third@example.com"),
        ..user1.clone() // 其余字段从user1复制
                        // 注意：如果是移动类型（如String），会发生所有权转移
                        // ..user1 后 user1 可能部分或全部无效
    };
    println!("user3: {:?}", user3);

    // 【元组结构体】
    // 有名字的元组
    // 当需要给元组一个名字，或实现trait时使用
    struct Color(u8, u8, u8); // RGB颜色
    struct Point(f64, f64); // 2D坐标

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);
    println!("Color: {} {} {}", black.0, black.1, black.2);
    // 通过索引访问，而不是字段名

    // 【单元结构体】
    // 没有字段的结构体
    // 用于：
    // - 实现trait但没有数据
    // - 作为类型标记
    struct AlwaysEqual;
    let _always_equal = AlwaysEqual;
}

// --- 方法语法 ---
// Rust的方法与结构体分离，放在impl块中
// 类似Go的receiver，但语法不同

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 【方法】
    // 第一个参数是 self（或 &self, &mut self）

    // &self - 不可变借用，只读
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // &mut self - 可变借用，可修改
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // 【关联函数】
    // 没有 self 参数，类似静态方法
    // 常用作构造函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // 多个impl块是允许的
    // 可以在不同的impl块中添加方法
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn method_syntax() {
    println!("=== 方法语法 ===");

    // 【调用方法】
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // 方法调用使用 .
    println!("面积: {}", rect.area());
    println!("周长: {}", rect.perimeter());

    // 【调用关联函数】
    // 使用 :: 而不是 .
    let sq = Rectangle::square(5);
    println!("正方形: {:?}", sq);

    // 【自动引用】
    // Rust会自动添加 & 或 &mut
    // rect.area() 等价于 (&rect).area()
    // 编译器知道方法需要什么类型的self

    // 【可变方法】
    let mut rect2 = Rectangle {
        width: 10,
        height: 20,
    };
    rect2.scale(2);
    println!("放大后面积: {}", rect2.area());
}

// --- 枚举 ---
// Rust的枚举远比C/Java的强大
// 可以携带任意类型的数据

#[derive(Debug)]
enum Direction {
    Up, // 无数据的变体
    Down,
    Left,
    Right,
}

fn enum_basics() {
    println!("=== 枚举基础 ===");

    let dir = Direction::Up;

    // 【match处理枚举】
    // 必须处理所有变体（穷尽性）
    match dir {
        Direction::Up => println!("向上"),
        Direction::Down => println!("向下"),
        Direction::Left => println!("向左"),
        Direction::Right => println!("向右"),
    }

    // 【if let简化】
    // 只关心一种情况时使用
    if let Direction::Up = dir {
        println!("正在向上!");
    }

    // 【枚举值】
    // 枚举的每个变体都是枚举类型的一个值
    // Direction::Up 的类型是 Direction
}

// --- 枚举携带数据 ---
// 这是Rust枚举最强大的特性

#[derive(Debug)]
enum Message {
    Quit,                    // 无数据
    Move { x: i32, y: i32 }, // 匿名结构体
    Write(String),           // 单个值
    ChangeColor(u8, u8, u8), // 多个值（元组）
}

impl Message {
    fn call(&self) {
        // 使用match处理不同的变体
        match self {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("移动到({}, {})", x, y),
            Message::Write(s) => println!("写入: {}", s),
            Message::ChangeColor(r, g, b) => println!("颜色: {} {} {}", r, g, b),
        }
    }
}

fn enum_with_data() {
    println!("=== 枚举携带数据 ===");

    // 创建不同类型的消息
    let m1 = Message::Move { x: 10, y: 20 };
    let m2 = Message::Write(String::from("hello"));
    let m3 = Message::ChangeColor(255, 0, 0);
    let m4 = Message::Quit;

    m1.call();
    m2.call();
    m3.call();
    m4.call();

    // 【枚举携带数据的意义】
    // 1. 状态和数据绑定在一起
    // 2. 类型安全 - 编译器确保处理所有情况
    // 3. 消除null - 用Option<T>替代null
}

// --- Option枚举（替代null）---
// Rust没有null，使用Option<T>表示可能缺失的值
// 这是Rust安全性的核心设计

fn option_demo() {
    println!("=== Option枚举 ===");

    // Option<T> 的定义（标准库已定义）
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // 使用Option
    let some_number: Option<i32> = Some(5);
    let absent_number: Option<i32> = None;

    // 【必须处理None的情况】
    // 不能直接使用Some中的值，必须先提取

    // 方法1：使用match
    match some_number {
        Some(n) => println!("有值: {}", n),
        None => println!("无值"),
    }

    // 方法2：使用if let
    if let Some(n) = absent_number {
        println!("有值: {}", n);
    } else {
        println!("无值（None）");
    }

    // 【Option的常用方法】
    let x = Some(5);

    // unwrap_or - 提供默认值
    let y = x.unwrap_or(0);
    println!("unwrap_or: {}", y);

    // map - 转换Some中的值
    let doubled = x.map(|n| n * 2);
    println!("map: {:?}", doubled);

    // and_then - 链式操作
    let result = Some(5).and_then(|n| Some(n * 2)).and_then(|n| Some(n + 1));
    println!("and_then: {:?}", result);

    // unwrap - 获取值，None时panic
    // 只用于确定有值的情况
    // let n = some_number.unwrap();

    // 【Option vs null】
    // - null: 可以在任何地方使用，容易忘记检查
    // - Option<T>: 必须显式处理None，类型系统强制
    // - 这消除了"null pointer exception"这类错误
}

// --- 练习：枚举和模式匹配 ---
// 定义形状枚举，计算面积
#[derive(Debug)]
enum Shape {
    Circle(f64),         // 半径
    Rectangle(f64, f64), // 宽、高
    Triangle(f64, f64),  // 底、高
}

fn area(shape: &Shape) -> f64 {
    // 模式匹配处理不同形状
    match shape {
        Shape::Circle(r) => {
            // r 是半径
            std::f64::consts::PI * r * r
        }
        Shape::Rectangle(w, h) => {
            // w 是宽，h 是高
            w * h
        }
        Shape::Triangle(b, h) => {
            // b 是底，h 是高
            b * h / 2.0
        }
    }
}

fn shape_demo() {
    println!("=== 形状计算示例 ===");

    let circle = Shape::Circle(2.0);
    let rect = Shape::Rectangle(3.0, 4.0);
    let triangle = Shape::Triangle(5.0, 3.0);

    println!("圆面积: {:.2}", area(&circle));
    println!("矩形面积: {:.2}", area(&rect));
    println!("三角形面积: {:.2}", area(&triangle));

    // 【设计优势】
    // 1. 所有形状统一在Shape类型中
    // 2. 编译器保证area函数处理所有形状
    // 3. 添加新形状时，编译器会提示修改area函数
}

// --- 枚举的设计模式 ---
// 【状态机】
// enum State {
//     Idle,
//     Running { since: Instant },
//     Paused { for: Duration },
// }

// 【结果类型】
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// 【链表】
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

pub fn run() {
    println!("\n========== 04_struct_enum ==========");
    struct_basics();
    method_syntax();
    enum_basics();
    enum_with_data();
    option_demo();
    shape_demo();
}
