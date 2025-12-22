#[allow(unused_imports)]
use std::io::{self, Write};

fn read_input(prompt: &str) -> String {
    let mut input  = String::new();
    let mut stdout = io::stdout();

    print!("{prompt} ");
    stdout.flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()

}

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    

    loop {
        let cmd = read_input("$");
        println!("{cmd}: command not found");
    }

}
