// ============================================================
// 05_pattern_match.rs - 模式匹配
// match和if let的详细用法
// 对比: Java switch / C++ switch -> Rust match（更强大）
// ============================================================

fn match_basics() {
    let x = 1;

    match x {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        _ => println!("其他"), // _ 是默认分支，类似default
    }

    // match返回值
    let n = match x {
        1 => "one",
        2 => "two",
        _ => "other",
    };
    println!("n = {}", n);
}

// --- 模式匹配多个条件 ---
fn match_multiple() {
    let n = 5;

    match n {
        1 | 2 | 3 | 4 | 5 => println!("1到5之间"),
        6..=10 => println!("6到10之间"), // 范围匹配
        _ => println!("其他"),
    }

    // 匹配区间
    let c = 'c';
    match c {
        'a'..='z' => println!("小写字母"),
        'A'..='Z' => println!("大写字母"),
        '0'..='9' => println!("数字"),
        _ => println!("其他"),
    }
}

// --- 解构元组/结构体 ---
fn destructure() {
    // 解构元组
    let tup = (1, 2.0, 3);
    let (a, b, c) = tup;
    println!("a={}, b={}, c={}", a, b, c);

    // match中解构
    let tup = (1, 2, 3);
    match tup {
        (1, _, _) => println!("第一个是1"),
        (_, 2, _) => println!("第二个是2"),
        (_, _, 3) => println!("第三个是3"),
        _ => println!("其他"),
    }

    // 解构结构体
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 10, y: 20 };
    let Point { x, y } = p;
    println!("x={}, y={}", x, y);

    // match中解构+条件
    let p = Point { x: 10, y: 20 };
    match p {
        Point { x, y: 0 } => println!("在x轴上, x={}", x),
        Point { x: 0, y } => println!("在y轴上, y={}", y),
        Point { x, y } if x == y => println!("在y=x线上"),
        Point { x, y } => println!("其他点: ({}, {})", x, y),
    }
}

// --- 匹配守卫 ---
fn match_guard() {
    let n = Some(5);

    match n {
        Some(x) if x < 5 => println!("小于5: {}", x),
        Some(x) => println!("大于等于5: {}", x),
        None => println!("None"),
    }

    // 使用|时加守卫
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"), // (4|5|6) if y
        _ => println!("no"),
    }
}

// --- @绑定 ---
fn at_binding() {
    let n = 5;

    match n {
        // @ 可以给匹配的值绑定一个名字
        n @ 1..=5 => println!("n在1-5之间: {}", n),
        n => println!("n是其他值: {}", n),
    }

    // 在结构体中使用
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 3, y: 3 };
    match p {
        Point { x: n @ 0..=5, y: _ } if n > 0 => println!("x在0-5之间且为正: {}", n),
        _ => println!("其他"),
    }
}

// --- if let / while let ---
fn if_let_while_let() {
    // if let - 单分支匹配
    let option: Option<i32> = Some(5);

    if let Some(n) = option {
        println!("有值: {}", n);
    } else {
        println!("None");
    }

    // while let - 循环直到None
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("pop: {}", top);
    }
}

// --- for循环中的模式 ---
fn for_pattern() {
    let v = vec!["abc", "def", "ghi"];

    for (index, value) in v.iter().enumerate() {
        println!("[{}] = {}", index, value);
    }

    // 解构元组
    let pairs = [(1, 2), (3, 4), (5, 6)];
    for (a, b) in pairs.iter() {
        println!("{} + {} = {}", a, b, a + b);
    }
}

// --- let模式 ---
fn let_pattern() {
    // 解构元组
    let (a, b) = (1, 2);
    println!("a={}, b={}", a, b);

    // 解构数组
    let [x, y, z] = [1, 2, 3];
    println!("x={}, y={}, z={}", x, y, z);
}

// --- 练习：简单的解释器 ---
enum Expr {
    Value(i32),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Value(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Sub(a, b) => eval(a) - eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}

fn interpreter_demo() {
    // (2 + 3) * 4 = 20
    let expr = Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Value(2)),
            Box::new(Expr::Value(3)),
        )),
        Box::new(Expr::Value(4)),
    );

    println!("结果: {}", eval(&expr));
}

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
