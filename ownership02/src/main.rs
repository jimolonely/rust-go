fn main() {
    let s1 = String::from("jimo");
    let len = get_len(&s1);
    println!("len={}", len);

    let mut s2 = String::from("jimo");
    change(&mut s2);
    println!("s2={}", s2);

    // 下面只要不是在一个作用域内，就可以多次创建一个可变变量的引用
    let mut s3 = String::from("jimo");
    {
        let s4 = &mut s3;
    }
    let s5 = &mut s3;

    // 不可变引用可创建多个，但可变只能有一个，且不能和不可变同时存在
    let mut s6 = String::from("jimo");
    let s7 = &s6; // ok
    let s8 = &s6; // ok
    // let s9 = &mut s6; // error

    let nothing = dangle(); // 悬垂指针，指针指向可能不存在的内存区域
}

fn get_len(s: &String) -> usize {
    // 不可变引用
    s.len()
}

fn change(s: &mut String) {
    // 可变引用
    s.push_str(",hehe");
}

fn dangle() -> &String {
    let s = String::from("jimo");
    &s
}
