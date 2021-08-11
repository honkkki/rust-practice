use std::collections::HashMap;


fn main() {
    let mut map = HashMap::new();
    let str = String::from("rust");
    map.insert("name", &str);       // 不传引用会导致所有权转移

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
        Some(v) => {println!("blue score: {}", v)},
        None => (),
    }

    for (k, v) in &result {
        println!("{}: {}", k, v);
    }

    println!("{:?}", result);

}
