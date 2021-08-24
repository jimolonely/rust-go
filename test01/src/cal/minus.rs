pub fn minus2(a: i32, b: i32) -> i32 {
    a - b
}

use crate::cal::add::add2;

pub fn mix() {
    println!("add={}", add2(1, 2));
}
