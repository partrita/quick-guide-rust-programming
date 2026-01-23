use std::io;

#[derive(Debug, PartialEq)]
pub enum Command {
    Help,
    Quit,
    Execute(String),
    Run,
}

fn user_input() -> Result<Command, String> {
    println!("input h/q/r/e: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.as_str().strip_suffix("\n") {
            Some("h") => return Ok(Command::Help),
            Some("q") => return Ok(Command::Quit),
            Some("r") => return Ok(Command::Run),
            Some("e") => return Ok(Command::Execute(format!("NOT IMPLEMENTED YET!"))),
            _ => return Err(format!("Wrong input: {input}")),
        },
        Err(error) => return Err(format!("Wrong input: {error}")),
    }
}

fn main() {
    let com: Command = user_input().expect("Wrong input");
    let p = String::new();

    assert_ne!(com, Command::Execute(p));
    match com {
        Command::Help => println!("show help message"),
        Command::Quit => return,
        Command::Run => println!("do something"),
        Command::Execute(path) => println!("execute {path}"),
    }
}
