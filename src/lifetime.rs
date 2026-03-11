// ============================================================
// 08_lifetime.rs - 生命周期
// ============================================================
//
// 【什么是生命周期？】
// 生命周期 = 引用有效的作用域范围
// 它是Rust确保引用始终有效的机制
//
// 【为什么需要生命周期？】
// - 防止悬垂引用（dangling reference）
// - 防止use-after-free
// - 在编译期保证内存安全
//
// 【核心思想】
// 编译器需要知道引用能活多久
// 才能确保引用不会指向已释放的内存
//
// 【对比其他语言】
// - C/C++: 没有检查，运行时可能崩溃
// - Java/Go: 有GC，不需要关心生命周期
// - Rust: 编译期检查，零运行时开销
//
// 【关键理解】
// 生命周期标注不会改变引用存活的时间
// 只是告诉编译器引用之间的关系
// 编译器用这些信息来验证引用是否有效
// ============================================================

// --- 什么是生命周期 ---
fn lifetime_concept() {
    println!("=== 生命周期概念 ===");

    // 【问题示例】
    // 下面这段代码演示了为什么需要生命周期检查
    let r; // ---------+-- 'a
           //          |
    {
        //          |
        let x = 5; // -+-- 'b  |
        r = &x; //  |       |
    } // -+       |
      //          |
      // println!("{}", r);     // 这里会报错！
      // 错误原因：x在块结束时被销毁，但r还试图引用它

    // 【生命周期图解】
    // 'a: r的生命周期
    // 'b: x的生命周期
    // 'a比'b长，但r引用了x，所以出错

    // 【正确示例】
    let x = 5; // ----------+-- 'b
    let r = &x; // --+-- 'a  |
    println!("r = {}", r); //   |       |
                           // --+       |
                           // x的生命周期覆盖r，所以没问题

    println!("生命周期的核心：引用不能比它引用的数据活得更长");
}

// --- 生命周期标注语法 ---
// 语法：'a, 'b, 'c 等
// 写在&后面：&'a str

// 【函数中的生命周期标注】
// longest函数返回两个字符串中较长的
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 【解读】
    // 'a 是一个生命周期参数
    // x和y的生命周期至少是'a
    // 返回值的生命周期也是'a

    // 这告诉编译器：返回值与参数x、y有相同的生命周期
    // 所以调用者知道返回值的有效范围

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetime_in_functions() {
    println!("=== 函数中的生命周期 ===");

    // 情况1：两个参数生命周期相同
    let string1 = String::from("hello");
    let string2 = String::from("world!");

    let result = longest(&string1, &string2);
    println!("最长的字符串: {}", result);
    // result的生命周期与string1、string2中较短的一致

    // 情况2：两个参数生命周期不同
    let string1 = String::from("hello"); // ----------+-- 'a
    let result; // --+       |
    {
        //   |       |
        let string2 = String::from("world!"); // --+-- 'b  |
        result = longest(&string1, &string2); //   |       |
        println!("result: {}", result); //   |       |
    } // --+       |
      // println!("result: {}", result);         // 错误！string2已销毁

    // 【为什么第二段代码有限制？】
    // longest返回的是x或y的引用
    // 编译器不知道运行时会返回哪个
    // 所以它按最坏情况假设：返回值可能是string2的引用
    // string2在内层块结束时销毁，所以result不能在外层使用
}

// --- Struct中的生命周期 ---
// 如果结构体包含引用，必须标注生命周期
struct ImportantExcerpt<'a> {
    // 【解读】
    // ImportantExcerpt不能比part引用的字符串活得更长
    // 'a表示part字段的生命周期
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("公告: {}", announcement);
        self.part
    }
}

fn struct_with_lifetime() {
    println!("=== 结构体中的生命周期 ===");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    // first_sentence是novel的切片

    let excerpt = ImportantExcerpt {
        part: first_sentence, // part引用novel的一部分
    };

    println!("引用: {}", excerpt.part);
    // excerpt不能比novel活得长，否则part会是悬垂引用

    // 【类比】
    // 类似C++的指针成员，需要确保指针指向的对象比结构体活得长
    // Rust把这个检查放到了编译期
}

// --- 方法中的生命周期 ---
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // 多个生命周期参数
    fn announce_and_return2<'b>(&self, announcement: &'b str) -> &'a str {
        println!("公告: {}", announcement);
        self.part // 返回self.part，生命周期是'a
    }

    // 【生命周期省略】
    // 在impl块中，self的生命周期会自动应用到返回值
}

// --- 生命周期省略规则 ---
// Rust编译器有三条规则来自动推断生命周期
// 在简单情况下不需要手动标注

fn elision_rules_demo() {
    println!("=== 生命周期省略规则 ===");

    // 【规则1】每个引用参数获得自己的生命周期
    // fn foo(x: &str, y: &str) -> ?
    // 等价于 fn foo<'a, 'b>(x: &'a str, y: &'b str) -> ?

    // 【规则2】如果只有一个输入生命周期，它赋给所有输出生命周期
    // fn foo(x: &str) -> &str
    // 等价于 fn foo<'a>(x: &'a str) -> &'a str

    // 【规则3】如果有&self或&mut self，self的生命周期赋给输出
    // fn foo(&self, x: &str) -> &str
    // self的生命周期自动应用到返回值

    // 【为什么有这些规则？】
    // 程序员写了太多重复的标注
    // 编译器发现几乎所有情况都符合这几个模式
    // 所以就自动推断了

    // 【什么时候需要手动标注？】
    // 当返回值的生命周期不确定时
    // 比如返回两个参数之一，编译器不知道选哪个
}

// 省略规则示例：这些都不需要手动标注
fn first_word(s: &str) -> &str {
    // 编译器自动推断：
    // 输入只有一个引用，输出也是引用
    // 所以输出的生命周期与输入相同
    s.split(' ').next().unwrap()
}

// --- 静态生命周期 ---
// 'static 表示整个程序运行期间都有效
fn static_lifetime() {
    println!("=== 静态生命周期 'static ===");

    // 字符串字面量就是'static的
    let s: &'static str = "我可以存活整个程序";
    println!("{}", s);

    // 【'static的两种含义】
    // 1. 生命周期：存活整个程序
    // 2. 拥有权：数据在二进制文件中，不会被释放

    // 【注意】
    // 不要为了解决生命周期错误就滥用'static
    // 这通常是设计问题的信号

    // 【何时使用】
    // - 常量字符串
    // - 全局数据
    // - 错误信息（有时候）
}

// --- 泛型 + 生命周期 + Trait Bound ---
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    // 可以同时有泛型参数和生命周期参数
    // 生命周期参数放在泛型参数的尖括号里
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// --- 生命周期在trait中 ---
trait Parser<'a> {
    fn parse(&self, input: &'a str) -> &'a str;
    // 返回值的生命周期与input相同
}

struct SimpleParser;

impl<'a> Parser<'a> for SimpleParser {
    fn parse(&self, input: &'a str) -> &'a str {
        input.split_whitespace().next().unwrap_or("")
    }
}

fn lifetime_in_traits() {
    println!("=== Trait中的生命周期 ===");

    let parser = SimpleParser;
    let result = parser.parse("hello world");
    println!("解析结果: {}", result);

    // result的生命周期与传入的字符串相同
}

// --- 实战示例：Builder模式 ---
struct Config<'a> {
    name: &'a str,
    value: Option<&'a str>,
}

impl<'a> Config<'a> {
    fn new(name: &'a str) -> Self {
        Config { name, value: None }
    }

    fn value(&mut self, value: &'a str) -> &mut Self {
        self.value = Some(value);
        self // 返回可变引用，支持链式调用
    }

    fn build(&self) -> String {
        format!("{}: {:?}", self.name, self.value)
    }
}

fn builder_pattern() {
    println!("=== Builder模式示例 ===");

    let config = Config::new("my_config").value("some_value").build();

    println!("{}", config);
}

// --- 总结 ---
fn lifetime_summary() {
    println!("\n=== 生命周期总结 ===");

    println!("1. 生命周期确保引用始终有效");
    println!("2. 生命周期标注不改变存活时间，只是描述关系");
    println!("3. 大多数情况编译器可以自动推断");
    println!("4. 当返回引用时，通常需要手动标注");
    println!("5. 'static表示整个程序期间有效");

    println!("\n关键口诀：");
    println!("  - 引用不能比数据活得长");
    println!("  - 返回引用要标注生命周期");
    println!("  - 结构体含引用要标注");
}

pub fn run() {
    println!("\n========== 08_lifetime ==========");
    lifetime_concept();
    lifetime_in_functions();
    struct_with_lifetime();
    elision_rules_demo();
    static_lifetime();
    lifetime_in_traits();
    builder_pattern();
    lifetime_summary();
}
