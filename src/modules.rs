// ============================================================
// 14_modules.rs - 模块系统
// ============================================================
//
// 【核心概念】
// 模块系统用于组织和管理代码
// 包括包（Package）、箱（Crate）、模块（Module）
//
// 【对比其他语言】
// - Java: package + import + public/private
// - Python: package + import + _前缀表示私有
// - C++: namespace + #include + public/private
// - Go: package + import + 首字母大小写决定可见性
// - Rust: mod + use + pub 可见性
//
// 【模块系统层次】
// 1. Package（包）- Cargo 项目，包含 Cargo.toml
// 2. Crate（箱）- 编译单元，二进制或库
// 3. Module（模块）- 代码组织单元，用 mod 定义
// 4. Path（路径）- 访问模块中项的方式
//
// 【核心关键字】
// - mod: 定义模块
// - pub: 公开可见性
// - use: 导入路径
// - as: 别名
// - super: 父模块
// - self: 当前模块
// ============================================================

// --- 模块基础 ---
fn module_basics() {
    println!("=== 模块基础 ===");

    // 【内联模块】
    // 使用 mod { } 定义模块
    mod math {
        // 默认私有，外部不可访问
        fn internal_helper() -> i32 {
            42
        }

        // pub 使函数公开
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }

        // 可以调用私有函数
        pub fn get_secret() -> i32 {
            internal_helper()
        }
    }

    // 【调用模块中的函数】
    // 使用 :: 访问
    println!("1 + 2 = {}", math::add(1, 2));
    println!("3 * 4 = {}", math::multiply(3, 4));
    println!("秘密数字: {}", math::get_secret());

    // math::internal_helper();  // 错误！私有函数

    println!("\n模块用 mod 定义，用 pub 控制可见性");
}

// --- 模块可见性 ---
fn visibility_rules() {
    println!("=== 模块可见性 ===");

    mod outer {
        // 【pub】完全公开
        pub fn public_fn() {
            println!("公开函数");
            private_fn(); // 内部可以调用私有
        }

        // 【私有】默认，仅当前模块可见
        fn private_fn() {
            println!("私有函数");
        }

        // 【pub(crate)】当前 crate 内可见
        pub(crate) fn crate_fn() {
            println!("crate 内可见");
        }

        // 【pub(super)】父模块可见
        pub(super) fn super_fn() {
            println!("父模块可见");
        }

        // 【pub(in path)】指定路径可见
        pub fn path_fn() {
            println!("指定路径可见");
        }

        // 【嵌套模块】
        pub mod inner {
            pub fn inner_fn() {
                println!("内部模块函数");
            }
        }
    }

    outer::public_fn();
    // outer::private_fn();  // 错误！私有
    // outer::crate_fn();    // 在同一个 crate 中可以调用
    outer::inner::inner_fn();

    println!("\n可见性规则：");
    println!("  pub - 完全公开");
    println!("  pub(crate) - crate 内可见");
    println!("  pub(super) - 父模块可见");
    println!("  pub(in path) - 指定路径可见");
    println!("  无 pub - 私有（当前模块可见）");
}

// --- 结构体的可见性 ---
fn struct_visibility() {
    println!("=== 结构体可见性 ===");

    mod shapes {
        // 【公开结构体】
        #[derive(Debug)]
        pub struct Rectangle {
            pub width: u32,   // 公开字段
            pub height: u32,  // 公开字段
            internal_id: u32, // 私有字段
        }

        impl Rectangle {
            // 公开构造函数
            pub fn new(width: u32, height: u32) -> Self {
                Rectangle {
                    width,
                    height,
                    internal_id: 0,
                }
            }

            // 访问私有字段
            pub fn get_id(&self) -> u32 {
                self.internal_id
            }
        }

        // 【元组结构体】
        pub struct Point(pub i32, pub i32); // 字段公开

        // 【单元结构体】
        pub struct Unit;
    }

    // 使用
    let rect = shapes::Rectangle::new(10, 20);
    println!("矩形: {:?}", rect);
    println!("宽: {}, 高: {}", rect.width, rect.height);
    // println!("{}", rect.internal_id);  // 错误！私有字段

    let point = shapes::Point(1, 2);
    println!("点: ({}, {})", point.0, point.1);

    println!("\n结构体字段可以单独设置可见性");
}

// --- use 关键字 ---
fn use_keyword() {
    println!("=== use 关键字 ===");

    mod animals {
        pub struct Dog {
            pub name: String,
        }

        pub struct Cat {
            pub name: String,
        }

        pub fn make_sound() {
            println!("动物叫声");
        }
    }

    // 【基本导入】
    use animals::Dog;
    let dog = Dog {
        name: String::from("旺财"),
    };
    println!("狗的名字: {}", dog.name);

    // 【导入多个】
    use animals::{make_sound, Cat};
    let cat = Cat {
        name: String::from("咪咪"),
    };
    println!("猫的名字: {}", cat.name);
    make_sound();

    // 【导入全部】
    use animals::*;
    let dog2 = Dog {
        name: String::from("小黑"),
    };
    println!("另一只狗: {}", dog2.name);

    // 【as 别名】
    use animals::Dog as Puppy;
    let puppy = Puppy {
        name: String::from("小狗"),
    };
    println!("小狗名字: {}", puppy.name);

    println!("\nuse 用于导入，as 用于别名");
}

// --- 路径 ---
fn paths_demo() {
    println!("=== 路径 ===");

    // 【绝对路径】
    // 从 crate 根开始
    // crate::module::item

    // 【相对路径】
    // 从当前模块开始
    // self::item
    // super::item

    mod parent {
        pub fn parent_fn() {
            println!("父模块函数");
        }

        pub mod child {
            // 【self】当前模块
            pub fn use_self() {
                println!("当前模块");
            }

            // 【super】父模块
            pub fn call_parent() {
                super::parent_fn(); // 调用父模块函数
            }

            // 【绝对路径】
            pub fn use_absolute() {
                // crate::modules_demo::...  在实际项目中
                println!("使用绝对路径");
            }
        }
    }

    parent::child::use_self();
    parent::child::call_parent();

    println!("\n路径类型：");
    println!("  crate:: - 从 crate 根开始");
    println!("  self:: - 从当前模块开始");
    println!("  super:: - 从父模块开始");
}

// --- 文件模块 ---
fn file_modules() {
    println!("=== 文件模块 ===");

    // 【模块文件结构】
    // src/
    // ├── main.rs          - 二进制 crate 根
    // ├── lib.rs           - 库 crate 根
    // ├── module1.rs       - 模块文件
    // ├── module2/
    // │   ├── mod.rs       - 模块目录
    // │   └── submodule.rs - 子模块
    // └── ...

    // 【在 main.rs 或 lib.rs 中声明模块】
    // mod module1;         // 声明模块
    // mod module2;         // 声明目录模块

    // 【使用模块】
    // module1::function();
    // module2::submodule::function();

    println!("文件模块结构：");
    println!("  module.rs - 单文件模块");
    println!("  module/mod.rs - 目录模块");
    println!("  module/sub.rs - 子模块");
}

// --- 模块设计模式 ---
fn module_patterns() {
    println!("=== 模块设计模式 ===");

    // 【模式1：类型、实现、trait 分离】
    // types.rs - 类型定义
    // impls.rs - 实现
    // traits.rs - trait 定义
    // mod.rs - 重新导出

    println!("【设计模式1】类型分离");
    println!("  types/mod.rs - 重新导出子模块");
    println!("  types/models.rs - 数据结构");
    println!("  types/impls.rs - 实现");

    // 【模式2：隐藏实现细节】
    println!("\n【设计模式2】隐藏实现");
    println!("  pub use 导出公共 API");
    println!("  私有模块存放内部实现");

    // 【模式3：facade 模式】
    println!("\n【设计模式3】Facade");
    println!("  lib.rs 作为统一入口");
    println!("  pub use 导出所有公共 API");

    // 示例
    pub mod api {
        // 私有实现
        mod private {
            pub fn helper() -> String {
                String::from("内部帮助函数")
            }
        }

        // 公开接口
        pub fn process() -> String {
            private::helper()
        }

        // 重新导出
        pub use std::collections::HashMap;
    }

    let result = api::process();
    println!("API 调用: {}", result);

    // HashMap 被重新导出
    let mut map = api::HashMap::new();
    map.insert("key", "value");
}

// --- re-exporting ---
fn re_exporting() {
    println!("=== 重新导出 ===");

    // 【pub use】重新导出
    // 将内部模块的项公开到外部

    mod backend {
        pub struct Database {
            pub name: String,
        }

        pub fn connect() -> Database {
            Database {
                name: String::from("mydb"),
            }
        }

        pub mod models {
            pub struct User {
                pub name: String,
            }
        }
    }

    // 重新导出，简化 API
    pub use backend::connect;
    pub use backend::models::User;
    pub use backend::Database;

    // 用户可以直接使用
    let db = connect();
    println!("数据库: {}", db.name);

    let user = User {
        name: String::from("Alice"),
    };
    println!("用户: {}", user.name);

    println!("\npub use 用于重新导出，简化公共 API");
}

// --- 模块与测试 ---
fn modules_and_tests() {
    println!("=== 模块与测试 ===");

    // 【测试模块】
    // 通常在文件底部定义
    mod calculator {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        // 测试模块
        #[cfg(test)] // 只在测试时编译
        mod tests {
            use super::*; // 导入父模块所有项

            #[test]
            fn test_add() {
                assert_eq!(add(1, 2), 3);
            }
        }
    }

    println!("测试模块使用 #[cfg(test)] 标记");
    println!("  use super::* 导入被测试代码");
}

// --- 模块最佳实践 ---
fn module_best_practices() {
    println!("=== 模块最佳实践 ===");

    println!("【1. 模块组织】");
    println!("  - 相关功能放在同一模块");
    println!("  - 使用目录模块组织大型模块");
    println!("  - mod.rs 作为模块入口");

    println!("\n【2. 可见性设计】");
    println!("  - 默认私有，按需公开");
    println!("  - 最小化公开 API");
    println!("  - 使用 pub(crate) 限制 crate 内");

    println!("\n【3. 导入策略】");
    println!("  - 避免过度使用 * 导入");
    println!("  - 在函数内导入而非文件顶部（有时）");
    println!("  - 使用 as 解决命名冲突");

    println!("\n【4. 重新导出】");
    println!("  - lib.rs 导出公共 API");
    println!("  - 隐藏内部模块结构");
    println!("  - 提供简洁的接口");

    println!("\n【5. 文档】");
    println!("  //! 模块级文档");
    println!("  /// 项级文档");
    println!("  使用 doc 注释");
}

// --- 实际项目结构示例 ---
fn project_structure() {
    println!("=== 实际项目结构示例 ===");

    println!("【库项目结构】");
    println!("my_lib/");
    println!("├── Cargo.toml");
    println!("└── src/");
    println!("    ├── lib.rs        # 库根");
    println!("    ├── models.rs     # 数据模型");
    println!("    ├── models/       # 或目录形式");
    println!("    │   ├── mod.rs");
    println!("    │   ├── user.rs");
    println!("    │   └── product.rs");
    println!("    ├── api/");
    println!("    │   ├── mod.rs");
    println!("    │   └── handlers.rs");
    println!("    └── utils.rs");

    println!("\n【二进制项目结构】");
    println!("my_app/");
    println!("├── Cargo.toml");
    println!("└── src/");
    println!("    ├── main.rs       # 主入口");
    println!("    ├── lib.rs        # 库代码（可选）");
    println!("    └── bin/          # 多个可执行文件");
    println!("        ├── tool1.rs");
    println!("        └── tool2.rs");

    println!("\n【推荐结构】");
    println!("  - 简单项目：单文件");
    println!("  - 中型项目：多模块文件");
    println!("  - 大型项目：目录模块 + 子 crate");
}

pub fn run() {
    println!("\n========== 14_modules ==========");
    module_basics();
    visibility_rules();
    struct_visibility();
    use_keyword();
    paths_demo();
    file_modules();
    module_patterns();
    re_exporting();
    modules_and_tests();
    module_best_practices();
    project_structure();
}
