#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug, Clone, Default)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.author == other.author
    }
}

impl PartialEq<Person> for Book {
    fn eq(&self, other: &Person) -> bool {
        self.author.contains(&other.name)
    }
}

impl PartialEq<Book> for Person {
    fn eq(&self, other: &Book) -> bool {
        other.author.contains(&self.name)
    }
}

fn main() {
    let second = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };
    let steve = Person {
        name: "Steve Klabnik".to_string(),
        age: 30,
    };
    if second == steve {
        println!("Yes, this book is writtend by {:?}", steve);
    }
}
