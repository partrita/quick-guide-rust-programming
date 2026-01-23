#![allow(dead_code)]

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
    isbn: String,
}

impl TryFrom<Book> for u32 {
    type Error = u32;

    fn try_from(book: Book) -> Result<u32, Self::Error> {
        match book.isbn.parse::<u32>() {
            Ok(n) => Ok(n),
            Err(_) => Err(0),
        }
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

    let isbn: Result<u32, u32> = the_book.try_into();
    let isbn2 = u32::try_from(rust_in_action);
    println!("The book is {:?} and Rust in Action is {:?}", isbn, isbn2);
}
