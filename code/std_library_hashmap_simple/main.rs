#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let mut library: HashMap<String, Book> = HashMap::new();
    let the_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };
    library.insert("718503105".to_owned(), the_book);
    library.insert(
        "1617294551".to_owned(),
        Book {
            title: String::from("Rust in Action"),
            author: String::from("Tim McNamara"),
            published: 20210810,
        },
    );
    library.insert(
        "0000000000".to_owned(),
        Book {
            title: String::from("The another book"),
            author: String::from("Unknown"),
            published: 20111111,
        },
    );

    let found = library.get("0000000000");
    println!("{:?}", found);
    let not_found = library.get("xxxxxxxxxx");
    println!("{:?}", not_found);
}
