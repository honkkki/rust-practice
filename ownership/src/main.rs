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

    let s1 = String::from("rust");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);
    println!("------------------------");

    let (s, len) = calc_len(String::from("rust"));
    println!("{}, {}", s, len);

    let arr = [1,2,3];
    let a = arr;
    println!("{:?} {:?}", arr, a);


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

fn calc_len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}


