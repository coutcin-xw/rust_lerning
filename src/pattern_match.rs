// ============================================================
// 05_pattern_match.rs - 模式匹配
// ============================================================
//
// 【核心概念】
// 模式匹配是Rust最强大的特性之一
// 它让你可以根据数据的结构进行条件判断和提取
//
// 【对比其他语言】
// - C/C++ switch: 只能匹配简单值，容易漏case
// - Java switch: Java 14+有模式匹配，但不如Rust强大
// - Python match: Python 3.10+引入，类似Rust
// - Scala: 强大的模式匹配，Rust的设计灵感来源
//
// 【核心优势】
// 1. 穷尽性检查 - 编译器确保处理所有情况
// 2. 解构能力 - 直接提取复杂数据结构中的值
// 3. 类型安全 - 模式匹配与类型系统紧密配合
// 4. 零成本抽象 - 编译期完全展开，无运行时开销
//
// 【match vs if let】
// match: 处理多个分支，保证穷尽性
// if let: 处理单个分支，更简洁
// ============================================================

// --- match基础 ---
fn match_basics() {
    println!("=== match基础 ===");

    // 【match语法】
    // match 值 {
    //     模式 => 表达式,
    //     模式 => 表达式,
    //     _ => 默认分支,  // _ 通配符
    // }

    let x = 1;

    match x {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        _ => println!("其他"), // _ 是默认分支，类似default
                               // 【重要】match必须穷尽所有可能
                               // 如果不用 _，编译器会警告未处理的情况
    }

    // 【match是表达式】
    // 可以返回值，每个分支返回的类型必须相同
    let n = match x {
        1 => "one",
        2 => "two",
        _ => "other",
    };
    println!("match返回值: n = {}", n);

    // 【对比C的switch】
    // C: 需要break，否则会fall-through
    // Rust: 自动break，不会fall-through
    // C: 可以漏掉case，Rust: 必须穷尽
}

// --- 模式匹配多个条件 ---
fn match_multiple() {
    println!("=== 匹配多个条件 ===");

    // 【| 操作符 - 或模式】
    // 匹配多个值
    let n = 5;

    match n {
        1 | 2 | 3 | 4 | 5 => println!("1到5之间"),
        // 等价于分开写：
        // 1 => ..., 2 => ..., 3 => ..., 4 => ..., 5 => ...
        6..=10 => println!("6到10之间"), // 范围匹配
        _ => println!("其他"),
    }

    // 【范围匹配 ..=】
    // 匹配一个闭区间范围
    let c = 'c';
    match c {
        'a'..='z' => println!("小写字母"),
        'A'..='Z' => println!("大写字母"),
        '0'..='9' => println!("数字"),
        _ => println!("其他"),
    }

    // 【范围匹配的要求】
    // 类型必须实现 std::cmp::PartialOrd trait
    // 编译器必须能验证范围是否为空
    // 支持的类型：整数、字符
}

// --- 解构元组/结构体 ---
fn destructure() {
    println!("=== 解构 ===");

    // 【元组解构】
    // 直接提取元组中的值
    let tup = (1, 2.0, 3);
    let (a, b, c) = tup;
    println!("元组解构: a={}, b={}, c={}", a, b, c);

    // 【match中解构元组】
    let tup = (1, 2, 3);
    match tup {
        (1, _, _) => println!("第一个是1"),
        // _ 忽略某个位置的值
        (_, 2, _) => println!("第二个是2"),
        (_, _, 3) => println!("第三个是3"),
        _ => println!("其他"),
    }

    // 【部分解构】
    // 可以只提取部分值
    let (first, _, third) = (1, 2, 3);
    println!("部分解构: first={}, third={}", first, third);

    // 【结构体解构】
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 10, y: 20 };

    // 解构结构体
    let Point { x, y } = p;
    println!("结构体解构: x={}, y={}", x, y);

    // 【match中解构结构体+条件】
    let p = Point { x: 10, y: 20 };
    match p {
        Point { x, y: 0 } => println!("在x轴上, x={}", x),
        // y必须等于0才能匹配
        Point { x: 0, y } => println!("在y轴上, y={}", y),
        // x必须等于0才能匹配
        Point { x, y } if x == y => println!("在y=x线上"),
        // 匹配守卫：额外的条件
        Point { x, y } => println!("其他点: ({}, {})", x, y),
        // 兜底分支
    }

    // 【解构的优势】
    // 1. 代码更简洁 - 不需要逐个访问字段
    // 2. 意图更清晰 - 一眼看出需要哪些字段
    // 3. 编译器检查 - 漏掉字段会报错
}

// --- 匹配守卫 ---
fn match_guard() {
    println!("=== 匹配守卫 ===");

    // 【匹配守卫】
    // 在模式后面加 if 条件
    // 可以添加额外的匹配条件

    let n = Some(5);

    match n {
        Some(x) if x < 5 => println!("小于5: {}", x),
        // 匹配Some，且x < 5
        Some(x) => println!("大于等于5: {}", x),
        None => println!("None"),
    }

    // 【守卫与|操作符】
    // 注意优先级
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"), // (4|5|6) if y
        // 先匹配 4|5|6，再检查 if y
        // 所以x=4且y=true才能匹配
        _ => println!("no"),
    }

    // 【何时使用守卫】
    // 当模式本身无法表达的条件
    // 例如：需要比较两个值
}

// --- @绑定 ---
fn at_binding() {
    println!("=== @绑定 ===");

    // 【@操作符】
    // 同时测试值和绑定变量
    // @ 让你可以在测试模式的同时绑定变量

    let n = 5;

    match n {
        // n @ 1..=5 意思是：
        // 1. 测试n是否在1..=5范围内
        // 2. 如果匹配，将值绑定到变量n
        n @ 1..=5 => println!("n在1-5之间: {}", n),
        n => println!("n是其他值: {}", n),
    }

    // 【对比】
    // 不使用@：
    // 1..=5 => println!("匹配但无法使用值"),
    // 使用@：
    // n @ 1..=5 => println!("匹配且可以使用值: {}", n),

    // 【在结构体中使用@】
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 3, y: 3 };
    match p {
        Point { x: n @ 0..=5, y: _ } if n > 0 => {
            println!("x在0-5之间且为正: {}", n)
        }
        // 同时测试范围和绑定变量
        _ => println!("其他"),
    }

    // 【@的使用场景】
    // - 需要同时测试范围和保留值
    // - 在复杂模式中提取部分值
}

// --- if let / while let ---
fn if_let_while_let() {
    println!("=== if let 和 while let ===");

    // 【if let】
    // 当只关心一种模式时使用
    // 是match的简化形式

    let option: Option<i32> = Some(5);

    // 使用match
    match option {
        Some(n) => println!("match: 有值: {}", n),
        _ => {} // 必须处理其他情况，很啰嗦
    }

    // 使用if let - 更简洁
    if let Some(n) = option {
        println!("if let: 有值: {}", n);
    } else {
        println!("None");
    }

    // 【何时使用if let】
    // - 只关心一种模式
    // - match太啰嗦
    // - 不需要穷尽性检查

    // 【while let】
    // 条件循环，当模式匹配时继续
    // 常用于处理迭代器或Option

    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        // pop返回Option<T>
        // 当Some时继续，None时停止
        println!("pop: {}", top);
    }

    // 【while let vs loop + match】
    // 等价的match写法：
    // loop {
    //     match stack.pop() {
    //         Some(top) => println!("{}", top),
    //         None => break,
    //     }
    // }
    // while let 更简洁
}

// --- for循环中的模式 ---
fn for_pattern() {
    println!("=== for循环中的模式 ===");

    // 【enumerate】
    // 同时获取索引和值
    let v = vec!["abc", "def", "ghi"];

    for (index, value) in v.iter().enumerate() {
        // (index, value) 是模式
        println!("[{}] = {}", index, value);
    }

    // 【解构元组】
    let pairs = [(1, 2), (3, 4), (5, 6)];
    for (a, b) in pairs.iter() {
        // 直接解构元组
        println!("{} + {} = {}", a, b, a + b);
    }

    // 【for循环模式的能力】
    // 可以在for中直接解构复杂数据
    // 让代码更简洁
}

// --- let模式 ---
fn let_pattern() {
    println!("=== let模式 ===");

    // 【let也是模式匹配】
    // 普通的let语句其实是模式匹配的特例

    // 解构元组
    let (a, b) = (1, 2);
    println!("元组: a={}, b={}", a, b);

    // 解构数组
    let [x, y, z] = [1, 2, 3];
    println!("数组: x={}, y={}, z={}", x, y, z);

    // 【函数参数也是模式】
    // fn foo((x, y): (i32, i32)) { ... }
    // 参数可以直接解构

    // 【let模式的应用】
    // 简化代码，避免临时变量
}

// --- 练习：简单的解释器 ---
// 【实战示例：用模式匹配实现AST求值】

// 定义表达式类型
enum Expr {
    Value(i32),                // 字面值
    Add(Box<Expr>, Box<Expr>), // 加法
    Sub(Box<Expr>, Box<Expr>), // 减法
    Mul(Box<Expr>, Box<Expr>), // 乘法
}

// 求值函数 - 使用模式匹配递归计算
fn eval(expr: &Expr) -> i32 {
    match expr {
        // 匹配字面值
        Expr::Value(n) => *n,

        // 匹配加法，递归求值
        Expr::Add(a, b) => eval(a) + eval(b),

        // 匹配减法
        Expr::Sub(a, b) => eval(a) - eval(b),

        // 匹配乘法
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
    // 模式匹配让AST求值代码非常清晰
    // 每个case对应一种操作
}

fn interpreter_demo() {
    println!("=== 解释器示例 ===");

    // 构建表达式树：(2 + 3) * 4 = 20
    let expr = Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Value(2)),
            Box::new(Expr::Value(3)),
        )),
        Box::new(Expr::Value(4)),
    );

    println!("(2 + 3) * 4 = {}", eval(&expr));

    // 【为什么用Box？】
    // Expr需要固定大小
    // Box<Expr> 是指针，大小固定
    // 这是Rust处理递归类型的常用方式
}

// --- 模式匹配的穷尽性 ---
// 【编译器保护】
// match必须覆盖所有可能
// 这是Rust安全性的重要保证

// fn incomplete_match(x: Option<i32>) {
//     match x {
//         Some(n) => println!("{}", n),
//         // 缺少None分支 - 编译错误！
//         // error: non-exhaustive patterns: `None` not covered
//     }
// }

pub fn run() {
    println!("\n========== 05_pattern_match ==========");
    match_basics();
    match_multiple();
    destructure();
    match_guard();
    at_binding();
    if_let_while_let();
    for_pattern();
    let_pattern();
    interpreter_demo();
}
