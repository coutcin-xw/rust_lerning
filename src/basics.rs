// ============================================================
// 01_basics.rs - 基础语法
// 对比: C++/Java -> Rust
// ============================================================

// --- 变量和可变性 ---
fn variable_basics() {
    // Rust默认是不可变的，这是安全性的基础
    let x = 5; // 不可变变量 (类似 Java final)
               // x = 6; // 错误! 不能修改不可变变量

    let mut y = 10; // mut = mutable，可变变量
    y = 20; // OK

    // 常量 (编译时常量)
    const MAX_SIZE: u32 = 100; // 必须标注类型

    println!("x={}, y={}, MAX_SIZE={}", x, y, MAX_SIZE);
}

// --- 基本数据类型 ---
fn data_types() {
    // 标量类型
    let i: i32 = 42; // 有符号整数
    let f: f64 = 3.14; // 浮点数
    let b: bool = true; // 布尔值
    let c: char = '中'; // Unicode字符

    // 元组 - 类似Python/C++ tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup; // 解构
    println!("tup.0 = {}", tup.0);

    // 数组 - 类似C/Java数组，但长度固定
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr[0] = {}", arr[0]);
}

// --- 函数 ---
fn greet(name: &str) -> String {
    // 最后一个表达式作为返回值（无分号）
    format!("Hello, {}!", name)
}

// 函数返回多个值（元组）
fn swap(x: i32, y: i32) -> (i32, i32) {
    (y, x)
}

// --- 控制流 ---
fn control_flow() {
    let number = 13;

    // if表达式（必须有bool条件，不像C可以写if(x)）
    if number < 10 {
        println!("小于10");
    } else if number < 20 {
        println!("10-20之间");
    } else {
        println!("大于等于20");
    }

    // let语句中使用if（类似三元运算符但更强大）
    let condition = true;
    let result = if condition { 5 } else { 6 };
    println!("result = {}", result);

    // 循环 - 有3种方式
    // 1. loop - 无限循环
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break可以返回值
        }
    };
    println!("loop result = {}", result);

    // 2. while - 类似其他语言
    let mut n = 5;
    while n > 0 {
        println!("countdown: {}", n);
        n -= 1;
    }

    // 3. for - 遍历范围或迭代器（推荐方式）
    for i in 0..5 {
        // 0..5 不包含5
        println!("for: {}", i);
    }

    // 遍历数组
    let arr = [10, 20, 30];
    for item in arr.iter() {
        println!("item: {}", item);
    }
}

pub fn run() {
    println!("\n========== 01_basics ==========");
    variable_basics();
    data_types();
    println!("{}", greet("Rust"));
    let (a, b) = swap(1, 2);
    println!("swap(1,2) = ({}, {})", a, b);
    control_flow();
}
