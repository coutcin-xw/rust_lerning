mod basics;
mod ownership;
mod borrowing;
mod struct_enum;
mod pattern_match;
mod error_handling;
mod trait_generics;
mod lifetime;
mod concurrency;
mod closures;
mod iterators;
mod smart_pointers;
mod async_await;
mod modules;
mod macros;

fn main() {
    // 按顺序运行所有示例
    basics::run();
    ownership::run();
    borrowing::run();
    struct_enum::run();
    pattern_match::run();
    error_handling::run();
    trait_generics::run();
    lifetime::run();
    concurrency::run();
    closures::run();
    iterators::run();
    smart_pointers::run();
    async_await::run();
    modules::run();
    macros::run();
    
    println!("\n========== 全部示例完成! ==========");
}