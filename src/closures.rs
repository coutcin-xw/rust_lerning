// ============================================================
// 10_closures.rs - 闭包
// ============================================================
//
// 【核心概念】
// 闭包 = 匿名函数 + 捕获环境变量
// 可以保存到变量、作为参数传递、作为返回值
//
// 【对比其他语言】
// - JavaScript: 箭头函数 () => {}
// - Python: lambda x: x * 2
// - Java: Lambda表达式 (x) -> x * 2
// - C++: Lambda表达式 [捕获](参数){主体}
// - Rust: |参数| {主体} 或 |参数| 表达式
//
// 【Rust闭包的特点】
// 1. 类型推断 - 通常不需要标注参数和返回类型
// 2. 捕获环境 - 可以访问外部变量
// 3. 三种trait - Fn / FnMut / FnOnce
// 4. 匿名类型 - 每个闭包都有唯一的类型
//
// 【与函数的区别】
// 函数：fn关键字，显式类型标注，不能捕获环境
// 闭包：匿名，类型推断，可以捕获环境
// ============================================================

// --- 闭包语法 ---
fn closure_syntax() {
    println!("=== 闭包语法 ===");

    // 【基本语法】
    // |参数| { 主体 }
    // 或 |参数| 表达式（单行时花括号可省略）

    // 定义闭包
    let add = |a, b| a + b;
    println!("add(1, 2) = {}", add(1, 2));

    // 带类型标注（可选）
    let add_explicit = |a: i32, b: i32| -> i32 { a + b };
    println!("add_explicit(3, 4) = {}", add_explicit(3, 4));

    // 多行闭包
    let complex = |x| {
        let doubled = x * 2;
        let squared = doubled * doubled;
        squared
    };
    println!("complex(3) = {}", complex(3));

    // 无参数闭包
    let say_hello = || println!("Hello!");
    say_hello();

    // 【闭包 vs 函数】
    // 函数必须显式标注类型
    fn add_fn(a: i32, b: i32) -> i32 {
        a + b
    }

    // 闭包可以推断类型
    let add_closure = |a, b| a + b;
    // 但推断后类型就固定了
    // add_closure(1.0, 2.0);  // 错误！已推断为i32

    println!("函数: {}", add_fn(1, 2));
    println!("闭包: {}", add_closure(1, 2));

    // 【何时使用闭包】
    // 1. 短小的、一次性的函数
    // 2. 需要捕获环境变量
    // 3. 作为高阶函数的参数
}

// --- 捕获环境变量 ---
fn capture_environment() {
    println!("=== 捕获环境变量 ===");

    // 【闭包可以捕获外部变量】
    // 这是闭包与普通函数的主要区别

    let x = 10;
    let y = 20;

    // 闭包捕获x和y
    let sum = || x + y;
    println!("sum() = {}", sum());

    // 【捕获方式】
    // 编译器会根据闭包如何使用变量来决定捕获方式：
    // 1. 不可变借用 (&T) - 只读
    // 2. 可变借用 (&mut T) - 需要&mut
    // 3. 移动 (T) - 获取所有权

    // 1. 不可变借用
    let list = vec![1, 2, 3];
    println!("Before: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);
    // 闭包只借用list，list仍然可用
    println!("Before calling: {:?}", list);
    only_borrows();
    println!("After: {:?}", list);

    // 2. 可变借用
    let mut list = vec![1, 2, 3];
    let mut borrows_mutably = || list.push(4);
    // 这里list被可变借用，不能同时访问
    // println!("{:?}", list);  // 错误！
    borrows_mutably();
    println!("After mutation: {:?}", list);

    // 3. 移动所有权
    let list = vec![1, 2, 3];
    let move_closure = move || {
        // move关键字强制获取所有权
        println!("Owned: {:?}", list);
    };
    // move常用于：
    // - 将闭包传给新线程
    // - 返回闭包
    // - 闭包生命周期超过捕获的变量

    move_closure();
    // list已经移动，不能再使用
    // println!("{:?}", list);  // 错误！

    println!("捕获方式由编译器自动决定，可用move强制移动");
}

// --- Fn / FnMut / FnOnce Traits ---
// 闭包实现的trait决定了如何捕获变量

// 【FnOnce】
// - 获取捕获变量的所有权
// - 只能调用一次
// - 闭包中移动了捕获的值

// 【FnMut】
// - 可变借用捕获的变量
// - 可以修改捕获的变量
// - 可以多次调用

// 【Fn】
// - 不可变借用捕获的变量
// - 只读访问
// - 可以多次调用

// 【继承关系】Fn <: FnMut <: FnOnce
// 实现Fn的也实现了FnMut和FnOnce

// 示例：不同trait的闭包
fn fn_traits() {
    println!("=== Fn/FnMut/FnOnce Traits ===");

    // Fn - 不可变借用
    let x = 10;
    let closure_fn = || x + 1; // 只读取x
    println!("Fn result: {}", closure_fn());

    // FnMut - 可变借用
    let mut counter = 0;
    let mut closure_fn_mut = || {
        counter += 1; // 修改捕获的变量
        counter
    };
    println!("FnMut result: {}", closure_fn_mut());
    println!("FnMut result: {}", closure_fn_mut());

    // FnOnce - 移动
    let s = String::from("hello");
    let closure_fn_once = || {
        let _ = s; // 移动s的所有权
        println!("Consumed!");
    };
    closure_fn_once();
    // closure_fn_once();  // 错误！只能调用一次

    println!("选择trait的原则：最严格的能通过编译就选最严格的");
}

// 接受Fn闭包的函数
fn call_with_fn(f: &impl Fn() -> i32) {
    println!("Fn result: {}", f());
}

// 接受FnMut闭包的函数
fn call_with_fn_mut(f: &mut impl FnMut() -> i32) {
    println!("FnMut result: {}", f());
}

// 接受FnOnce闭包的函数
fn call_with_fn_once(f: impl FnOnce()) {
    f();
}

// --- 闭包作为参数 ---
fn closure_as_parameter() {
    println!("=== 闭包作为参数 ===");

    // 【三种方式】

    // 1. 泛型参数（最灵活）
    fn apply<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(x)
    }

    let double = |x| x * 2;
    println!("apply(double, 5) = {}", apply(double, 5));

    // 2. impl Trait（更简洁）
    fn apply_simple(f: impl Fn(i32) -> i32, x: i32) -> i32 {
        f(x)
    }

    println!("apply_simple(|x|x+1, 5) = {}", apply_simple(|x| x + 1, 5));

    // 3. 函数指针（限制最大，但性能最好）
    fn apply_fn(f: fn(i32) -> i32, x: i32) -> i32 {
        f(x)
    }

    fn triple(x: i32) -> i32 {
        x * 3
    }

    println!("apply_fn(triple, 5) = {}", apply_fn(triple, 5));

    // 【选择建议】
    // - 需要捕获环境：泛型或impl Trait
    // - 不需要捕获：函数指针
    // - 需要存储闭包：Box<dyn Fn>

    // 【闭包类型是匿名的】
    // 每个闭包都有唯一的、编译器生成的类型
    // 所以不能用具体类型作为参数，必须用泛型或trait对象

    // 使用trait对象存储不同闭包
    let closures: Vec<Box<dyn Fn(i32) -> i32>> = vec![Box::new(|x| x + 1), Box::new(|x| x * 2)];
    for (i, c) in closures.iter().enumerate() {
        println!("closure[{}] = {}", i, c(10));
    }
}

// --- 闭包作为返回值 ---
fn closure_as_return_value() {
    println!("=== 闭包作为返回值 ===");

    // 【返回闭包】
    // 必须使用impl Trait或Box<dyn Trait>

    // impl Trait方式
    fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
        // 返回一个闭包
        move |y| x + y // 必须move，因为闭包的生命周期可能超过x
    }

    let add5 = make_adder(5);
    println!("add5(10) = {}", add5(10));

    // 【返回不同闭包】
    // 如果需要返回不同类型的闭包，用Box<dyn Trait>
    fn get_closure(n: bool) -> Box<dyn Fn(i32) -> i32> {
        if n {
            Box::new(|x| x + 1)
        } else {
            Box::new(|x| x * 2)
        }
    }

    let c1 = get_closure(true);
    let c2 = get_closure(false);
    println!("c1(5) = {}", c1(5));
    println!("c2(5) = {}", c2(5));

    // 【为什么需要move？】
    // 函数返回后，局部变量被销毁
    // 闭包如果借用这些变量，会产生悬垂引用
    // move将所有权转移给闭包，避免这个问题
}

// --- 闭包的实际应用 ---
fn closure_use_cases() {
    println!("=== 闭包实际应用 ===");

    // 【1. 迭代器方法】
    let nums = vec![1, 2, 3, 4, 5];

    // map - 转换每个元素
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);

    // filter - 过滤元素
    let evens: Vec<&i32> = nums.iter().filter(|x| *x % 2 == 0).collect();
    println!("evens: {:?}", evens);

    // fold - 累积计算
    let sum: i32 = nums.iter().fold(0, |acc, x| acc + x);
    println!("sum: {}", sum);

    // 【2. 自定义排序】
    let mut people = vec![("Alice", 30), ("Bob", 25), ("Charlie", 35)];
    people.sort_by(|a, b| a.1.cmp(&b.1)); // 按年龄排序
    println!("sorted: {:?}", people);

    // 【3. 延迟计算】
    let expensive_result = || {
        println!("computing...");
        42 // 复杂计算
    };

    println!("闭包已定义，但未执行");
    println!("result: {}", expensive_result()); // 这时才执行

    // 【4. 回调函数】
    fn process<F>(data: Vec<i32>, callback: F) -> Vec<i32>
    where
        F: Fn(i32) -> i32,
    {
        data.into_iter().map(callback).collect()
    }

    let result = process(vec![1, 2, 3], |x| x * x);
    println!("processed: {:?}", result);

    // 【5. 构建器模式】
    struct QueryBuilder {
        filters: Vec<Box<dyn Fn(&str) -> bool>>,
    }

    impl QueryBuilder {
        fn new() -> Self {
            QueryBuilder { filters: vec![] }
        }

        fn filter<F>(mut self, f: F) -> Self
        where
            F: Fn(&str) -> bool + 'static,
        {
            self.filters.push(Box::new(f));
            self
        }

        fn execute<'a>(&self, data: &[&'a str]) -> Vec<&'a str> {
            data.iter()
                .filter(|item| self.filters.iter().all(|f| f(item)))
                .copied()
                .collect()
        }
    }

    let query = QueryBuilder::new()
        .filter(|s| s.len() > 3)
        .filter(|s| s.starts_with('A'));

    let data = vec!["Apple", "Bat", "Ant", "Cat", "Alice"];
    let result = query.execute(&data);
    println!("query result: {:?}", result);
}

// --- Cacher：闭包缓存模式 ---
// 演示如何存储和使用闭包
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn cacher_demo() {
    println!("=== 闭包存储（Cacher模式）===");

    let mut expensive = Cacher::new(|num| {
        println!("calculating slowly...");
        std::thread::sleep(std::time::Duration::from_secs(1));
        num * 2
    });

    println!("First call...");
    let result1 = expensive.value(10);
    println!("result1: {}", result1);

    println!("Second call (cached)...");
    let result2 = expensive.value(10);
    println!("result2: {}", result2);

    // 第二次调用不会重新计算
}

// --- 闭包性能 ---
fn closure_performance() {
    println!("=== 闭包性能 ===");

    // 【性能特点】
    // 1. 闭包本质是结构体，存储捕获的变量
    // 2. 调用闭包是直接的函数调用，无虚函数开销
    // 3. 泛型闭包会被单态化，性能与普通函数相当

    // 【内存布局】
    // 不捕获变量的闭包：零大小类型
    let c1 = || 42;
    println!("sizeof(c1) = {}", std::mem::size_of_val(&c1)); // 0

    // 捕获变量的闭包：大小等于捕获的变量
    let x = 10i32;
    let c2 = || x;
    println!("sizeof(c2) = {}", std::mem::size_of_val(&c2)); // 4

    let y = 20i32;
    let c3 = || x + y;
    println!("sizeof(c3) = {}", std::mem::size_of_val(&c3)); // 8

    // 【内联优化】
    // 编译器会将简单的闭包内联
    // 所以闭包通常没有额外开销
}

pub fn run() {
    println!("\n========== 10_closures ==========");
    closure_syntax();
    capture_environment();
    fn_traits();
    closure_as_parameter();
    closure_as_return_value();
    closure_use_cases();
    cacher_demo();
    closure_performance();
}
