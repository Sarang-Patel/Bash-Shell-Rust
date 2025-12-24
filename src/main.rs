#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashSet;
use std::{env};
use std::path::{Path, PathBuf};
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

fn tokenize_input(input : String) -> Vec<String> {
    let mut tokens : Vec<String> = Vec::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut backslash = false;
    let mut curr = String::new();

    for c in input.chars() {
        if backslash {
            curr.push(c);
            backslash = false;
            continue;
        }

        match c {
            '\\' if !in_single_quotes && !in_double_quotes => {
                backslash = true;
            },
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes;
            },
            ' ' if !in_single_quotes && !in_double_quotes => {
                if !curr.is_empty() {
                    tokens.push(curr.clone());
                    curr.clear();
                }
            },
            '\"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes;
            },
            _ => curr.push(c),
        }
    } 

    if !curr.is_empty() {
        tokens.push(curr.clone());
    }

    tokens
}

fn main() {
        
    let separator = if cfg!(windows) { ";" } else { ":" };
    let builtin: HashSet<String> = ["exit", "echo", "type", "pwd", "cd"].iter().map(|s| s.to_string()).collect();
    
    loop {
        let path_var = env::var("PATH").unwrap_or_default();
        let input = read_input("$");

        let tokens = tokenize_input(input);

        let cmd: String = tokens.get(0).cloned().unwrap_or_default();
        // let mut args: Vec<String> = parts.iter().skip(1).map(|s| s.to_string()).collect();

        let args = tokens.into_iter().skip(1).collect::<Vec<_>>();

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
                },
                "pwd" => {
                    match env::current_dir() {
                        Ok(path) => {
                            println!("{}", path.display());
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                },
                "cd" => {
                    let dir = args.get(0);

                    let dir = if let Some(arg) = dir {
                        if arg == "~"  {
                            env::home_dir()
                        }else {
                            Some(PathBuf::from(arg))
                        }
                    }else{
                        None
                    };

                    if let Some(path) = dir {
                        
                        if let Err(_) = env::set_current_dir(&path) {
                            println!("cd: {}: No such file or directory", path.display());
                        }
                    } else {
                        println!("cd: missing operand");
                    }
                },
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
                println!("{cmd}: command not found");
            }
        }

    }

}
