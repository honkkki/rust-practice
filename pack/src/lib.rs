mod rest {
    pub struct Breakfast {
        pub toast: String,
        pub fruit: String,
    }

    impl Breakfast {
        pub fn make(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat() {
    let meal = rest::Breakfast::make("wheat");
    println!("{}", meal.toast);
    println!("{}", meal.fruit);
}