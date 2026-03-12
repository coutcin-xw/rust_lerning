// ============================================================
// 01_basics.rs - 基础语法
// ============================================================
//
// 【本章目标】
// 了解Rust的基础语法，建立与其他语言的对比认知
//
// 【对比其他语言】
// - C++: 模板、RAII、内存管理
// - Java: 自动垃圾回收、面向对象
// - Go: 简洁语法、自动垃圾回收
// - Python: 动态类型、简洁语法
// - Rust: 静态类型、所有权系统、零成本抽象
//
// 【核心特点】
// 1. 默认不可变 - 更安全，更容易推理
// 2. 强类型系统 - 编译期捕获错误
// 3. 表达式语法 - 一切皆表达式
// 4. 模式匹配 - 强大的控制流
// ============================================================

// --- 变量和可变性 ---
fn variable_basics() {
    println!("=== 变量和可变性 ===");

    // 【不可变变量】
    // Rust默认所有变量都是不可变的（immutable）
    // 这是Rust安全哲学的核心：减少意外的修改
    let x = 5; // 不可变变量，类似Java的final
               // x = 6;  // 错误！不能修改不可变变量

    // 【为什么默认不可变？】
    // 1. 更容易理解代码 - 不用担心变量被意外修改
    // 2. 更容易并发 - 不可变数据天然线程安全
    // 3. 编译器优化 - 不可变数据可以更激进地优化

    // 【可变变量】
    // 使用 mut 关键字显式声明可变
    let mut y = 10; // mut = mutable，可变变量
    y = 20; // OK，可以修改
    println!("可变变量 y = {}", y);

    // 【常量 vs 不可变变量】
    // 常量（const）：
    // - 编译时就确定值，不能是函数返回值
    // - 必须标注类型
    // - 命名规范：SCREAMING_SNAKE_CASE
    // - 整个程序生命周期内有效
    const MAX_SIZE: u32 = 100;

    // 不可变变量（let）：
    // - 运行时可以确定值
    // - 类型可以推断
    // - 只在作用域内有效

    // 【变量遮蔽（Shadowing）】
    // Rust允许声明同名变量，新变量会遮蔽旧变量
    let x = x + 1; // 新的x，类型可以不同
    let x = "现在我是字符串"; // 类型也可以改变！
    println!("遮蔽后的x: {}", x);

    println!("x={}, y={}, MAX_SIZE={}", 5, y, MAX_SIZE);
}

// --- 基本数据类型 ---
fn data_types() {
    println!("=== 基本数据类型 ===");

    // 【标量类型（Scalar Types）】
    // 单个值的类型

    // 整数类型
    // - 有符号：i8, i16, i32, i64, i128, isize（指针大小）
    // - 无符号：u8, u16, u32, u64, u128, usize
    // - 默认是i32，是最快的类型
    let i: i32 = 42;
    let big: u128 = 100_000_000; // 下划线提高可读性

    // 浮点类型
    // - f32: 单精度（32位）
    // - f64: 双精度（64位），默认类型
    let f: f64 = 3.14;
    let f2 = 2.0_f32; // 类型后缀

    // 布尔类型
    let b: bool = true;

    // 字符类型
    // - char是Unicode标量值，4字节
    // - 可以表示任何Unicode字符，包括emoji
    let c: char = '中';
    let emoji = '😀';
    println!("字符: {} {}", c, emoji);

    // 【复合类型（Compound Types）】
    // 多个值组合成的类型

    // 元组（Tuple）
    // - 固定长度，可以有不同类型
    // - 类似Python的tuple或C++的std::tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 解构（Destructuring）
    let (a, b, c) = tup;
    println!("解构: a={}, b={}, c={}", a, b, c);

    // 通过索引访问
    println!("索引访问: tup.0 = {}, tup.1 = {}", tup.0, tup.1);

    // 数组（Array）
    // - 固定长度，所有元素类型相同
    // - 存储在栈上
    // - 类似C的数组，但更安全（边界检查）
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr[0] = {}", arr[0]);

    // 数组初始化的简写
    let same = [3; 5]; // [3, 3, 3, 3, 3]
    println!("相同元素数组: {:?}", same);

    // 【数组 vs Vector】
    // 数组：固定长度，栈分配
    // Vector：可变长度，堆分配
    // 选择：知道长度且不需要改变 -> 数组，否则 -> Vector
}

// --- 函数 ---
fn greet(name: &str) -> String {
    // 【函数定义规则】
    // 1. fn 关键字
    // 2. 参数必须声明类型
    // 3. 返回值类型用 -> 指定
    // 4. 最后一个表达式作为返回值（无分号）

    // 【表达式 vs 语句】
    // 语句（statement）：执行动作，无返回值，以分号结尾
    // 表达式（expression）：计算值，有返回值，无分号

    let greeting = "Hello"; // 语句
    format!("{}, {}!", greeting, name) // 表达式，作为返回值
}

// 函数返回多个值（元组）
fn swap(x: i32, y: i32) -> (i32, i32) {
    // 元组可以用来返回多个值
    (y, x) // 表达式返回
}

// 无返回值的函数
fn no_return() {
    // 无 -> 的函数返回单元类型 ()
    // 单元类型只有一个值：()
    println!("无返回值");
}

// --- 控制流 ---
fn control_flow() {
    println!("=== 控制流 ===");

    // 【if表达式】
    // 注意：Rust的if是表达式，可以有返回值
    let number = 13;

    // if条件必须是bool类型（不像C可以写if(x)）
    if number < 10 {
        println!("小于10");
    } else if number < 20 {
        println!("10-20之间");
    } else {
        println!("大于等于20");
    }

    // if作为表达式
    // 类似三元运算符，但更强大
    let condition = true;
    let result = if condition { 5 } else { 6 };
    // 注意：两个分支必须返回相同类型！
    println!("if表达式结果 = {}", result);

    // 【循环的三种形式】

    // 1. loop - 无限循环
    // 用于不确定循环次数的场景
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // break可以返回值
            break counter * 2;
        }
    };
    println!("loop结果 = {}", result);

    // loop + break 标签
    // 用于嵌套循环时指定跳出哪一层
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break; // 跳出内层循环
            }
            if count == 2 {
                break 'counting_up; // 跳出外层循环
            }
            remaining -= 1;
        }

        count += 1;
    }

    // 2. while - 条件循环
    // 类似其他语言的while
    let mut n = 5;
    while n > 0 {
        println!("countdown: {}", n);
        n -= 1;
    }

    // 3. for - 遍历集合（推荐方式）
    // 最常用的循环形式
    // 安全、高效、不会越界

    // 遍历范围
    for i in 0..5 {
        // 0..5 是范围语法，不包含5
        // 0..=5 包含5
        println!("for: {}", i);
    }

    // 遍历数组
    let arr = [10, 20, 30];
    for item in arr.iter() {
        // .iter() 创建迭代器
        println!("item: {}", item);
    }

    // 更简洁的写法
    for item in &arr {
        // 直接遍历引用
        println!("item ref: {}", item);
    }

    // 使用enumerate获取索引
    for (index, value) in arr.iter().enumerate() {
        println!("arr[{}] = {}", index, value);
    }

    // 【循环选择建议】
    // - 需要遍历集合：for（最安全）
    // - 需要根据条件循环：while
    // - 需要无限循环：loop
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
