// ============================================================
// 03_borrowing.rs - 借用和引用
// 借用 = 使用引用，不获取所有权
// 对比: C++引用 vs Rust借用（更安全）
// ============================================================

// --- 引用基础 ---
fn reference_basics() {
    let s1 = String::from("hello");

    // 不可变引用 &T
    let r1 = &s1;
    println!("引用: {}", r1);

    // 可变引用 &mut T
    let mut s2 = String::from("hello");
    //let r2 = &mut s2;
    let  r2=&mut s2;
    r2.push_str(" world");
    println!("可变引用: {}", r2);
}

// --- 借用规则 ---
fn borrowing_rules() {
    // 规则1: 任何时候要么有多个不可变引用，要么有一个可变引用
    let mut s = String::from("test");

    // 正确：先使用不可变引用
    let r1 = &s;
    let r2 = &s;
    println!("多个不可变引用: {} {}", r1, r2);
    // r1和r2在这里最后使用，之后可以创建可变引用

    let r3 = &mut s;
    println!("可变引用: {}", r3);

    // 规则2: 引用必须总是有效的
}

// --- 悬垂引用 ---
// Rust在编译期就防止了悬垂引用
// fn dangling_reference() -> &String {
//     let s = String::from("hello");
//     &s  // 错误! s在函数结束时被销毁
// }

// --- Slice类型 ---
fn slice_types() {
    let s = String::from("hello world");

    // 字符串切片 &str（类似Python s[0:5]）
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    // 简写
    let hello = &s[..5];
    let world = &s[6..];
    println!("{} {}", hello, world);

    // 数组切片
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    println!("slice: {:?}", slice);
}

// --- 函数参数使用引用 ---
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    &s
}

fn first_word_demo() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("第一个单词: {}", word);

    // 字符串字面量就是&str
    let s = "hello world";
    let word = first_word(s);
    println!("字面量的第一个单词: {}", word);
}

// --- 修改数据但借用规则复杂场景 ---
fn demo_scopes() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(" world");
        println!("r1: {}", r1);
    } // r1离开作用域

    let r2 = &s; // 现在可以创建不可变引用
    let r3 = &s;
    println!("r2: {}, r3: {}", r2, r3);
}

// --- 练习：修改字符串 ---
fn modify_string_ref() {
    let mut s = String::from("abc");

    // 使用可变引用修改
    let r = &mut s;
    r.make_ascii_uppercase();
    println!("修改后: {}", r);
}

pub fn run() {
    println!("\n========== 03_borrowing ==========");
    reference_basics();
    borrowing_rules();
    slice_types();
    first_word_demo();
    demo_scopes();
    modify_string_ref();
}
