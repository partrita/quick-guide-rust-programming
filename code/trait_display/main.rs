use std::fmt;

struct Book {
    title: String,
    author: String,
    published: u32,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Title: {}\nAuthor: {}\nPublished: {}\n",
            self.title, self.author, self.published
        )
    }
}

fn main() {
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };

    println!("{}", book);
}
