// ============================================================
// 15_macros.rs - 宏
// ============================================================
//
// 【核心概念】
// 宏 = 元编程，在编译期生成代码
// 可以扩展语言语法，减少重复代码
//
// 【对比其他语言】
// - C/C++: 预处理器宏（文本替换，不安全）
// - Lisp: 宏系统（代码即数据）
// - JavaScript: 无内置宏（需Babel等工具）
// - Rust: 卫生的宏系统（类型安全，作用域安全）
//
// 【Rust宏的特点】
// 1. 卫生性（Hygiene）- 不会意外捕获变量
// 2. 类型安全 - 宏展开后进行类型检查
// 3. 模式匹配 - 使用类似match的语法
// 4. 两种类型 - 声明式宏和过程宏
//
// 【宏 vs 函数】
// - 宏：编译期展开，可以生成代码，语法灵活
// - 函数：运行时调用，类型固定，更易理解
//
// 【何时使用宏】
// - 需要生成重复代码
// - 需要可变参数
// - 需要扩展语法
// - 编译期计算
// ============================================================

// --- 声明式宏基础 ---
// 使用 macro_rules! 定义

// 【简单宏】
macro_rules! say_hello {
    // () 表示无参数
    () => {
        println!("Hello!");
    };
}

// 【带参数的宏】
macro_rules! create_function {
    // $name:ident 表示参数是一个标识符
    ($name:ident) => {
        fn $name() {
            println!("函数 {} 被调用", stringify!($name));
        }
    };
}

// 使用宏创建函数
create_function!(foo);
create_function!(bar);

fn declarative_macros() {
    println!("=== 声明式宏基础 ===");

    // 调用简单宏
    say_hello!();

    // 调用宏创建的函数
    foo();
    bar();

    println!("\n宏使用 macro_rules! 定义");
}

// --- 宏的参数类型 ---
macro_rules! param_types {
    // ident: 标识符（变量名、函数名等）
    ($name:ident) => {
        println!("标识符: {}", stringify!($name));
    };

    // expr: 表达式
    ($e:expr) => {
        println!("表达式结果: {}", $e);
    };

    // ty: 类型
    ($t:ty) => {
        println!("类型: {}", stringify!($t));
    };

    // literal: 字面量
    ($l:literal) => {
        println!("字面量: {}", $l);
    };

    // tt: 标记树（token tree），任意标记
    ($($tt:tt)*) => {
        println!("标记树: {}", stringify!($($tt)*));
    };

    // stmt: 语句
    ($s:stmt) => {
        println!("语句: {}", stringify!($s));
    };

    // block: 代码块
    ($b:block) => {
        println!("代码块");
        $b
    };

    // item: 项（函数、结构体等）
    ($i:item) => {
        println!("项: {}", stringify!($i));
    };

    // vis: 可见性修饰符
    ($v:vis $name:ident) => {
        println!("可见性: {}", stringify!($v));
    };
}

fn macro_param_types() {
    println!("=== 宏参数类型 ===");

    param_types!(my_var);
    param_types!(1 + 2);
    param_types!(i32);
    param_types!(42);
    // 路径作为标记树传递
    param_types!(std::collections::HashMap<String, i32>);
    // 语句示例
    println!("语句示例: let x = 5;");

    println!("\n常用参数类型：");
    println!("  ident - 标识符");
    println!("  expr - 表达式");
    println!("  ty - 类型");
    println!("  literal - 字面量");
    println!("  tt - 标记树");
    println!("  stmt - 语句");
    println!("  block - 代码块");
    println!("  path - 路径");
    println!("  item - 项");
}

// --- 重复模式 ---
macro_rules! vector {
    // $(...),* 表示重复0次或多次，用逗号分隔
    // $(...),+ 表示重复1次或多次
    // $(...)* 表示重复0次或多次，无分隔符

    // 创建 Vec
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };

    // 带尾随逗号
    ($($x:expr),+ ,) => {
        vector![$($x),+]
    };
}

macro_rules! hash_map {
    // 创建 HashMap
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

fn repetition_patterns() {
    println!("=== 重复模式 ===");

    let v = vector![1, 2, 3, 4, 5];
    println!("创建的Vec: {:?}", v);

    let map = hash_map![
        "a" => 1,
        "b" => 2,
        "c" => 3
    ];
    println!("创建的HashMap: {:?}", map);

    println!("\n重复模式语法：");
    println!("  $(...),* - 0次或多次，逗号分隔");
    println!("  $(...),+ - 1次或多次，逗号分隔");
    println!("  $(...)*  - 0次或多次，无分隔");
}

// --- 高级宏模式 ---
macro_rules! calculate {
    // 递归模式匹配
    (eval $e:expr) => {{
        println!("计算: {} = {}", stringify!($e), $e);
        $e
    }};

    // 多分支
    (add $a:expr, $b:expr) => {
        $a + $b
    };

    (sub $a:expr, $b:expr) => {
        $a - $b
    };

    (mul $a:expr, $b:expr) => {
        $a * $b
    };
}

// 结构化宏
macro_rules! struct_builder {
    (
        struct $name:ident {
            $($field:ident: $type:ty),*
        }
    ) => {
        struct $name {
            $($field: $type),*
        }

        impl $name {
            fn new($($field: $type),*) -> Self {
                Self { $($field),* }
            }
        }
    };
}

// 使用宏定义结构体
struct_builder! {
    struct Person {
        name: String,
        age: u32
    }
}

fn advanced_macros() {
    println!("=== 高级宏模式 ===");

    let result = calculate!(eval 1 + 2 * 3);
    println!("结果: {}", result);

    let sum = calculate!(add 5, 3);
    let diff = calculate!(sub 5, 3);
    let product = calculate!(mul 5, 3);
    println!("sum: {}, diff: {}, product: {}", sum, diff, product);

    // 使用宏生成的结构体
    let person = Person::new(String::from("Alice"), 30);
    println!("Person: {} ({})", person.name, person.age);

    println!("\n宏可以生成复杂的代码结构");
}

// --- 宏的卫生性 ---
macro_rules! hygiene_demo {
    ($x:expr) => {{
        // 宏内的变量不会与外部冲突
        let x = 10;
        println!("宏内的 x: {}", x);
        println!("参数值: {}", $x);
    }};
}

fn macro_hygiene() {
    println!("=== 宏的卫生性 ===");

    let x = 5;
    hygiene_demo!(x);

    println!("宏外的 x: {}", x);

    println!("\n卫生性保证宏不会意外捕获外部变量");
}

// --- 内置宏 ---
fn built_in_macros() {
    println!("=== 内置宏 ===");

    // 【println! / print!】
    println!("Hello, {}!", "World");

    // 【format!】
    let s = format!("值: {}", 42);
    println!("{}", s);

    // 【vec!】
    let v = vec![1, 2, 3];
    println!("Vec: {:?}", v);

    // 【assert! / assert_eq! / assert_ne!】
    assert!(true);
    assert_eq!(1 + 1, 2);
    assert_ne!(1, 2);

    // 【debug_assert!】只在 debug 模式生效
    debug_assert!(true);

    // 【panic!】
    // panic!("程序终止");

    // 【todo! / unimplemented!】
    // fn not_done() { todo!("还没实现"); }

    // 【unreachable!】
    // 标记不应该执行的代码

    // 【stringify!】转换为字符串
    println!("字符串化: {}", stringify!(1 + 2));

    // 【concat!】连接字符串
    println!("连接: {}", concat!("Hello", " ", "World"));

    // 【env!】获取环境变量
    // let path = env!("PATH");

    // 【option_env!】可选环境变量
    // if let Some(path) = option_env!("CUSTOM_VAR") { ... }

    // 【include_str!】包含文件内容
    // let content = include_str!("file.txt");

    // 【include_bytes!】包含文件字节
    // let bytes = include_bytes!("file.bin");

    // 【cfg!】编译配置
    if cfg!(debug_assertions) {
        println!("Debug 模式");
    }

    // 【file! / line! / column!】
    println!("文件: {}, 行: {}, 列: {}", file!(), line!(), column!());

    // 【module_path!】
    println!("模块路径: {}", module_path!());

    println!("\n内置宏提供了丰富的编译期功能");
}

// --- 过程宏简介 ---
// 过程宏需要单独的 crate

fn procedural_macros_intro() {
    println!("=== 过程宏简介 ===");

    println!("过程宏需要在单独的 crate (proc-macro) 中定义");

    println!("\n【三种过程宏】");

    // 【1. 派生宏】
    println!("1. 派生宏 #[derive(...)]");
    println!("   自动实现 trait");
    println!("   示例: #[derive(Debug, Clone, Serialize)]");

    // 【2. 属性宏】
    println!("\n2. 属性宏 #[attribute]");
    println!("   添加到项上，转换代码");
    println!("   示例: #[route(GET, \"/index\")]");
    println!("   示例: #[test]");

    // 【3. 函数式宏】
    println!("\n3. 函数式宏");
    println!("   像函数调用");
    println!("   示例: sql!(SELECT * FROM users)");
    println!("   示例: serde_json::json!({{\"key\": \"value\"}})");

    println!("\n【过程宏的流程】");
    println!("  1. 创建 proc-macro 类型的 crate");
    println!("  2. 实现 TokenStream -> TokenStream");
    println!("  3. 使用 syn 解析，quote 生成");
}

// --- 派生宏示例（伪代码）---
fn derive_macro_example() {
    println!("=== 派生宏示例 ===");

    // 模拟派生宏的效果
    // 实际需要 proc-macro crate

    #[derive(Debug, Clone)]
    struct User {
        name: String,
        age: u32,
    }

    let user = User {
        name: String::from("Alice"),
        age: 30,
    };

    println!("User: {:?}", user);
    println!("Cloned: {:?}", user.clone());

    // 派生宏自动生成了 Debug 和 Clone 实现

    println!("\n派生宏自动生成 trait 实现");
}

// --- 实用宏示例 ---
// 计时宏
macro_rules! time_it {
    ($name:expr, $block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let duration = start.elapsed();
        println!("{} 耗时: {:?}", $name, duration);
        result
    }};
}

// 条件编译宏
macro_rules! if_debug {
    ($($body:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $($body)*
        }
    };
}

// 链式方法宏示例（概念演示）
// 注意：实际项目中推荐使用迭代器适配器的链式调用

fn practical_macros() {
    println!("=== 实用宏示例 ===");

    // 计时宏
    let sum: i32 = time_it!("计算总和", { (1..=1000).sum() });
    println!("总和: {}", sum);

    // 条件编译宏
    if_debug! {
        println!("只在 debug 模式输出");
    }

    // Rust 原生支持的链式调用
    let result: Vec<i32> = (1..=5).map(|x| x * 2).filter(|x| x % 4 == 0).collect();
    println!("链式调用结果: {:?}", result);

    println!("\n宏可以简化重复的代码模式");
}

// --- 调试宏 ---
macro_rules! debug_var {
    ($var:ident) => {
        println!("{} = {:?}", stringify!($var), $var);
    };
}

macro_rules! debug_vars {
    ($($var:ident),*) => {
        $(
            debug_var!($var);
        )*
    };
}

fn debug_macros() {
    println!("=== 调试宏 ===");

    let x = 10;
    let y = "hello";
    let z = vec![1, 2, 3];

    debug_var!(x);
    debug_var!(y);
    debug_var!(z);

    debug_vars!(x, y, z);

    println!("\n调试宏帮助快速输出变量信息");
}

// --- 宏的最佳实践 ---
fn macro_best_practices() {
    println!("=== 宏的最佳实践 ===");

    println!("【1. 优先使用函数】");
    println!("  - 函数更易理解和维护");
    println!("  - 宏用于函数无法做到的场景");

    println!("\n【2. 文档注释】");
    println!("  - 使用 /// 和 //! 注释宏");
    println!("  - 解释参数和用法");

    println!("\n【3. 错误处理】");
    println!("  - 提供清晰的编译错误");
    println!("  - 使用 compile_error! 报错");

    println!("\n【4. 卫生性】");
    println!("  - 避免意外捕获变量");
    println!("  - 使用独特的变量名");

    println!("\n【5. 局限性】");
    println!("  - 宏定义不能在函数内");
    println!("  - 宏展开可能导致代码膨胀");
    println!("  - 调试困难");

    println!("\n【6. 测试】");
    println!("  - 为宏编写测试");
    println!("  - trybuild 用于编译错误测试");
}

pub fn run() {
    println!("\n========== 15_macros ==========");
    declarative_macros();
    macro_param_types();
    repetition_patterns();
    advanced_macros();
    macro_hygiene();
    built_in_macros();
    procedural_macros_intro();
    derive_macro_example();
    practical_macros();
    debug_macros();
    macro_best_practices();
}
