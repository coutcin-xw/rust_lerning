mod basics;
mod ownership;
mod borrowing;
mod struct_enum;
mod pattern_match;
mod error_handling;
mod trait_generics;
mod lifetime;
mod concurrency;

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
    
    println!("\n========== 全部示例完成! ==========");
}