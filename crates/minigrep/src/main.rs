use std::env;

fn main() {
    println!("Hello, world!, This is minigrep");
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    // 参数赋值
    let query = &args[1];
    let file_path = &args[2];

    println!("query:{}, file_path:{}", query, file_path);
}
