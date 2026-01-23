#![allow(dead_code)]

#[derive(Debug)]
enum WEEK {
    Sunday(String, i32),
    Monday(String),
    Tuesday(String),
    Wednesday(String),
    Thursday(String),
    Friday(String),
    Saturday {
        what: String,
        place: String,
        when: i32,
    },
}

fn main() {
    let schedule: [WEEK; 2] = [
        WEEK::Sunday("Sleep".to_string(), 10),
        WEEK::Saturday {
            what: "Party".to_string(),
            place: "Club".to_string(),
            when: 22,
        },
    ];

    for day in schedule.into_iter() {
        match day {
            WEEK::Sunday(todo, hours) => println!("Sunday: do {} for {} hours", todo, hours),
            WEEK::Monday(todo) => println!("Monday: do {}", todo),
            WEEK::Tuesday(todo) => println!("Tuesday: do {}", todo),
            WEEK::Wednesday(todo) => println!("Wednesday: do {}", todo),
            WEEK::Thursday(todo) => println!("Thursday: do {}", todo),
            WEEK::Friday(todo) => println!("Friday: do {}", todo),
            WEEK::Saturday { what, place, when } => {
                println!("Saturday: do {} at {} from {}", what, place, when)
            }
        }
    }
}
