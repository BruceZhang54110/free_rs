
use std::env;
use std::process;

use minigrep1::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args); // stderr
    // 传入两个参数

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep1::run(config) {
        eprintln!("Application error {}", e);
        process::exit(1)
    }
}

