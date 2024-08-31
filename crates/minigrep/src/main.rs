use std::env;
use std::process;
use minigrep::Config;


fn main() {
    println!("Hello, world!, This is minigrep, search your words");
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    // 参数赋值
    // 如果没发生panic!,unwrap 会返回 Ok内部封装的值，
    // 当值是Err是，该方法会调用一个闭包
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        // 立即停止程序并将传递给它的数字作为退出状态码
        process::exit(1);
    });

    // 读取文件
    //let contents = fs::read_to_string(config.file_path)
    // 设置发生panic!时的msg
    //.expect("Should have been able to read the file");
    //println!("读取到文件内容：{}", contents);

    // 使用 if let 检查run 是否返回一个Err,只关心检测错误
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    
}
