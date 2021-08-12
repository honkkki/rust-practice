mod rest {
    pub struct Breakfast {
        pub toast: String,
        pub fruit: String,
    }

    pub struct Drink {
        pub name: String,
    }

    impl Breakfast {
        pub fn make(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruit: String::from("peaches"),
            }
        }
    }

    impl Drink {
        pub fn drink(name: &str) -> Drink {
            let name = name.to_string();
            Drink {
                name
            }
        }
    }
}

pub fn eat() {
    let meal = rest::Breakfast::make("wheat");
    println!("{}", meal.toast);
    println!("{}", meal.fruit);
}


pub fn drink() {
    let d = rest::Drink::drink("cola");
    println!("drink: {}", d.name)
}