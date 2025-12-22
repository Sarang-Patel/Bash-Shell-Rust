#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashSet;


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

        let builtin: HashSet<_> = ["exit", "echo", "type"].iter().cloned().collect();
        let mut parts = input.splitn(2, ' ');
        let cmd = parts.next().unwrap_or("").trim();
        let arg = parts.next().unwrap_or("").trim();


        match cmd {
            "exit" => break,
            "echo" => println!("{arg}"),
            "type" => {
                if builtin.contains(arg) {
                    println!("{arg} is a shell builtin");
                }else {
                    println!("{arg}: not found");
                }
            }
            _ => println!("{cmd}: command not found"),
        }

        
    }

}
