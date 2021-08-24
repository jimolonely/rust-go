mod lib;
mod cal;
use cal::add;

fn main() {
    lib::algo();
    println!("minus={}", cal::minus::minus2(5, 2));
    println!("cal::add2={}", add::add2(2, 4));
}
