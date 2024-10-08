use std::{env, fs};
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
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

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {query, file_path, ignore_case})
    }

}

/// 使用‘a 生命周期标注，保证参数 content 和 返回数组的生命周期是相同的
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

/// 使用‘a 生命周期标注，保证参数 content 和 返回数组的生命周期是相同的
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

/// 错误类型，使用了 trait 对象 Box<dyn Error>
/// 目前只需知道 Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，
/// 不过无需指定具体将会返回的值的类型。
/// 这提供了在不同的错误场景可能有不同类型的错误返回值的灵活性。
/// 这也就是 dyn，它是 “动态的”（“dynamic”）的缩写
/// Result 之后的 ? 是传播错误的简写
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    if results.is_empty() {
        println!("未找到字符串:{}", &config.query);
    } else {
        for line in results {
            println!("{line}");
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "safe";
        let contents = "\
Rsut:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_result() {
        let query = "Safe";
        let contents = "\
Rsut:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents));
    }

}



