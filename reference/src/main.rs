fn main() {
    let mut s = String::from("hello");
    let len = calc_len(&mut s);
    println!("{}, {}", s, len);

    let s = String::from("hello");
    let s1 = &s;
    println!("{}", s1);
    println!("{}", s1.len());

    let mut s = String::from("rust");
    let s1 = &mut s;
    s1.push_str(" hello");
    println!("{}", s1);
    println!("{}", s);


    let r1 = &mut s;
    let r2 = &mut s;
    println!("{} {}", r1, r2);

}

fn calc_len(s: &mut String) -> usize {
    // let ss = s;
    // println!("{}", ss);
    println!("{}", s);

    s.push_str(" rust");

    s.len()
}

// 悬空指针 s离开作用域已被释放
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
