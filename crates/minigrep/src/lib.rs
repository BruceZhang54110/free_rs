use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

// 创建构造函数
impl Config {
    /// &'static str 表示静态生命周期
    /// 这个字符串字面值在程序的整个运行期间都是有效的
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{}", contents);
    Ok(())
}
