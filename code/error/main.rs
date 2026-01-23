mod mycommand;
use crate::mycommand::*;

#[derive(Debug, PartialEq)]
pub enum SuperError {
    CommandError(MyError),
}

pub type Result<T, E = SuperError> = std::result::Result<T, E>;

impl From<MyError> for SuperError {
    fn from(err: MyError) -> Self {
        Self::CommandError(err)
    }
}

fn super_handle_command(cmd: &str) -> Result<usize> {
    Ok(mycommand::handle_command(cmd)?)
}

fn main() {
    let status: Result<usize, SuperError> = super_handle_command("bad");
    if status.is_ok() {
        println!("Everything is fine");
    } else {
        println!("I don't feel good:{:?}", status);
    }
}
