fn main() {
    let local = IPAddrType::IPV4(127, 0, 0, 1);

    // Option<T> 的表示
    let n1 = Some(5);
    let s1 = Some("jimo");
    let n2: Option<i32> = None;

    value_in_coin2(ColorCoin::YUAN(Color::RED));

    let five = Some(5);
    println!("five+1={:?}", plus_one(five));
    println!("None+1={:?}", plus_one(None));

    // if let简化匹配
    let n = Some(0u8);
    if let Some(3) = n {
        println!("match 3");
    } else {
        println!("not match 3");
    }
}

enum IPAddrType {
    IPV4(u8, u8, u8, u8),
    IPV6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    FEN,
    JIAO,
    YUAN,
}

fn value_in_coin(coin: Coin) -> u32 {
    match coin {
        Coin::FEN => 1,
        Coin::JIAO => 10,
        Coin::YUAN => 100,
    }
}

#[derive(Debug)]
enum Color {
    GREEN,
    YELLO,
    RED,
}

enum ColorCoin {
    FEN,
    JIAO,
    YUAN(Color),
}

fn value_in_coin2(coin: ColorCoin) -> u32 {
    match coin {
        ColorCoin::FEN => 1,
        ColorCoin::JIAO => 10,
        ColorCoin::YUAN(color) => {
            println!("元的颜色是：{:?}", color);
            100
        }
    }
}

// Option<T> 匹配
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

// _通配符匹配默认
fn match_default(x: i32) {
    match x {
        1 => println!("1"),
        3 => println!("3"),
        _ => println!("other"),
    }
}
