use std::{path::PathBuf, time::Duration};

// fn main() {
//     println!("Hello, world!");
// }
#[derive(Debug, Default)]
struct MyConfiguration {
    output: Option<PathBuf>,
    search_path: Vec<PathBuf>,
    timeout: Duration,
    check: bool,
}
impl MyConfiguration {}

fn main() {
    let mut conf = MyConfiguration::default();
    conf.check = true;
    println!("conf={:#?}", conf);
}

//2.2 用format!连接字符串
//
// fn say_hello(name: &str) -> String {
//     format!("Hello {}!", name)
// }
