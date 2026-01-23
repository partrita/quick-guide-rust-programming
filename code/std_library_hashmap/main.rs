use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let mut library: HashMap<Book, String> = HashMap::new();
    library.insert(
        Book {
            title: String::from("The Rust Programming Language"),
            author: String::from("Steve Klabnik and Carol Nichols"),
            published: 20230228,
        },
        "1718503105".to_owned(),
    );
    library.insert(
        Book {
            title: String::from("Rust in Action"),
            author: String::from("Tim McNamara"),
            published: 20210810,
        },
        "1617294551".to_owned(),
    );
    library.insert(
        Book {
            title: String::from("The another book"),
            author: String::from("Unknown"),
            published: 20111111,
        },
        "0000000000".to_owned(),
    );

    let found = library.get(&Book {
        title: String::from("The another book"),
        author: String::from("Unknown"),
        published: 20111111,
    });
    println!("{:?}", found);
}
