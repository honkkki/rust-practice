use std::collections::HashMap;
use rand::Rng;

mod lib;

fn main() {
    lib::eat();
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map);
    let num = rand::thread_rng().gen_range(1, 101);
    println!("{}", num);
    println!("---------------------------------");
    lib::drink();


}
