use std::vec;

fn main() {
    // 简化代码的推断
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // 读取
    // 索引方式
    let n2 = &v[1];
    println!("n={}", n2);

    // get方式
    match v.get(1) {
        Some(n2) => println!("=2"),
        None => println!("无数据"),
    }

    // 越界
    if let None = v.get(100) {
        println!("100 is None");
    }

    // 遍历
    for x in &v {
        println!("x={}", x);
    }

    // 遍历并修改值
    for x in &mut v {
        *x += 10;
    }

    // 想存储多种类型
    let row = vec![
        Complecate::Int(3),
        Complecate::Text(String::from("jimo"))
    ];
}

enum Complecate{
    Int(i32),
    Text(String)
}
