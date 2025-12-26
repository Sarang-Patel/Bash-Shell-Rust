use std::fs::File;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashSet;
use std::{env};
use std::path::{Path, PathBuf};
use is_executable::IsExecutable;
use std::process::{Command, Stdio};


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
            if in_double_quotes && c != '\\' && c != '"' {
                curr.push('\\');
            }
            curr.push(c);
            backslash = false;
            continue;
        }

        match c {
            '\\' if !in_single_quotes => {
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
            '\"' if !in_single_quotes && !backslash => {
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

        let mut tokens = tokenize_input(input);

        let redirection_symbol1 = ">".to_string();
        let redirection_symbol2 = "1>".to_string();

        let redirect_index = tokens.iter().position(|r| r == &redirection_symbol1 || r == &redirection_symbol2);
        let mut redirection_part = Vec::new(); 

        match redirect_index {
            Some(index) => {
                redirection_part = tokens.split_off(index);
            },
            None => {},
        }

        let mut redirection_target = "".to_string();

        if redirection_part.len() > 1 {
            redirection_target = redirection_part.split_off(1)[0].to_string();
        }

        let cmd: String = tokens.get(0).cloned().unwrap_or_default();
        let args = tokens.into_iter().skip(1).collect::<Vec<_>>();


        if builtin.contains(&cmd) {
            match cmd.as_str() {
                "exit" => break,
                "echo" => if redirection_target != "".to_string() {
                    let mut file = File::create(redirection_target).expect("failed to create file");
                    file.write_all(args.join(" ").as_bytes()).expect("failed to write");
                }else{
                    println!("{}", args.join(" "))
                },
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
            if let Some(full_path) = path_var.split(separator).map(|dir| Path::new(dir).join(cmd.to_string()))
            .find(|p| p.exists() && p.is_executable()) {

                if redirection_target != "".to_string() {

                    let file = File::create(redirection_target).expect("failed to create file");
                    let mut running_cmd = Command::new(full_path).args(&args).stdout(file).stderr(Stdio::inherit()).spawn().expect("Failed to execute process");

                    running_cmd.wait().expect("failed to finish");

                }else{

                    let mut output = Command::new(full_path).args(&args).stdout(Stdio::inherit()).stderr(Stdio::inherit()).spawn().unwrap();

                    output.wait().expect("failed to finish process");
                }


            }else{
                println!("{cmd}: command not found");
            }
        }

    }

}
