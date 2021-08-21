use std::collections::HashMap;

use rand::Rng;

// 包导入和合并
use std::io::{self, Write};
use std::{cmp::Ordering, io}; // use std::io;

//通配符
use std::collections::*;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("rand={}", rand::thread_rng().gen_range(1, 101));
}
