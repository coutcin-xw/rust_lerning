// ============================================================
// 12_smart_pointers.rs - 智能指针
// ============================================================
//
// 【核心概念】
// 智能指针 = 指针 + 元数据 + 自动管理
// 它们拥有数据，在离开作用域时自动清理
//
// 【对比其他语言】
// - C++: unique_ptr, shared_ptr, weak_ptr
// - Java: 所有对象都是智能指针（GC管理）
// - Rust: Box, Rc, Arc, Cell, RefCell, Cow
//
// 【Rust智能指针的特点】
// 1. 自动内存管理 - 离开作用域自动drop
// 2. 所有权语义 - 编译期检查
// 3. 零开销抽象 - 与手写代码性能相当
// 4. 类型安全 - 编译期保证正确使用
//
// 【常用智能指针】
// - Box<T>: 堆分配，唯一所有权
// - Rc<T>: 引用计数，共享所有权（单线程）
// - Arc<T>: 原子引用计数，共享所有权（多线程）
// - Cell<T>: 内部可变性（Copy类型）
// - RefCell<T>: 内部可变性（运行时借用检查）
// - Cow<T>: 写时克隆
// ============================================================

use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

// --- Box<T> 堆分配 ---
fn box_demo() {
    println!("=== Box<T> 堆分配 ===");

    // 【基本用法】
    // Box 将数据分配在堆上
    let b = Box::new(5);
    println!("Box值: {}", b);

    // 【为什么要用Box？】

    // 1. 编译时大小未知
    // 递归类型必须有固定大小
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>), // Box<List> 有固定大小（指针大小）
        Nil,
    }

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("递归列表: {:?}", list);

    // 2. 大数据转移所有权
    // 避免大量数据复制
    struct LargeData {
        data: [u8; 1000],
    }

    let large = Box::new(LargeData { data: [0; 1000] });
    // 移动Box只是移动指针（8字节），而不是整个数组

    // 3. 特征对象
    // 存储不同类型的trait实现
    trait Animal {
        fn make_sound(&self) -> &str;
    }

    struct Dog;
    struct Cat;

    impl Animal for Dog {
        fn make_sound(&self) -> &str {
            "汪汪"
        }
    }

    impl Animal for Cat {
        fn make_sound(&self) -> &str {
            "喵喵"
        }
    }

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];

    for animal in animals.iter() {
        println!("动物叫声: {}", (**animal).make_sound());
    }

    // 【Box vs 普通引用】
    // Box拥有数据，引用只是借用
    let x = 5;
    let ref_x = &x; // 引用，不拥有
    let box_x = Box::new(x); // Box，拥有

    println!("引用: {}, Box: {}", ref_x, box_x);

    println!("Box用于：递归类型、大数据、trait对象");
}

// --- Rc<T> 引用计数 ---
// Reference Counting，共享所有权（单线程）

fn rc_demo() {
    println!("=== Rc<T> 引用计数 ===");

    // 【基本用法】
    // Rc 允许多个所有者共享同一数据
    let data = Rc::new(vec![1, 2, 3]);

    // clone 增加引用计数
    let data1 = Rc::clone(&data); // 引用计数 = 2
    let data2 = Rc::clone(&data); // 引用计数 = 3

    println!("数据: {:?}", data);
    println!("引用计数: {}", Rc::strong_count(&data));

    // 所有引用都可以访问数据
    println!("data1: {:?}", data1);
    println!("data2: {:?}", data2);

    // 当所有Rc离开作用域，引用计数归零，数据被释放

    // 【使用场景：共享数据】
    struct Node {
        value: i32,
        children: Vec<Rc<Node>>,
    }

    let leaf1 = Rc::new(Node {
        value: 1,
        children: vec![],
    });
    let leaf2 = Rc::new(Node {
        value: 2,
        children: vec![],
    });

    let parent = Rc::new(Node {
        value: 0,
        children: vec![Rc::clone(&leaf1), Rc::clone(&leaf2)],
    });

    println!("父节点有 {} 个子节点", parent.children.len());

    // 【Rc的限制】
    // 1. 只能用于单线程
    // 2. 默认不可变
    // 3. 可能导致循环引用（内存泄漏）

    // 【Weak<T> 防止循环引用】
    use std::rc::Weak;

    struct Owner {
        name: String,
        tools: Vec<Weak<Tool>>, // Weak引用，不增加strong_count
    }

    struct Tool {
        name: String,
        owner: Rc<Owner>,
    }

    let owner = Rc::new(Owner {
        name: String::from("张三"),
        tools: vec![],
    });

    let tool = Rc::new(Tool {
        name: String::from("锤子"),
        owner: Rc::clone(&owner),
    });

    // 使用 Weak 避免循环引用
    // Weak 升级为 Rc：weak.upgrade() -> Option<Rc<T>>

    println!("Rc用于单线程共享所有权");
}

// --- Arc<T> 原子引用计数 ---
// Atomic Reference Counting，线程安全的Rc

fn arc_demo() {
    println!("=== Arc<T> 原子引用计数 ===");

    use std::thread;

    // 【基本用法】
    // Arc 与 Rc 用法相同，但是线程安全的
    let data = Arc::new(vec![1, 2, 3]);

    let mut handles = vec![];

    for _ in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("线程读取: {:?}", data);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 【Arc vs Rc】
    // Arc: 原子操作，线程安全，有性能开销
    // Rc: 非原子操作，单线程，性能更好

    // 【Arc + Mutex 组合】
    // 共享可变数据的常用模式
    let counter = Arc::new(std::sync::Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("计数器最终值: {}", *counter.lock().unwrap());

    println!("Arc用于多线程共享所有权");
}

// --- Deref trait ---
// 解引用，让智能指针像普通引用一样使用

fn deref_demo() {
    println!("=== Deref trait ===");

    // 【Deref 让智能指针透明】
    // 可以像普通引用一样使用

    let x = 5;
    let y = &x; // 引用
    let z = Box::new(x); // Box

    assert_eq!(5, *y); // 解引用
    assert_eq!(5, *z); // Box 也支持解引用

    // 【自定义智能指针】
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let my_box = MyBox::new(10);
    println!("MyBox值: {}", *my_box);

    // 【Deref 强制转换】
    // Rust 自动进行解引用转换
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    let name = MyBox::new(String::from("Rust"));
    hello(&name); // 自动从 &MyBox<String> 转换为 &str
                  // 过程：&MyBox<String> -> &String -> &str

    // 【DerefMut trait】
    // 用于可变解引用
    let mut s = MyBox::new(String::from("hello"));
    // s.push_str(" world");  // 需要实现 DerefMut
}

// --- Drop trait ---
// 自定义清理行为

fn drop_demo() {
    println!("=== Drop trait ===");

    // 【Drop 自动调用】
    struct CustomPointer {
        name: String,
    }

    impl Drop for CustomPointer {
        fn drop(&mut self) {
            println!("释放: {}", self.name);
        }
    }

    {
        let _p1 = CustomPointer {
            name: String::from("指针1"),
        };
        let _p2 = CustomPointer {
            name: String::from("指针2"),
        };
        println!("作用域结束...");
    } // p2 先drop，p1 后drop（后进先出）

    // 【手动drop】
    let mut p = CustomPointer {
        name: String::from("手动释放"),
    };
    drop(p); // 显式调用
             // p.name = String::from("test");  // 错误！p已被移动

    println!("Drop用于自动资源清理");
}

// --- Cell<T> 内部可变性 ---
// 用于Copy类型，零开销的内部可变性

fn cell_demo() {
    println!("=== Cell<T> 内部可变性 ===");

    // 【问题】
    // 不可变引用不能修改值
    // let x = 5;
    // x = 6;  // 错误！

    // 【解决：Cell】
    // Cell 提供内部可变性
    let cell = Cell::new(5);

    println!("初始值: {}", cell.get());

    // 即使 cell 不是 mut，也可以修改内部值
    cell.set(10);
    println!("修改后: {}", cell.get());

    // 【Cell的工作原理】
    // 通过 get/set 方法访问
    // 只适用于 Copy 类型

    // 【使用场景】
    struct Counter {
        count: Cell<i32>,
    }

    let counter = Counter {
        count: Cell::new(0),
    };

    // 不需要 mut counter 就能修改 count
    counter.count.set(counter.count.get() + 1);
    println!("计数: {}", counter.count.get());

    // 【Cell vs RefCell】
    // Cell: Copy类型，通过get/set访问，无运行时检查
    // RefCell: 任意类型，通过borrow/borrow_mut访问，运行时检查

    println!("Cell用于Copy类型的内部可变性");
}

// --- RefCell<T> 内部可变性 ---
// 运行时借用检查

fn refcell_demo() {
    println!("=== RefCell<T> 内部可变性 ===");

    // 【问题】
    // 编译时借用检查有时太严格
    // 某些模式在运行时才能确定是否安全

    // 【RefCell 解决方案】
    // 将借用检查从编译期推迟到运行期
    let ref_cell = RefCell::new(5);

    println!("初始值: {}", ref_cell.borrow());

    // borrow() - 不可变借用
    // borrow_mut() - 可变借用
    *ref_cell.borrow_mut() = 10;
    println!("修改后: {}", ref_cell.borrow());

    // 【运行时借用检查】
    // 可以有多个不可变借用 OR 一个可变借用
    {
        let r1 = ref_cell.borrow(); // 不可变借用
        let r2 = ref_cell.borrow(); // 再来一个不可变借用
        println!("r1: {}, r2: {}", r1, r2);
    } // 借用结束

    {
        let mut r = ref_cell.borrow_mut(); // 可变借用
        *r += 1;
    }

    // 【运行时panic】
    // 违反借用规则会在运行时panic
    // let r1 = ref_cell.borrow();
    // let r2 = ref_cell.borrow_mut();  // panic! 已经有不可变借用了

    // 【Rc + RefCell 组合】
    // 共享可变数据（单线程）
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));

    let shared1 = Rc::clone(&shared);
    let shared2 = Rc::clone(&shared);

    // 任何引用都可以修改
    shared1.borrow_mut().push(4);
    shared2.borrow_mut().push(5);

    println!("共享数据: {:?}", shared.borrow());

    // 【使用场景】
    // 1. Mock对象（测试替身）
    // 2. 实现观察者模式
    // 3. 复杂数据结构中的回调

    println!("RefCell用于运行时借用检查的内部可变性");
}

// --- RefCell 借用规则示例 ---
fn refcell_borrow_rules() {
    println!("=== RefCell 借用规则 ===");

    let data = RefCell::new(5);

    // 【规则1：多个不可变借用】
    {
        let r1 = data.borrow();
        let r2 = data.borrow();
        println!("多个不可变: {} {}", *r1, *r2);
    }

    // 【规则2：单个可变借用】
    {
        let mut r = data.borrow_mut();
        *r = 10;
        println!("可变借用: {}", r);
    }

    // 【违反规则会panic】
    // 以下代码会panic（注释掉避免影响运行）
    // {
    //     let r1 = data.borrow();
    //     let mut r2 = data.borrow_mut();  // panic!
    // }

    // 【跟踪借用状态】
    println!("借用状态示例:");
    let data = RefCell::new(1);
    println!("  初始");

    {
        let _r1 = data.borrow();
        println!("  不可变借用 x1");
        {
            let _r2 = data.borrow();
            println!("  不可变借用 x2");
        }
        println!("  不可变借用 x1");
    }
    println!("  无借用");

    {
        let _r = data.borrow_mut();
        println!("  可变借用");
    }
    println!("  无借用");
}

// --- Cow<T> 写时克隆 ---
// Copy on Write，避免不必要的克隆

fn cow_demo() {
    println!("=== Cow<T> 写时克隆 ===");

    use std::borrow::Cow;

    // 【基本概念】
    // Cow 可以持有借用或拥有的数据
    // 写入时才克隆

    fn process_input(input: Cow<str>) -> Cow<str> {
        // 如果不需要修改，直接返回借用
        if input.contains('X') {
            // 需要修改，克隆后返回
            let mut owned = input.into_owned();
            owned = owned.replace('X', "Y");
            Cow::Owned(owned)
        } else {
            input
        }
    }

    // 场景1：不需要修改
    let borrowed = Cow::Borrowed("hello");
    let result = process_input(borrowed);
    println!("结果（借用）: {}", result);

    // 场景2：需要修改
    let borrowed = Cow::Borrowed("hello X");
    let result = process_input(borrowed);
    println!("结果（拥有）: {}", result);

    // 【使用场景】
    // 1. 函数可能修改输入，但大多数情况不修改
    // 2. 避免不必要的字符串克隆
    // 3. 处理可能是静态字符串或动态字符串

    println!("Cow用于写时克隆优化");
}

// --- 智能指针选择指南 ---
fn smart_pointer_guide() {
    println!("=== 智能指针选择指南 ===");

    println!("根据场景选择智能指针：");

    println!("\n【所有权模型】");
    println!("  唯一所有权 -> Box<T>");
    println!("  共享所有权（单线程） -> Rc<T>");
    println!("  共享所有权（多线程） -> Arc<T>");

    println!("\n【可变性】");
    println!("  内部可变性（Copy类型） -> Cell<T>");
    println!("  内部可变性（任意类型） -> RefCell<T>");
    println!("  多线程可变 -> Arc<Mutex<T>>");

    println!("\n【组合模式】");
    println!("  共享可变数据（单线程） -> Rc<RefCell<T>>");
    println!("  共享可变数据（多线程） -> Arc<Mutex<T>>");

    println!("\n【性能考虑】");
    println!("  零开销 -> Cell<T>（Copy类型）");
    println!("  运行时检查 -> RefCell<T>");
    println!("  原子操作 -> Arc<T>");

    println!("\n【常见组合】");
    let examples: HashMap<&str, &str> = [
        ("递归数据结构", "Box<T>"),
        ("共享只读数据", "Rc<T> / Arc<T>"),
        ("共享可变数据", "Rc<RefCell<T>> / Arc<Mutex<T>>"),
        ("回调/观察者", "RefCell<T>"),
        ("trait对象", "Box<dyn Trait>"),
    ]
    .iter()
    .cloned()
    .collect();

    for (scene, ptr) in examples {
        println!("  {} -> {}", scene, ptr);
    }
}

pub fn run() {
    println!("\n========== 12_smart_pointers ==========");
    box_demo();
    rc_demo();
    arc_demo();
    deref_demo();
    drop_demo();
    cell_demo();
    refcell_demo();
    refcell_borrow_rules();
    cow_demo();
    smart_pointer_guide();
}
