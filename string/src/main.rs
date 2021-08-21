fn main() {
    // 创建字符串
    let mut s = "hello".to_string();

    // 更新字符串
    s.push_str(",jimo");
    println!("s={}", s);

    let s1 = String::from("hello");
    let s2 = String::from(",world");
    // s3接过了s1的所有权，s1就死了，这比复制字符串高效
    let s3 = s1 + &s2;
    // println!("s1={}", s1); // s1不能再用了，相当于add(self, s:&str) -> String
    println!("s3={}", s3);

    // 拼接多个字符串
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = String::from("Yes!");

    // 不会剥夺所有权
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s={},s1={}", s, s1);

    // 字符串索引与切片
    let s = "你好";
    // println!("s[0..1]={}", &s[0..1]); // 出错
    println!("s[0..3]={}", &s[0..3]); // 你

    // 遍历字符串
    for c in s.chars() {
        println!("c={}", c);
    }
}
