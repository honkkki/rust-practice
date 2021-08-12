pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct News {
    pub title: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for News {
    fn summarize(&self) -> String {
        format!("news title: {}, news content: {}", &self.title, &self.content)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", &self.username, &self.content)
    }
}

