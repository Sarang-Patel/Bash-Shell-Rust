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
        let input = read_input("$");

        let mut parts = input.splitn(2, ' ');
        let cmd = parts.next().unwrap_or("");
        let arg = parts.next().unwrap_or("");

        if cmd == "exit" {
            break;
        }else if cmd == "echo" {
            println!("{arg}");
        }else {
            println!("{cmd}: command not found");
        }
        
    }

}
