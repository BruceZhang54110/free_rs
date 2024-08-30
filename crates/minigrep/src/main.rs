use std::{env, error::Error, fs, process};


struct Config {
    query: String,
    file_path: String,
}

// 创建构造函数
impl Config {
    fn new(args: &[String]) -> Config {
        // 这里 Config 尝试获取 args 中值的所有权将违反 Rust 的借用规则，这里使用clone() 拷贝一份出来
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config {
            query,
            file_path,
        }
    }

    /// &'static str 表示静态生命周期
    /// 这个字符串字面值在程序的整个运行期间都是有效的
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path, })
    }

}

/// 错误类型，使用了 trait 对象 Box<dyn Error>
/// 目前只需知道 Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，
/// 不过无需指定具体将会返回的值的类型。
/// 这提供了在不同的错误场景可能有不同类型的错误返回值的灵活性。
/// 这也就是 dyn，它是 “动态的”（“dynamic”）的缩写
/// Result 之后的 ? 是传播错误的简写
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{}", contents);
    Ok(())
}


fn main() {
    println!("Hello, world!, This is minigrep");
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    // 参数赋值
    // 如果没发生panic!,unwrap 会返回 Ok内部封装的值，
    // 当值是Err是，该方法会调用一个闭包
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        // 立即停止程序并将传递给它的数字作为退出状态码
        process::exit(1);
    });

    // 读取文件
    //let contents = fs::read_to_string(config.file_path)
    // 设置发生panic!时的msg
    //.expect("Should have been able to read the file");
    //println!("读取到文件内容：{}", contents);

    // 使用 if let 检查run 是否返回一个Err,只关心检测错误
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
    
}
