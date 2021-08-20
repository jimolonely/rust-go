use std::io;

fn test() {}

fn main() {
    println!("请输入：");
    let mut msg = String::new();
    io::stdin()
        .read_line(&mut msg)
        .expect("输入失败");
    println!("你输入的是：{}", msg);
}
