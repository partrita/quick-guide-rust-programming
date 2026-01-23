#![allow(dead_code)]

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let mut book_array = [
        Book {
            title: String::from("The Fellowship of the Ring"),
            author: String::from("J. R. R. Tolkien"),
            published: 19540729,
        },
        Book {
            title: String::from("The Two Towers"),
            author: String::from("J. R. R. Tolkien"),
            published: 19541111,
        },
        Book {
            title: String::from("The Return of the King"),
            author: String::from("J. R. R. Tolkien"),
            published: 19551020,
        },
    ];

    for b in book_array.iter_mut() {
        b.author = String::from("John Ronald Reuel Tolkien");
    }

    println!("{:?}", book_array);
}
