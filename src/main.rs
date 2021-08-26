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
