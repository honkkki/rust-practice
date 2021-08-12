use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = args().collect();
    println!("{:?}", args);
    println!("{}", args[1]);

    let filename = format!("{}", args[1]);
    let mut file = File::open(filename).expect("open file error");
    let mut res = String::new();
    file.read_to_string(&mut res).unwrap();
    println!("file data: {}", res);
}
