#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashSet;
use std::{env};
use std::path::Path;
use is_executable::IsExecutable;


fn read_input(prompt: &str) -> String {
    let mut input  = String::new();
    let mut stdout = io::stdout();

    print!("{prompt} ");
    stdout.flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()

}

fn main() {
        
    let path_var = env::var("PATH").expect("PATH not set");

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
                    for dir in path_var.split(":") {
                        let full_path = Path::new(dir).join(arg);
                        let full_path_str = full_path.to_str().unwrap();
 
                        if full_path.exists() && full_path.is_executable() {
                            println!("{arg} is {full_path_str}");
                            break;
                        }

                    }
                    println!("{arg}: not found");
                }
            }
            _ => println!("{cmd}: command not found"),
        }

        
    }

}
