// 核心模块（按序号学习）
#[path = "01_basics.rs"] mod basics;
#[path = "02_ownership.rs"] mod ownership;
#[path = "03_borrowing.rs"] mod borrowing;
#[path = "04_struct_enum.rs"] mod struct_enum;
#[path = "05_pattern_match.rs"] mod pattern_match;
#[path = "06_error_handling.rs"] mod error_handling;
#[path = "07_trait_generics.rs"] mod trait_generics;
#[path = "08_lifetime.rs"] mod lifetime;
#[path = "09_concurrency.rs"] mod concurrency;

// 进阶模块
#[path = "10_closures.rs"] mod closures;
#[path = "11_iterators.rs"] mod iterators;
#[path = "12_smart_pointers.rs"] mod smart_pointers;
#[path = "13_async_await.rs"] mod async_await;
#[path = "14_modules.rs"] mod modules;
#[path = "15_macros.rs"] mod macros;

fn main() {
    // ===== 核心模块（建议按顺序学习）=====
    basics::run();
    ownership::run();
    borrowing::run();
    struct_enum::run();
    pattern_match::run();
    error_handling::run();
    trait_generics::run();
    lifetime::run();
    concurrency::run();
    
    // ===== 进阶模块 =====
    closures::run();
    iterators::run();
    smart_pointers::run();
    async_await::run();
    modules::run();
    macros::run();
    
    println!("\n========== 全部示例完成! ==========");
}