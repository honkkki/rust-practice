#[derive(Debug)]
enum Cell {
    Int(i32),
    String(String),
}

fn main() {
    let mut v = vec![1, 2, 3];
    println!("{:?}", v);

    let mut num = v[1];
    num = num + 1;
    println!("{} {:?}", num, v);

    for i in &mut v {
        *i += 10;
    }

    println!("{:?}", v);


    match v.get(1) {
        Some(num) => println!("index:1 num is {}", num),
        None => println!("no index:1 number"),
    }

    let mut v = Vec::new();
    v.push("hello");
    v.push("rust");
    v[1] = "golang";
    println!("{:?}", v);

    // vec and enum
    let v = vec![
        Cell::Int(1),
        Cell::String("orange".to_string()),
    ];

    println!("{:?}", v);

}
