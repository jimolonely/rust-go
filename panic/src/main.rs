use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn");

    let v = vec![1, 2, 3];
    // v[99]; // RUST_BACKTRACE=full cargo run

    // 匹配捕获错误信息结果Result<T,E>
    let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file)=>file,
    //     Err(error)=>{
    //         panic!("打开文件失败：{:?}",error);
    //     }
    // };

    // 匹配相信错误信息
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("创建文件也失败了：{:?}", e),
            },
            other_error => panic!("打开文件失败：{:?}", other_error),
        },
    };

    // unwrap简化
    // let f = File::open("hh.txt").unwrap();

    // expect
    let f = File::open("hh.txt").expect("打开hh失败");
}
