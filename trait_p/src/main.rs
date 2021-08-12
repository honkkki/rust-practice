use trait_p::{Tweet, Summary};

fn main() {
    let username = String::from("jisoo");
    let content = String::from("hello");
    let tweet = Tweet {
        username,
        content,
    };

    println!("new tweet--- {}", tweet.summarize());

}
