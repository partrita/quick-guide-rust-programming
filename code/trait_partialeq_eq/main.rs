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
        if self.author.contains("Unknown") {
            return false;
        }
        self.author.contains(&other.name)
    }
}

impl PartialEq<Book> for Person {
    fn eq(&self, other: &Book) -> bool {
        if self.name.contains("Unknown") {
            return false;
        }
        other.author.contains(&self.name)
    }
}

fn main() {
    let second = Book {
        title: String::from("Necronomicon"),
        author: String::from("Unknown"),
        published: 20230228,
    };
    let unknown = Person {
        name: "Unknown".to_string(),
        age: 30,
    };
    if second == unknown {
        println!("Yes, this book is writtend by {:?}.", unknown);
    } else {
        println!("No, we don't know who wrote it.")
    }
}
