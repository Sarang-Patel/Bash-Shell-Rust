#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    let mut cmd  = String::new();

    print!("$ ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut cmd).expect("failed to read line");

    let cmd = cmd.trim();

    println!("{cmd}: command not found");
}
