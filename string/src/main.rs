fn main() {
    // 遍历中文字符
    let s = "hello哈哈";
    for v in s.chars() {
        println!("{}", v);
    }

    let s1 = "hello".to_string();
    let s2 = "rust".to_string();
    let s = s1 + " " + &s2;     // s1 borrowed
    println!("{}", s);

    let s1 = "hello".to_string();
    let s = format!("{} {}", s1, s2);
    println!("{}", s);

}
