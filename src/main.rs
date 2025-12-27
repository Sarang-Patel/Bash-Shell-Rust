use std::fs::File;
use std::io::{self, Write};
use std::collections::HashSet;
use std::{env};
use std::path::{Path, PathBuf};
use is_executable::IsExecutable;
use std::process::{Command, Stdio};
use std::fs::OpenOptions;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result, completion::Completer};

// fn read_input(prompt: &str) -> String {
//     let mut input  = String::new();
//     let mut stdout = io::stdout();
//     print!("{prompt} ");
//     stdout.flush().unwrap();
//     io::stdin().read_line(&mut input).expect("Failed to read line");
//     input.trim().to_string()
// }

struct MyHelper {
    builtins: Vec<String>
}

impl rustyline::Helper for MyHelper {}

impl Completer for MyHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let mut result : Vec<String> = Vec::new();

        for word in self.builtins.iter() {
            if word.starts_with(&line[..pos]) {
                result.push(word.clone());
            }
        }

        Ok((0, result))
    }
}

impl rustyline::hint::Hinter for MyHelper { 
    type Hint = &'static str;
}

impl rustyline::highlight::Highlighter for MyHelper {}
impl rustyline::validate::Validator for MyHelper {}

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

fn open_out_file(dest: &str, append : bool) -> std::io::Result<File> {
    let mut opts = OpenOptions::new();

    opts.write(true).create(true);

    if append {
        opts.append(true);
    }else{
        opts.truncate(true);
    }

    opts.open(dest)
}

fn main() -> Result<()> {

    let helper = MyHelper {
        builtins: vec![
            "echo".to_string(),
            "exit".to_string(),
            "pwd".to_string(),
            "cd".to_string(),
            "type".to_string(),
        ],
    };

    let mut rl : Editor<MyHelper, _> = Editor::new()?;
    rl.set_helper(Some(helper));
    // #[cfg(feature = "with-file-history")]
    // if rl.load_history("history.txt").is_err() {
    //     println!("No previous history.");
    // }

    let separator = if cfg!(windows) { ";" } else { ":" };
    let builtin: HashSet<String> = ["exit", "echo", "type", "pwd", "cd"].iter().map(|s| s.to_string()).collect();
    
    loop {
        let path_var = env::var("PATH").unwrap_or_default();
        let input = match rl.readline("$ ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                line
            },
            Err(ReadlineError::Interrupted) => {
                continue
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        };

        let mut tokens = tokenize_input(input);

        let write_redirection_symbol1 = ">".to_string();
        let write_redirection_symbol2 = "1>".to_string();
        let err_redirection_symbol = "2>".to_string();
        let append_redirection_symbol1 = ">>".to_string();
        let append_redirection_symbol2 = "1>>".to_string();
        let err_append_symbol = "2>>".to_string();

        let redirect_index = tokens.iter().position(|r|
            r == &write_redirection_symbol1 ||
            r == &write_redirection_symbol2 ||
            r == &err_redirection_symbol ||
            r == &append_redirection_symbol1 ||
            r == &append_redirection_symbol2 ||
            r == &err_append_symbol
        );

        let mut redirection_part = Vec::new(); 

        let mut out_append : bool = false;
        let mut err_append : bool = false;

        match redirect_index {
            Some(index) => {
                redirection_part = tokens.split_off(index);
            },
            None => {},
        }

        let mut redirection_target: Option<&str> = None;
        let mut redirection_error_target: Option<&str> = None;

        if redirection_part.len() > 1 {
            for (i, c) in redirection_part.iter().enumerate() {
                match c.as_str() {
                    ">" | "1>" => {
                        redirection_target = redirection_part.get(i + 1).map(|s| s.as_str());
                        out_append = false;
                    }
                     ">>" | "1>>" => {
                        redirection_target = redirection_part.get(i + 1).map(|s| s.as_str());
                        out_append = true;
                    }
                    "2>" => {
                        redirection_error_target = redirection_part.get(i + 1).map(|s| s.as_str());
                        err_append = false;
                    }
                    "2>>" => {
                        redirection_error_target = redirection_part.get(i + 1).map(|s| s.as_str());
                        err_append = true;
                    }
                    _ => {}
                }
            }
        }

        let stdout_file = redirection_target
            .map(|dest| open_out_file(dest, out_append).expect("failed to open stdout file"));

        let stderr_file = redirection_error_target
            .map(|dest| open_out_file(dest, err_append).expect("failed to open stderr file"));


        let stdout = match &stdout_file {
            Some(file) => Stdio::from(file.try_clone().unwrap()),
            None => Stdio::inherit(),
        };

        let stderr = match &stderr_file {
            Some(file) => Stdio::from(file.try_clone().unwrap()),
            None => Stdio::inherit(),
        };


        let mut builtin_out: Box<dyn Write> = match &stdout_file {
            Some(file) => Box::new(file.try_clone().unwrap()),
            None => Box::new(io::stdout()),
        };

        let mut builtin_err: Box<dyn Write> = match &stderr_file {
            Some(file) => Box::new(file.try_clone().unwrap()),
            None => Box::new(io::stderr()),
        };



        let cmd: String = tokens.get(0).cloned().unwrap_or_default();
        let args = tokens.into_iter().skip(1).collect::<Vec<_>>();


        if builtin.contains(&cmd) {
            match cmd.as_str() {
                "exit" => break,
                "echo" => {
                    if args.first().map(|s| s == "-z").unwrap_or(false) {
                        builtin_err.write_all(b"echo: invalid option '-z'\n").unwrap();
                        continue;
                    }

                    let mut text = args.join(" ");
                    text.push('\n');
                    builtin_out.write_all(text.as_bytes()).unwrap();
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
            if let Some(_full_path) = path_var.split(separator).map(|dir| Path::new(dir).join(cmd.to_string()))
            .find(|p| p.exists() && p.is_executable()) {

                let mut output = Command::new(cmd).args(&args).stdout(stdout).stderr(stderr).spawn().expect("Failed to execute process");

                output.wait().expect("failed to finish process");

            }else{
                println!("{cmd}: command not found");
            }
        }

    }

    // #[cfg(feature = "with-file-history")]
    // rl.save_history("history.txt");

    Ok(())

}
