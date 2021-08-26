fn main() {}



// 3.1 生成器

#[derive(Debug, PartialEq)]
pub struct Foo {
    bar: String,
}

pub struct FooBuilder {
    bar: String,
}

impl FooBuilder {
    pub fn new() -> FooBuilder {
        FooBuilder {
            bar: String::from("X"),
        }
    }
    pub fn name(mut self, bar: String) -> FooBuilder {
        self.bar = bar;
        self
    }

    pub fn build(self) -> Foo {
        Foo { bar: self.bar }
    }
}

#[test]
fn builder_test() {
    let foo = Foo {
        bar: String::from("Y"),
    };
    let foo_from_builder: Foo = FooBuilder::new().name("Y".to_string()).build();
    assert_eq!(foo, foo_from_builder);
}

//用mem::{take(_), replace(_)}在修改枚举变体时保持值的所有权

// use std::mem;
// enum MyEnum {
//     A { name: String, x: u8 },
//     B { name: String },
// }

// fn a_to_b(e: &mut MyEnum) {
//     // we mutably borrow `e` here. This precludes us from changing it directly
//     // as in `*e = ...`, because the borrow checker won't allow it. Therefore
//     // the assignment to `e` must be outside the `if let` clause.
//     *e = if let MyEnum::A { ref mut name, x: 0 } = *e {
//         // this takes out our `name` and put in an empty String instead
//         // (note that empty strings don't allocate).
//         // Then, construct the new enum variant (which will
//         // be assigned to `*e`, because it is the result of the `if let` expression).
//         MyEnum::B {
//             name: mem::take(name),
//         }

//     // In all other cases, we return immediately, thus skipping the assignment
//     } else {
//         return;
//     }
// }

//2.5 将集合视为智能指针
//
// use std::ops::Deref;

// struct Vec<T> {
//     data: T,
// }
// impl<T> Deref for Vec<T> {
//     type Target = Type;
//     fn deref(&self) -> &<Self as Deref>::Target {
//         todo!()
//     }
// }
//2.4 Default 特性
//
// use std::{path::PathBuf, time::Duration};

// #[derive(Debug, Default)]
// struct MyConfiguration {
//     output: Option<PathBuf>,
//     search_path: Vec<PathBuf>,
//     timeout: Duration,
//     check: bool,
// }
// impl MyConfiguration {}

// fn main() {
//     let mut conf = MyConfiguration::default();
//     conf.check = true;
//     println!("conf={:#?}", conf);
// }

//2.2 用format!连接字符串
//
// fn say_hello(name: &str) -> String {
//     format!("Hello {}!", name)
// }
