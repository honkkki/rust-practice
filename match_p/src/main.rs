fn main() {
    let num: u8 = 1;
    match num {
        1 => println!("one"),
        2 => println!("two"),
        _ => (),
    }

    let v = Some(6);
    match v {
        Some(6) => {
            println!("hello 6");
        }

        _ => (),
    }

    if let Some(6) = v {
        println!("six");
    } else {
        println!("others");
    }
}
