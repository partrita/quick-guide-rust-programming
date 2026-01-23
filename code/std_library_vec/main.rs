#![allow(dead_code)]

use std::vec::Vec;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn find_rust<'a>(books: &'a Vec<Book>) -> Vec<&'a Book> {
    let mut found: Vec<&Book> = Vec::new();
    for b in books.iter() {
        if b.title.contains("Rust") {
            found.push(b);
        }
    }
    found
}

fn main() {
    let rust_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };

    let rust_in_action = Book {
        title: String::from("Rust in Action"),
        author: String::from("Tim McNamara"),
        published: 20210810,
    };

    let another = Book {
        title: String::from("The another book"),
        author: String::from("Unknown"),
        published: 20111111,
    };

    let mut library: Vec<Book> = Vec::new();
    library.push(rust_book);
    library.push(rust_in_action);
    library.push(another);

    let rust_books = find_rust(&library);
    let mut only_titles: Vec<String> = Vec::new();

    if rust_books.is_empty() {
        println!("Cannot find any Rust book");
    } else {
        for b in rust_books.into_iter() {
            let mut title = b.title.clone();
            title.push('\n');
            only_titles.push(title);
        }
    }

    let collect = only_titles.into_iter().collect::<String>();
    println!("{}", collect);
}
