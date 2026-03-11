// ============================================================
// 02_ownership.rs - 所有权系统
// ============================================================
//
// 【核心概念】
// Rust没有GC，也没有手动内存管理，而是通过所有权系统在编译期保证内存安全
//
// 【三大规则】
// 1. 每个值都有一个所有者（owner）
// 2. 同一时刻只能有一个所有者
// 3. 所有者离开作用域时，值被自动释放（调用drop）
//
// 【对比其他语言】
// - C/C++: 手动malloc/free，容易内存泄漏、悬垂指针
// - Java/Python/Go: 有GC，运行时自动回收，但不可控
// - Rust: 编译期确定释放时机，零运行时开销
// ============================================================

// --- 所有权规则 ---
fn ownership_rules() {
    println!("=== 所有权基本规则 ===");

    // 【规则1：所有权转移(move)】
    // String是在堆上分配的，赋值时会转移所有权
    let s1 = String::from("hello"); // s1拥有这个字符串
    let s2 = s1; // 所有权从s1转移到s2
                 // s1现在无效了！下面这行会编译错误：
                 // println!("{}", s1);  // 错误: value borrowed after move

    // 【为什么这样设计？】
    // 如果s1和s2都指向同一块堆内存，当它们离开作用域时
    // 会发生double free（重复释放），导致内存错误
    // Rust通过move语义避免了这个问题

    // 【规则2：Copy类型】
    // 基本类型（整数、浮点数、布尔、字符等）存储在栈上
    // 实现了Copy trait，赋值时会复制，而不是移动
    let x = 5;
    let y = x; // x被复制给y，x仍然有效
    println!("x={}, y={}", x, y); // 两个都能用

    // 【Copy类型包括】
    // - 所有整数类型：i8, i16, i32, i64, i128, isize, u*
    // - 浮点类型：f32, f64
    // - 布尔类型：bool
    // - 字符类型：char
    // - 元素都是Copy的元组：(i32, i32)是Copy，(i32, String)不是

    // 【规则3：函数中的所有权】
    let s = String::from("hello");
    takes_ownership(s); // s的所有权移动到函数内
                        // println!("{}", s);  // 错误！s已经无效

    let n = 5;
    makes_copy(n); // n是Copy的，复制给函数
    println!("n仍然有效: {}", n); // n还能用
}

fn takes_ownership(s: String) {
    println!("函数获得所有权: {}", s);
} // s在这里离开作用域，drop被调用，内存被释放

fn makes_copy(n: i32) {
    println!("函数获得副本: {}", n);
} // n离开作用域，但只是栈上的副本，原变量不受影响

// --- 返回所有权 ---
fn give_ownership() -> String {
    println!("=== 返回所有权 ===");

    let s = String::from("hello");
    s // 返回s，所有权转移给调用者
      // 注意：这里不会发生释放，因为所有权被"送出去"了
}

// 接收并返回所有权
fn take_and_give(s: String) -> String {
    s // 所有权进来，又出去
}

// --- 引用和借用 ---
fn use_reference() {
    println!("=== 引用与借用 ===");

    // 【问题】每次传递都失去所有权太麻烦了
    // 【解决】使用引用（&），不获取所有权，只是"借用"

    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1 创建一个引用
                                     // s1的所有权还在，我们只是把它"借给"函数用

    println!("'{}'的长度是 {}", s1, len); // s1仍然有效！

    // 【对比C++】
    // C++的引用类似，但Rust的引用有编译期安全检查
}

fn calculate_length(s: &String) -> usize {
    // s是一个引用，不拥有这个String
    // 函数结束后，String不会被释放
    s.len()
} // s离开作用域，但因为没有所有权，不会释放String

// --- 可变引用 ---
fn mutable_reference() {
    println!("=== 可变引用 ===");

    // 默认引用是不可变的，不能通过引用修改值
    // 要修改，需要可变引用 &mut

    let mut s = String::from("hello"); // mut让s本身可变
    change(&mut s); // 传递可变引用
    println!("修改后: {}", s);

    // 【可变引用的限制】
    // 在同一作用域内，对同一数据只能有一个可变引用
    // 这是Rust防止数据竞争的核心机制
}

fn change(s: &mut String) {
    s.push_str(", world"); // 通过可变引用修改
}

// --- 借用规则验证 ---
fn borrowing_rules_demo() {
    println!("=== 借用规则 ===");

    // 【核心规则】
    // 可以有多个不可变引用，或者一个可变引用
    // 但不能同时有可变和不可变引用

    let mut s = String::from("hello");

    // ✅ 一个可变引用
    let r1 = &mut s;
    r1.push_str(" world");
    println!("r1: {}", r1);
    // r1在这里最后一次使用，之后可以创建新引用

    // ✅ 多个不可变引用
    let r1 = &s; // 新的r1，覆盖了之前的
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    // r1和r2都是不可变的，没问题

    // ❌ 下面这行会编译错误：
    // let r3 = &mut s;  // 错误！有不可变引用时不能有可变引用
    // println!("{}, {}, {}", r1, r2, r3);

    // 【为什么这样？】
    // 防止数据竞争（data race），这是并发编程中常见的bug
    // 编译器在编译期就能发现这类问题！

    // 【作用域技巧】
    // 引用的作用域从声明开始，到最后一次使用结束
    {
        let r1 = &mut s;
        r1.push_str("!");
    } // r1在这里结束

    let r2 = &s; // 现在可以了，因为r1已经结束
    println!("r2: {}", r2);
}

// --- Drop trait ---
// Rust通过Drop trait实现自动资源释放
// 类似C++的析构函数

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // 在这里可以写清理逻辑
        println!("释放内存: {}", self.data);
    }
}

fn drop_trait_demo() {
    println!("=== Drop自动释放 ===");

    let c = CustomSmartPointer {
        data: String::from("my data"),
    };
    println!("创建 CustomSmartPointer");

    // 不需要手动释放！
    // 当c离开作用域时，Rust自动调用drop方法

    // 【手动释放】
    // 如果需要提前释放，使用std::mem::drop
    // drop(c);  // 显式调用
    // 但不能用c.drop()！这是Rust的规则

    println!("即将离开作用域...");
} // c在这里被drop

// --- 克隆 ---
fn clone_demo() {
    println!("=== Clone深拷贝 ===");

    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深拷贝，堆上的数据也被复制

    println!("s1 = {}, s2 = {}", s1, s2); // 两个都有效

    // clone会复制堆上的数据，所以开销较大
    // 一般只在确实需要独立副本时使用
}

pub fn run() {
    println!("\n========== 02_ownership ==========");
    ownership_rules();
    let s = give_ownership();
    println!("调用者获得所有权: {}", s);
    use_reference();
    mutable_reference();
    borrowing_rules_demo();
    drop_trait_demo();
    clone_demo();
}
