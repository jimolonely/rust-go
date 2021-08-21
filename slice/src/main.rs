fn main() {
    let s = String::from("jimo hehe");
    let i = first_word(&s);
    println!("i={}", i);

    // s.clear(); // 这里清理掉之后再用i访问s就会有问题，所以有了切片

    let jimo = &s[0..4];
    let hehe = &s[5..9];
    println!("jimo={},hehe={}", jimo, hehe);

    // 语法糖
    let jimo1 = &s[..4];
    println!("jimo1={}", jimo1);

    println!("first_word2={}", first_word2(&s));

    // 字符串字面量,可方便String和str传参
    let word = first_word3(&s[..]);
    let sl = "string literal";
    let word = first_word3(&sl[..]);
    let word = first_word3(sl);

    // 数组切片
    let a = [1, 2, 3, 4, 5];
    let sa = &a[1..3];
    println!("a[1..3]={:?}", sa);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
