// ============================================================
// 08_lifetime.rs - 生命周期
// Rust的核心概念：确保引用始终有效
// 对比: C++无检查 / Rust编译期检查
// ============================================================

// --- 什么是生命周期 ---
fn lifetime_concept() {
    // 每个引用都有生命周期 - 引用有效的作用域
    let r;
    {
        let x = 5;
        r = &x; // x的生命周期在块内
    } // x在这里被销毁
      // println!("{}", r);  // 错误! r引用的x已销毁

    // 正确示例
    let x = 5;
    let r = &x;
    println!("r = {}", r); // x的生命周期覆盖r
}

// --- 生命周期标注 ---
// 语法: 'a, 'b, 'c 等
// 含义: 'a 表示某个生命周期

// 函数的生命周期标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetime_in_functions() {
    let string1 = String::from("hello");
    let string2 = String::from("world!");

    let result = longest(&string1, &string2);
    println!("最长的字符串: {}", result);

    // 正确示例2：返回参数中较短的生命周期
    let string1 = String::from("hello");
    let result;
    {
        let string2 = String::from("world!");
        // result的生命周期等于string1和string2中最短的那个
        // 这里string2生命周期更短，所以result不能超出这个块
        result = longest(&string1, &string2);
        println!("result: {}", result); // string2还在作用域内，所以OK
    }
    // 下面这行会报错！因为string2已经销毁了
    // println!("result: {}", result);
}

// --- Struct中的生命周期 ---
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("公告: {}", announcement);
        self.part
    }
}

fn struct_with_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("引用: {}", excerpt.part);
}

// --- 方法中的生命周期 ---
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // 多个生命周期
    fn announce_and_return2<'b>(&self, announcement: &'b str) -> &'a str {
        println!("{}", announcement);
        self.part
    }
}

// --- 生命周期省略规则 ---
// Rust会自动推断生命周期，以下情况不需要手动标注：
// 1. 每个引用参数有自己的生命周期
// 2. 如果只有一个输入生命周期，它赋给所有输出生命周期
// 3. 如果有&self或&mut self，它赋给所有输出生命周期

fn first_word(s: &str) -> &str {
    // 自动推断
    s.split(' ').next().unwrap()
}

// --- 静态生命周期 ---
// 'static 生命周期存活整个程序
fn static_lifetime() {
    let s: &'static str = "我可以存活整个程序";
    println!("{}", s);

    // 字符串字面量都是'static的
    // 错误信息中使用
}

// --- 泛型 + 生命周期 ---
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
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
}

struct SimpleParser;

impl<'a> Parser<'a> for SimpleParser {
    fn parse(&self, input: &'a str) -> &'a str {
        input.split_whitespace().next().unwrap_or("")
    }
}

fn lifetime_in_traits() {
    let parser = SimpleParser;
    let result = parser.parse("hello world");
    println!("解析结果: {}", result);
}

// --- 练习 ---
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
        self
    }

    fn build(&self) -> String {
        format!("{}: {:?}", self.name, self.value)
    }
}

fn builder_pattern() {
    let config = Config::new("my_config").value("some_value").build();

    println!("{}", config);
}

pub fn run() {
    println!("\n========== 08_lifetime ==========");
    lifetime_concept();
    lifetime_in_functions();
    struct_with_lifetime();
    static_lifetime();
    lifetime_in_traits();
    builder_pattern();
}
