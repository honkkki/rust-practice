fn main() {
    let mut s = String::new();
    println!("{}", s);
    s = "hello".parse().unwrap();
    take_ownership(s);      // s的值发生了移动
    // println!("{}", s);       // error: s has been free

    let str = give_ownership();
    println!("{}", str);

    let s1 = String::from("get");
    let str1 = take_and_give(s1);
    println!("{}", str1);

    let str = String::from("hello");
    let s = str;        // move
    println!("{}", s);

}

fn take_ownership(str: String) {
    println!("{}", str)
}

fn give_ownership() -> String {
    let str = String::from("give");
    str
}

fn take_and_give(str: String) -> String {
    str
}


