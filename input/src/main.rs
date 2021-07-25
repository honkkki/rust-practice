use std::io;

fn main() {
    let mut input = String::new();
    println!("hello!");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("I say: {}", input)
        }

        _ => {panic!()}
    }
}
