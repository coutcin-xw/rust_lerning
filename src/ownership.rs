// ============================================================
// 02_ownership.rs - 所有权系统
// 核心概念：每个值有且只有一个所有者，当所有者离开作用域时值被释放
// 对比: C(手动管理) / Java(GC) -> Rust(所有权+RAII)
// ============================================================

// --- 所有权规则 ---
fn ownership_rules() {
    // 1. Rust中每个值都有一个所有者
    let s1 = String::from("hello"); // s1是所有者
    let s2 = s1; // 所有权转移(move)给s2
                 // println!("{}", s1);  // 错误! s1已失效

    // 2. 基本类型是Copy的（栈上数据）
    let x = 5;
    let y = x; // 复制，x仍然有效
    println!("x={}, y={}", x, y);

    // 3. 函数中的所有权
    let s = String::from("hello");
    takes_ownership(s); // s的值移动到函数内，s在此失效
                        // println!("{}", s);  // 错误!

    let n = 5;
    makes_copy(n); // n是Copy的，复制后在函数外仍然有效
    println!("n仍然有效: {}", n);
}

fn takes_ownership(s: String) {
    println!("获得所有权: {}", s);
} // s离开作用域，内存被释放

fn makes_copy(n: i32) {
    println!("复制: {}", n);
} // n离开作用域，但i32是Copy的，不影响原变量

// --- 返回所有权 ---
fn give_ownership() -> String {
    let s = String::from("hello");
    s // 返回所有权给调用者
}

// 接收并返回所有权
fn take_and_give(s: String) -> String {
    s
}

// --- 引用和借用（在03_borrowing中详细讲解）---
fn use_reference() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 借用s1，不获取所有权
    println!("'{}'的长度是 {}", s1, len); // s1仍然有效
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s离开作用域，但不释放String（因为只是引用）

// --- 可变引用 ---
fn mutable_reference() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("修改后: {}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// --- 借用规则验证 ---
// 同一时刻只能有一个可变引用，或多个不可变引用
fn borrowing_rules_demo() {
    let mut s = String::from("hello");

    // 可变引用
    let r1 = &mut s;
    // let r2 = &mut s;  // 错误! 不能同时有两个可变引用
    r1.push_str(" world");
    println!("r1: {}", r1);

    // 不可变引用
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s;  // 错误! 有不可变引用时不能有可变引用
    println!("r1: {}, r2: {}", r1, r2);
}

// --- Drop trait ---
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("释放内存: {}", self.data);
    }
}

fn drop_trait_demo() {
    let c = CustomSmartPointer {
        data: String::from("my data"),
    };
    println!("创建 CustomSmartPointer");
    // c.drop();  // 显式调用drop（不推荐）
    println!("即将离开作用域...");
    // 自动调用drop
}

pub fn run() {
    println!("\n========== 02_ownership ==========");
    ownership_rules();
    let s = give_ownership();
    println!("获得所有权: {}", s);
    use_reference();
    mutable_reference();
    borrowing_rules_demo();
    drop_trait_demo();
}
