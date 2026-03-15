// ============================================================
// 11_iterators.rs - 迭代器
// ============================================================
//
// 【核心概念】
// 迭代器 = 惰性序列 + 统一的遍历接口
// 是Rust零成本抽象的典型代表
//
// 【对比其他语言】
// - Java: Iterator接口，需要实现hasNext/next
// - Python: 迭代器协议 __iter__/__next__
// - C++: 迭代器模式（指针风格）
// - JavaScript: for...of 和迭代器协议
// - Rust: Iterator trait + 适配器模式
//
// 【Rust迭代器的特点】
// 1. 惰性求值 - 创建时不执行，消费时才执行
// 2. 零成本抽象 - 编译后与手写循环一样快
// 3. 组合式设计 - 适配器可任意组合
// 4. 内存安全 - 编译期检查生命周期
//
// 【Iterator Trait】
// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }
// ============================================================

// --- 迭代器基础 ---
fn iterator_basics() {
    println!("=== 迭代器基础 ===");

    // 【创建迭代器】
    let v = vec![1, 2, 3, 4, 5];

    // iter() - 不可变引用
    let iter: std::slice::Iter<i32> = v.iter();
    // 元素类型是 &i32

    // iter_mut() - 可变引用
    let mut v2 = vec![1, 2, 3];
    let iter_mut: std::slice::IterMut<i32> = v2.iter_mut();
    // 元素类型是 &mut i32

    // into_iter() - 获取所有权
    let iter_own: std::vec::IntoIter<i32> = v.into_iter();
    // 元素类型是 i32，v被消费

    // 【for循环自动使用迭代器】
    let v = vec![1, 2, 3];
    for item in &v {
        // &v 自动调用 iter()
        println!("item: {}", item);
    }

    let mut v = vec![1, 2, 3];
    for item in &mut v {
        // &mut v 自动调用 iter_mut()
        *item *= 2;
    }
    println!("after mutate: {:?}", v);

    for item in v {
        // v 自动调用 into_iter()
        println!("owned: {}", item);
    }
    // v已被消费

    // 【迭代器 vs 索引】
    // 迭代器更安全，编译器保证不会越界
    // 迭代器更快，编译器可以优化边界检查
}

// --- Iterator Trait详解 ---
fn iterator_trait() {
    println!("=== Iterator Trait ===");

    // 【自定义迭代器】
    // 实现 Iterator trait 创建自己的迭代器

    struct Counter {
        count: usize,
        max: usize,
    }

    impl Counter {
        fn new(max: usize) -> Counter {
            Counter { count: 0, max }
        }
    }

    // 实现 Iterator trait
    impl Iterator for Counter {
        type Item = usize; // 关联类型

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let counter = Counter::new(5);

    // 使用迭代器
    for n in counter {
        print!("{} ", n);
    }
    println!();

    // 【Iterator的默认方法】
    // Iterator trait 提供了很多默认方法
    // 只需要实现 next()，其他方法自动可用

    let counter = Counter::new(5);
    let sum: usize = counter.sum(); // sum() 是默认方法
    println!("sum: {}", sum);

    let counter = Counter::new(3);
    let collected: Vec<usize> = counter.collect();
    println!("collected: {:?}", collected);

    let mut counter = Counter::new(10);
    let nth = counter.nth(5); // 跳过5个，返回第6个
    println!("nth(5): {:?}", nth);

    // 【IntoIterator trait】
    // 让类型可以被for循环遍历
    // 大多数集合类型都实现了它
}

// --- 迭代器适配器 ---
// 适配器返回新的迭代器，可以链式调用

fn iterator_adapters() {
    println!("=== 迭代器适配器 ===");

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 【map - 转换每个元素】
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);

    // 【filter - 过滤元素】
    let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("evens: {:?}", evens);

    // 【filter_map - 过滤并转换】
    let parsed: Vec<i32> = vec!["1", "2", "three", "4"]
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("parsed: {:?}", parsed);

    // 【take - 取前n个】
    let first_three: Vec<&i32> = v.iter().take(3).collect();
    println!("first 3: {:?}", first_three);

    // 【skip - 跳过前n个】
    let after_three: Vec<&i32> = v.iter().skip(3).collect();
    println!("skip 3: {:?}", after_three);

    // 【take_while / skip_while】
    let take_while: Vec<&i32> = v.iter().take_while(|x| **x < 5).collect();
    println!("take_while < 5: {:?}", take_while);

    let skip_while: Vec<&i32> = v.iter().skip_while(|x| **x < 5).collect();
    println!("skip_while < 5: {:?}", skip_while);

    // 【chain - 连接两个迭代器】
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let chained: Vec<&i32> = v1.iter().chain(v2.iter()).collect();
    println!("chained: {:?}", chained);

    // 【zip - 配对两个迭代器】
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![30, 25, 35];
    let pairs: Vec<(&str, i32)> = names
        .iter()
        .zip(ages.iter())
        .map(|(n, a)| (*n, *a))
        .collect();
    println!("pairs: {:?}", pairs);

    // 【enumerate - 获取索引】
    for (i, v) in names.iter().enumerate() {
        println!("[{}] = {}", i, v);
    }

    // 【flatten - 展平嵌套】
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flat: Vec<&i32> = nested.iter().flatten().collect();
    println!("flattened: {:?}", flat);

    // 【flat_map - map + flatten】
    let words = vec!["hello", "world"];
    let chars: Vec<char> = words.iter().flat_map(|s| s.chars()).collect();
    println!("chars: {:?}", chars);

    // 【inspect - 调试用，查看中间值】
    let result: Vec<i32> = v
        .iter()
        .inspect(|x| println!("before filter: {}", x))
        .filter(|x| *x % 2 == 0)
        .inspect(|x| println!("after filter: {}", x))
        .map(|x| x * 2)
        .collect();
    println!("result: {:?}", result);

    // 【惰性求值】
    // 适配器调用时不会执行任何代码
    // 只有消费时才执行
    let iter = v.iter().map(|x| {
        println!("mapping {}", x); // 这时不会打印
        x * 2
    });
    println!("迭代器已创建，但未执行");

    // 消费时才执行
    let _: Vec<i32> = iter.collect(); // 现在打印
}

// --- 迭代器消费者 ---
// 消费者会执行迭代并产生结果

fn iterator_consumers() {
    println!("=== 迭代器消费者 ===");

    let v = vec![1, 2, 3, 4, 5];

    // 【collect - 收集到集合】
    let vec: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("collected: {:?}", vec);

    // 收集到其他类型
    use std::collections::HashSet;
    let set: HashSet<i32> = v.iter().copied().collect();
    println!("set: {:?}", set);

    // 【fold - 累积计算】
    let sum = v.iter().fold(0, |acc, x| acc + x);
    println!("sum (fold): {}", sum);

    // 带初始值的fold
    let product = v.iter().fold(1, |acc, x| acc * x);
    println!("product: {}", product);

    // 【reduce - 累积计算（无初始值）】
    let sum = v.iter().copied().reduce(|acc, x| acc + x);
    println!("sum (reduce): {:?}", sum);

    // 【sum / product】
    let sum: i32 = v.iter().sum();
    let product: i32 = v.iter().product();
    println!("sum: {}, product: {}", sum, product);

    // 【min / max】
    let min = v.iter().min();
    let max = v.iter().max();
    println!("min: {:?}, max: {:?}", min, max);

    // 【any / all】
    let has_even = v.iter().any(|x| x % 2 == 0);
    let all_positive = v.iter().all(|x| *x > 0);
    println!("has_even: {}, all_positive: {}", has_even, all_positive);

    // 【find / find_map】
    let first_even = v.iter().find(|x| *x % 2 == 0);
    println!("first_even: {:?}", first_even);

    let first_even_sq = v
        .iter()
        .find_map(|x| if x % 2 == 0 { Some(x * x) } else { None });
    println!("first_even_sq: {:?}", first_even_sq);

    // 【position】
    let pos = v.iter().position(|x| *x == 3);
    println!("position of 3: {:?}", pos);

    // 【count】
    let count = v.iter().count();
    println!("count: {}", count);

    // 【last】
    let last = v.iter().last();
    println!("last: {:?}", last);

    // 【for_each】
    v.iter().for_each(|x| print!("{} ", x));
    println!();

    // 【partition - 分区】
    let (evens, odds): (Vec<i32>, Vec<i32>) = v.iter().partition(|x| *x % 2 == 0);
    println!("evens: {:?}, odds: {:?}", evens, odds);

    // 【unzip - 解构配对】
    let pairs: Vec<(i32, i32)> = vec![(1, 2), (3, 4), (5, 6)];
    let (left, right): (Vec<i32>, Vec<i32>) = pairs.into_iter().unzip();
    println!("left: {:?}, right: {:?}", left, right);
}

// --- 常用迭代模式 ---
fn iterator_patterns() {
    println!("=== 常用迭代模式 ===");

    // 【1. 链式调用】
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: Vec<i32> = v
        .iter()
        .filter(|x| *x % 2 == 0) // 过滤偶数
        .map(|x| x * x) // 平方
        .take(3) // 取前3个
        .collect();
    println!("chain result: {:?}", result);

    // 【2. 处理嵌套结构】
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let all: Vec<&i32> = matrix.iter().flatten().collect();
    println!("flattened matrix: {:?}", all);

    // 【3. 分组处理】
    let text = "hello world how are you";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("words: {:?}", words);

    // 【4. 构建映射】
    use std::collections::HashMap;
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![30, 25, 35];
    let map: HashMap<&str, i32> = names
        .iter()
        .zip(ages.iter())
        .map(|(n, a)| (*n, *a))
        .collect();
    println!("map: {:?}", map);

    // 【5. 查找和替换】
    let v = vec![1, 2, 3, 4, 5];
    let replaced: Vec<i32> = v.iter().map(|x| if *x == 3 { 99 } else { *x }).collect();
    println!("replaced: {:?}", replaced);

    // 【6. 窗口迭代】
    let v = vec![1, 2, 3, 4, 5];
    println!("windows(2):");
    for win in v.windows(2) {
        println!("  {:?}", win);
    }

    // 【7. 块迭代】
    println!("chunks(2):");
    for chunk in v.chunks(2) {
        println!("  {:?}", chunk);
    }

    // 【8. 扫描状态】
    let v = vec![1, 2, 3, 4, 5];
    let scanned: Vec<i32> = v
        .iter()
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .collect();
    println!("scanned (partial sums): {:?}", scanned);
}

// --- 迭代器性能 ---
fn iterator_performance() {
    println!("=== 迭代器性能 ===");

    // 【零成本抽象】
    // 迭代器在编译后与手写循环一样快
    // 编译器会进行以下优化：
    // 1. 内联 - 迭代器方法会被内联
    // 2. 消除边界检查 - 编译器证明不会越界
    // 3. 循环展开 - 小循环会被展开
    // 4. SIMD - 向量化（自动并行）

    // 【示例对比】
    let v: Vec<i32> = (0..1000).collect();

    // 手写循环
    let mut sum1 = 0;
    for i in 0..v.len() {
        sum1 += v[i];
    }

    // 迭代器
    let sum2: i32 = v.iter().sum();

    assert_eq!(sum1, sum2);
    println!("两种方式性能相当");

    // 【惰性求值优势】
    // 只有需要的元素会被计算
    let v: Vec<i32> = (1..).take(10).collect(); // 无限迭代器取前10个
    println!("from infinite: {:?}", v);

    // 【选择建议】
    // - 简单遍历：for循环更直观
    // - 复杂转换：迭代器链更清晰
    // - 性能敏感：都一样快
    // - 可读性优先：迭代器通常是更好的选择
}

// --- 反向迭代 ---
fn reverse_iteration() {
    println!("=== 反向迭代 ===");

    let v = vec![1, 2, 3, 4, 5];

    // rev() 反转迭代器
    for n in v.iter().rev() {
        print!("{} ", n);
    }
    println!();

    // 双端迭代器
    let mut iter = v.iter();

    // 可以从两端获取
    println!("front: {:?}", iter.next()); // 从前面取
    println!("back: {:?}", iter.next_back()); // 从后面取
    println!("front: {:?}", iter.next());
    println!("back: {:?}", iter.next_back());
}

// --- 可迭代对象 ---
fn into_iter_demo() {
    println!("=== IntoIterator ===");

    // 【IntoIterator trait】
    // 允许类型被转换为迭代器
    // for循环自动使用它

    // 三种形式
    let v = vec![1, 2, 3];

    // 1. for item in &v  -> iter()
    //    item 类型: &i32
    for item in &v {
        println!("borrowed: {}", item);
    }
    println!("v still valid: {:?}", v);

    // 2. for item in &mut v  -> iter_mut()
    //    item 类型: &mut i32
    let mut v = vec![1, 2, 3];
    for item in &mut v {
        *item *= 2;
    }
    println!("v mutated: {:?}", v);

    // 3. for item in v  -> into_iter()
    //    item 类型: i32
    for item in v {
        println!("owned: {}", item);
    }
    // v 已被消费
    // println!("{:?}", v);  // 错误！

    // 【链式合并】
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let combined: Vec<i32> = v1.into_iter().chain(v2).collect();
    println!("combined: {:?}", combined);
}

pub fn run() {
    println!("\n========== 11_iterators ==========");
    iterator_basics();
    iterator_trait();
    iterator_adapters();
    iterator_consumers();
    iterator_patterns();
    iterator_performance();
    reverse_iteration();
    into_iter_demo();
}
