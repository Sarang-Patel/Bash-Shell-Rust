#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashSet;
use std::{env};
use std::path::Path;
use is_executable::IsExecutable;
use std::process::Command;


fn read_input(prompt: &str) -> String {
    let mut input  = String::new();
    let mut stdout = io::stdout();

    print!("{prompt} ");
    stdout.flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()

}

fn main() {
        
    let separator = if cfg!(windows) { ";" } else { ":" };
    let builtin: HashSet<String> = ["exit", "echo", "type"].iter().map(|s| s.to_string()).collect();
    
    loop {
        let path_var = env::var("PATH").unwrap_or_default();
        let input = read_input("$");

        let parts: Vec<&str> = input.split_whitespace().collect();

        let cmd: String = parts.get(0).unwrap_or(&"").to_string();
        let args: Vec<String> = parts.iter().skip(1).map(|s| s.to_string()).collect();

        if builtin.contains(&cmd) {
            match cmd.as_str() {
                "exit" => break,
                "echo" => println!("{}", args.join(" ")),
                "type" => {
                    if args.is_empty() {
                        println!("type: missing operand");
                        continue;
                    }

                    for arg in &args {
                    if builtin.contains(arg) {
                        println!("{arg} is a shell builtin");
                    }else {
                        if let Some(full_path) = path_var.split(separator)
                        .map(|dir| Path::new(dir).join(arg))
                        .find(|p| p.exists() && p.is_executable()) {
                            println!("{arg} is {}", full_path.display());
                        }else{
                            println!("{arg}: not found");
                        }
                    }}
                }
                _ => println!("{cmd}: command not found"),
            }
        }else {
            if let Some(_full_path) = path_var.split(separator).map(|dir| Path::new(dir).join(cmd.to_string()))
            .find(|p| p.exists() && p.is_executable()) {
                let output = Command::new(cmd).args(&args).output().expect("Failed to execute process");

                let stdout = str::from_utf8(&output.stdout).unwrap();
                let stderr = str::from_utf8(&output.stderr).unwrap();

                if !stdout.is_empty() {
                    print!("{}", stdout);
                }

                if !stderr.is_empty() {
                    eprint!("{}", stderr);
                }

            }else{
                println!("{cmd}: not found.");
            }
        }

    }

}
