// ============================================================
// 03_borrowing.rs - 借用和引用
// ============================================================
//
// 【什么是借用？】
// 借用 = 创建一个引用
// 引用让你使用值但不获取所有权
//
// 【为什么要借用？】
// 1. 避免所有权频繁转移的麻烦
// 2. 让多个地方可以访问同一数据
// 3. 保证内存安全（编译期检查）
//
// 【两种引用】
// - 不可变引用 &T：只能读，不能改
// - 可变引用 &mut T：可以读写
//
// 【对比其他语言】
// - C++: 引用类似，但没有编译期安全检查
// - Java: 没有引用的概念，所有对象通过指针访问
// - Go: 有指针，但没有借用规则检查
// ============================================================

// --- 引用基础 ---
fn reference_basics() {
    println!("=== 引用基础 ===");

    // 【不可变引用 &T】
    // 类似C++的 const T& 或 Java的final引用
    // 只能读取，不能修改
    let s1 = String::from("hello");

    let r1 = &s1; // r1是对s1的不可变引用
    let r2 = &s1; // 可以有多个不可变引用
    println!("多个引用: {} {}", r1, r2);

    // 通过引用可以读，但不能写
    // r1.push_str(" world");  // 错误！不可变引用不能修改

    println!("原值仍然有效: {}", s1);

    // 【可变引用 &mut T】
    // 类似C++的 T&（非const引用）
    // 可以通过引用修改值
    let mut s2 = String::from("hello");
    // 注意：s2本身必须是mut的，才能创建&mut引用

    let r2 = &mut s2; // 创建可变引用
    r2.push_str(" world"); // 通过引用修改
    println!("修改后: {}", r2);
}

// --- 借用规则详解 ---
fn borrowing_rules() {
    println!("=== 借用规则详解 ===");

    // 【核心规则】
    // 1. 可以有多个不可变引用
    // 2. 或者一个可变引用
    // 3. 但不能同时有可变和不可变引用

    let mut s = String::from("test");

    // ✅ 多个不可变引用
    let r1 = &s;
    let r2 = &s;
    println!("多个不可变引用: {} {}", r1, r2);
    // r1和r2在这里最后使用，之后可以创建可变引用

    // ✅ 一个可变引用
    let r3 = &mut s;
    println!("可变引用: {}", r3);

    // ❌ 错误示例（取消注释会编译错误）：
    // let r1 = &s;
    // let r2 = &mut s;  // 错误！有不可变引用时不能有可变引用
    // println!("{} {}", r1, r2);

    // 【为什么这样设计？】
    // 防止数据竞争（data race）
    // 数据竞争：两个指针同时访问同一数据，至少一个写，没有同步机制
    // 这是很多并发bug的根源，Rust在编译期就禁止了！

    // 【引用的作用域】
    // 引用的作用域 = 从声明到最后一次使用的位置
    // 不是到变量所在的大括号结束！这叫 NLL（Non-Lexical Lifetimes）
}

// --- 悬垂引用 ---
fn dangling_reference_demo() {
    println!("=== 悬垂引用（Rust防止） ===");

    // 【什么是悬垂引用？】
    // 引用指向的内存已经被释放

    // ❌ 在C++中这样的代码会编译通过，运行时崩溃：
    // char* dangling() {
    //     char s[] = "hello";
    //     return &s[0];  // 返回局部变量的指针
    // }  // s在这里被销毁，返回的指针无效

    // ✅ Rust在编译期就防止了这个问题：
    // fn dangling() -> &String {
    //     let s = String::from("hello");
    //     &s  // 错误！返回了局部变量的引用
    // }  // s在这里被销毁

    // 编译器会报错：returns a reference to data owned by the current function

    println!("Rust编译器会阻止你创建悬垂引用！");
}

// --- 切片类型 ---
fn slice_types() {
    println!("=== 切片类型 ===");

    // 【字符串切片 &str】
    // 引用String中的一部分
    // 类似Python的 s[0:5] 或 Go的 s[0:5]

    let s = String::from("hello world");

    // 创建切片：[start..end] 不包含end
    let hello = &s[0..5]; // 索引0到4（不包含5）
    let world = &s[6..11]; // 索引6到10
    println!("切片: {} {}", hello, world);

    // 简写形式
    let hello = &s[..5]; // 从开头到索引4
    let world = &s[6..]; // 从索引6到结尾
    let full = &s[..]; // 整个字符串
    println!("{} {} {}", hello, world, full);

    // 【切片是引用】
    // 切片是对原数据的引用，没有所有权
    // 切片必须有效（不能越界）

    // 【数组切片】
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4]; // [2, 3, 4]
    println!("数组切片: {:?}", slice);

    // 【&str vs String】
    // &str: 切片类型，可以是字面量或String的切片
    // String: 堆分配的可增长字符串
    let literal: &str = "hello"; // 字面量，编译期确定
    let string: String = String::from("hello"); // 堆分配
    let slice: &str = &string; // String可以转成&str
}

// --- 函数参数使用切片 ---
fn first_word(s: &str) -> &str {
    // 【技巧】参数用&str而不是&String
    // 原因：&str更通用
    // - 可以接受 &String（自动转换）
    // - 可以接受 &str（字符串字面量）
    // - 可以接受切片

    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i]; // 返回切片
        }
    }

    &s // 没有空格，返回整个字符串
}

fn first_word_demo() {
    println!("=== 切片作为函数参数 ===");

    // 传String的切片
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("第一个单词: {}", word);

    // 直接传字符串字面量（&str类型）
    let literal = "hello world";
    let word = first_word(literal);
    println!("字面量的第一个单词: {}", word);
}

// --- 引用作用域示例 ---
fn demo_scopes() {
    println!("=== 引用作用域示例 ===");

    let mut s = String::from("hello");

    // 可变引用在独立作用域中
    {
        let r1 = &mut s;
        r1.push_str(" world");
        println!("r1: {}", r1);
    } // r1离开作用域，可以被其他引用使用

    // 现在可以创建不可变引用
    let r2 = &s;
    let r3 = &s;
    println!("r2: {}, r3: {}", r2, r3);

    // 【技巧】使用大括号限制可变引用的作用域
    // 这样可以避免"同时有多个可变引用"的错误
}

// --- 修改字符串 ---
fn modify_string_ref() {
    println!("=== 通过引用修改 ===");

    let mut s = String::from("abc");

    // 使用可变引用修改
    let r = &mut s;
    r.make_ascii_uppercase(); // 变成大写
    println!("修改后: {}", r);

    // 【注意】这里r还在作用域内，不能再创建其他引用
}

// --- 解引用 ---
fn dereference_demo() {
    println!("=== 解引用 ===");

    let x = 5;
    let r = &x;

    // * 是解引用操作符
    println!("r指向的值: {}", *r); // 解引用获取值

    // 【自动解引用】
    // Rust会在必要时自动解引用
    let s = String::from("hello");
    let r = &s;
    // 下面两种写法等价：
    println!("长度: {}", (*r).len()); // 手动解引用
    println!("长度: {}", r.len()); // 自动解引用

    // 【解引用强制多态】
    // Rust会自动进行多层解引用
}

pub fn run() {
    println!("\n========== 03_borrowing ==========");
    reference_basics();
    borrowing_rules();
    dangling_reference_demo();
    slice_types();
    first_word_demo();
    demo_scopes();
    modify_string_ref();
    dereference_demo();
}
