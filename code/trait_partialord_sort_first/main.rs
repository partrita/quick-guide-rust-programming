use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct Person {
    name: String,
    age: i32,
    height: i32,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.height <= 0 || other.height <= 0 {
            return None;
        }

        if self.height > other.height {
            Some(Ordering::Greater)
        } else if self.height < other.height {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

fn class_sorting(people: &mut Vec<Person>) {
    let len = people.len();

    for i in 0..(len - 1) {
        for j in (i + 1)..len {
            if people[i] > people[j] {
                people.swap(i, j);
            }
        }
    }
}

fn main() {
    let mut class: Vec<Person> = vec![
        Person {
            name: "aaa".to_owned(),
            age: 10,
            height: 110,
        },
        Person {
            name: "bbb".to_owned(),
            age: 10,
            height: 100,
        },
        Person {
            name: "ccc".to_owned(),
            age: 10,
            height: 120,
        },
        Person {
            name: "ddd".to_owned(),
            age: 10,
            height: 90,
        },
    ];

    class_sorting(&mut class);

    for p in class.iter() {
        println!("{} is {}", p.name, p.height);
    }
}
