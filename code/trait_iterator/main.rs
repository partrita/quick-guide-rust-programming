#![allow(dead_code)]

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

#[derive(Debug)]
struct BookShelf<'a> {
    books: &'a [Book],
}

impl<'a> Iterator for BookShelf<'a> {
    type Item = &'a Book;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((first, remains)) = self.books.split_first() {
            self.books = remains;
            Some(first)
        } else {
            None
        }
    }
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

    let mut mybooks_iter = BookShelf {
        books: &mut book_array,
    };
    println!("{:?}", mybooks_iter.next());
    println!("{:?}", mybooks_iter.next());
    println!("{:?}", mybooks_iter.next());
    println!("{:?}", mybooks_iter.next());

    let mybooks_for = BookShelf {
        books: &mut book_array,
    };
    for b in mybooks_for {
        println!("{:?}", b);
    }
}
