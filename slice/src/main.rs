fn main() {
    let s = String::from("hello world");
    let index = first_world(&s);
    println!("{}", index);

    let s = String::from("hello rust");
    let h = &s[0..5];
    let r = &s[6..];
    let all = &s[..];
    println!("{}, {}\n{}", h, r, all);
    println!("-------------------");

    let s1 = String::from("hello rust");
    let s2 = "hello rust";
    let i1 = first_world(&s1);
    let i2 = first_world(s2);
    println!("{} {}", i1, i2);

    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[..3];
    println!("{:?} {:?}", arr, slice)
}

// 参数为字符串切片类型
fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
