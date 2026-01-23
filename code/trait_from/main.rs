#![allow(dead_code)]

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
    isbn: String,
}

impl From<Book> for u32 {
    fn from(book: Book) -> u32 {
        book.isbn.parse().unwrap_or(0)
    }
}

fn main() {
    let the_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
        isbn: String::from("718503105"),
    };

    let rust_in_action = Book {
        title: String::from("Rust in Action"),
        author: String::from("Tim McNamara"),
        published: 20210810,
        isbn: String::from("1617294551"),
    };

    let isbn: u32 = the_book.into();
    let isbn2 = u32::from(rust_in_action);
    println!("The book is {isbn} and Rust in Action is {isbn2}");
}
