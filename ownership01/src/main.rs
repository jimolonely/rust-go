fn main() {
    let s = String::from("jimo");
    // let s1 = s;
    let s1 = s.clone();
    println!("s={},s1={}", s, s1);

    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    println!("x1={}", x);
    // println!("s={}", s); // 不能调用s

    let s3 = give_ownership();
    let s4 = String::from("s4");
    let s5 = takes_and_gives_back(s4);
    println!("s3={},s5={}", s3, s5);
}

fn takes_ownership(ss: String) {
    println!("ss={}", ss)
}

fn makes_copy(x: i32) {
    println!("x={}", x)
}

fn give_ownership() -> String {
    let ss = String::from("hehe");
    ss
}

fn takes_and_gives_back(a: String) -> String {
    a
}
