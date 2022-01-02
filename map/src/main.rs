use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    let str = String::from("rust");
    let color = String::from("orange");
    let color2 = String::from("purple");
    let name = String::from("karina");
    map.insert("name", &str);       // 不传引用会导致所有权转移
    map.insert("color", &color);
    map.insert("color", &color2);
    map.entry("name").or_insert(&name);

    println!("{:?}", map);
    println!("{:?}", str);

    let teams = vec!["blue", "red"];
    let scores = vec![10, 20];
    let result: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
    println!("{:?}", result);
    println!("{:?}", teams);

    let color = String::from("blue");
    let score = result.get(&color.as_str());
    match score {
        Some(v) => { println!("blue score: {}", v) }
        None => (),
    }

    for (k, v) in &result {
        println!("{}: {}", k, v);
    }

    println!("{:?}", result);
    println!("------------------------------");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
