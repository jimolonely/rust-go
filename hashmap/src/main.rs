use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    m.insert(1, 2);
    m.insert(1, 2);
    println!("m={:?}", m);

    // zip
    let name = vec![String::from("jimo"), String::from("hehe")];
    let age = vec![18, 19];

    let na: HashMap<_, _> = name.iter().zip(age.iter()).collect();
    println!("na={:?}", na);

    // hashmap也会剥夺字符串String的所有权,str不会
    let s = "寂寞".to_string();
    let s1 = "哈哈".to_string();
    let mut m = HashMap::new();
    m.insert(s, s1); // key和value也不能是同一个对象，也会被剥夺
                     // println!("s={:}", s); // 报错
    println!("m={:?}", m);

    // 访问
    let s = "寂寞".to_string();
    println!("m.get(s)={:?}", m.get(&s));

    // 遍历
    for (k, v) in &m {
        println!("k={},v={}", k, v);
    }

    // 只有不存在键时才插入新值
    m.entry("寂寞".to_string()).or_insert("牛逼".to_string());
    m.entry("啦啦".to_string()).or_insert("牛逼".to_string());
    println!("m={:?}", m);

    // 基于旧值更新新值
    let text = "你 是个 很强的 人 你 是个 人";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map={:?}", map);
}
