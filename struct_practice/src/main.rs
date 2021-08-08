use std::fmt::Debug;

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    login_times: u64,
}

#[derive(Debug)]
struct Rect {
    width: u32,
    length: u32,
}

impl Rect {
    // 结构体的方法
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    // 关联函数 构造
    fn new_rect(width: u32, length: u32) -> Rect {
        Rect {
            width,
            length,
        }
    }
}

fn main() {
    let mut u1 = User {
        name: "jisoo".to_string(),
        email: "123@123.com".to_string(),
        login_times: 1,
    };

    let u2 = &mut u1;
    u2.email = "karina@123.com".to_string();
    println!("{:#?}", u2);
    println!("----------------------------");

    let rect = Rect {
        width: 20,
        length: 50,
    };
    println!("{}", rect.area());
    println!("{}", rect.get_width());
    println!("{:#?}", rect);

    let rect = Rect::new_rect(50, 50);
    println!("{:#?}", rect);

}
