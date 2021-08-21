fn main() {
    let r = Rectangle {
        width: 12.2,
        height: 11.1,
    };
    println!("r={:#?}", r);

    (&r).area();
    println!("r.area={}", r.area()); // 等价于 (&r).area()

    // 多个参数
    println!("r canhold r:{}", r.can_hold(&r));

    println!("square={:?}", Rectangle::square(12.0));
}

#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数
    fn square(size: f32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
