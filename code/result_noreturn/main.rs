fn check_command_valid(cmd: &str) -> Result<(), String> {
    match cmd {
        "good" => Ok(()),
        "unsupported" => Err("Unsupported command".to_owned()),
        "bad" => Err("Bad command".to_owned()),
        _ => Err("Wierd command".to_owned()),
    }
}

fn main() {
    match check_command_valid("blabla") {
        Ok(_) => (),
        Err(error_msg) => println!("Command failed because it is a {}", error_msg),
    }
}
