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

        let builtin = ["exit", "echo", "type"];
        let mut parts = input.splitn(2, ' ');
        let cmd = parts.next().unwrap_or("");

        if cmd == "exit" {
            break;
        }else if cmd == "echo" {
            let arg = parts.next().unwrap_or("");

            println!("{arg}");
        }else if cmd == "type" {
            let arg = parts.next().unwrap_or("");
            
            if builtin.contains(&arg) {
                println!("{arg} is a shell builtin");
            }else {
                println!("{arg}: not found");
            }
        }
        else {
            println!("{cmd}: command not found");
        }
        
    }

}
