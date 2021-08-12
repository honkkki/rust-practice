fn main() {
    let s1 = String::from("long str");
    let s2 = "str";
    println!("longest str: {}", longest(s1.as_str(), s2));
}

// 保证返回的引用的生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
