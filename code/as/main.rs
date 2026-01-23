use std::num::ParseIntError;

fn string_to_digit(input: String) -> i32 {
    let mut ret = 0;
    for c in input.chars() {
        ret = ret * 10;
        ret += c as i32 - '0' as i32;
    }
    ret
}

fn parse_example(input: &str) -> Result<i32, ParseIntError> {
    input.parse()
}

fn main() {
    println!("{}", string_to_digit("1234".to_string()));
    let ret = parse_example("1234");
    match ret {
        Ok(value) => {
            println!("Parsed integer: {}", value);
        }
        Err(_) => {
            println!("Failed to parse the string as an integer");
        }
    }
}
