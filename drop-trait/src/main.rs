use std::mem::drop;

fn main() {
    let p1 = Pointer {
        data: String::from("呵呵"),
    };
    let p2 = Pointer {
        data: String::from("寂寞"),
    };

    // 提前清理
    drop(p1);
    println!("最后结束");
}

struct Pointer {
    data: String,
}

impl Drop for Pointer {
    fn drop(&mut self) {
        println!("销毁数据：{}", self.data);
    }
}
