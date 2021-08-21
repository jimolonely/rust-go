fn main() {
    let user1 = build_user(String::from("jimo"), String::from("jimo@jimo.com"));

    // 快速复制用户
    let user2 = User{
        email:String::from("hehe"),
        username:String::from("hehe@jimo.com"),
        ..user1
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u32,
}

fn build_user(username: String, email: String) -> User {
    User {
        username, // 变量名一样，减少书写映射
        email,
        active: true,
        sign_in_count: 1,
    }
}

// 元组结构体
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);