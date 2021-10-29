use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    m.insert("name", "rust");

    println!("{:?}", m);

    for (k, v) in &m {
        println!("{}: {}", k, v);
    }

    println!("{:?}", m);

    let str = String::from("rust and golang");
    for char in str.chars() {
        println!("{}", char);
    }

    println!("{}", str);

    if m.len() == 0 {
        println!("map's len == 0")
    } else {
        println!("{}", m.len())
    }

}

