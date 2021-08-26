use std::env;
use std::process;

use jimoline::Config;

// cargo run ion Cargo.toml
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("解析参数失败：{}", err);
        process::exit(1);
    });

    println!("query={},file={}", config.query, config.filename);

    if let Err(e) = jimoline::run(config) {
        println!("运行错误：{}", e);
        process::exit(1);
    }
}

