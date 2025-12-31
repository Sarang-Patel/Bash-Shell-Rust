#![allow(unused)]

mod helper;
mod history_helper;
mod tokenizer;
mod pipeline;
mod builtins;
mod external;

use std::cell::RefCell;
use std::fs::{self, File};
use std::io::{self, Write};
use std::collections::HashSet;
use std::{env};
use std::path::{Path, PathBuf};
use is_executable::IsExecutable;
use rustyline::CompletionType;
use rustyline::config::Configurer;
use std::process::{Command, Stdio};
use std::fs::OpenOptions;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result, completion::Completer};
use helper::MyHelper;
use whoami;

use crate::builtins::BuiltinContext;
use crate::pipeline::CommandSpec;
use history_helper::History;


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

fn record_command(cmd: &str, history: &mut Vec<String>, new_commands: &mut Vec<String>) {
    history.push(cmd.to_string());
    new_commands.push(cmd.to_string());
}



fn main() -> Result<()> {

    println!("\n\nMyShell 0.1.0\nRust-based interactive shell\nType 'help' to see available commands.");
    println!("\n\nUser: {}", whoami::username());
    println!("Device: {}\n", whoami::devicename());
    let separator = if cfg!(windows) { ";" } else { ":" };
    let builtin: HashSet<String> = ["exit", "echo", "type", "pwd", "cd", "history", "help"].iter().map(|s| s.to_string()).collect();

    let helper = MyHelper::new(builtin.clone());

    let mut history = History::new();

    let mut new_commands: Vec<String> = Vec::new();

    if let Ok(histfile) = env::var("HISTFILE") {
        history.read_from_file(&histfile);
    }


    let mut rl : Editor<MyHelper, _> = Editor::new()?;
    rl.set_helper(Some(helper));

    rl.set_completion_type(CompletionType::List);

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

        history.add_command(input.clone());

        let mut tokens = tokenizer::tokenize_input(input);
        let split_commands = tokenizer::split_commands(tokens);

        let mut all_commands = Vec::new();

        for command in split_commands {
            all_commands.push(command_specfn(command, &builtin));
        }

        if all_commands.len() == 1 {
            let CommandSpec {
                cmd,
                args,
                stdin,
                stdout,
                stderr,
                mut builtin_in,
                mut builtin_out,
                mut builtin_err,
                isbuiltin,
            } = all_commands.remove(0);

            if builtin.contains(&cmd) {
            
                let ctx = BuiltinContext {
                    args: &args,
                    builtin_out: &mut builtin_out,
                    builtin_err: &mut builtin_err,
                    builtin_set: &builtin,
                    history: &mut history,
                };

                if !builtins::run(&cmd, ctx) {
                    break;
                }
            
            }else {
                external::run(&cmd, &args, stdout, stderr);
            }


        }else{
            pipeline::execute_pipeline(all_commands, &builtin, &mut history);

        }


        
    }

    if let Ok(histfile) = env::var("HISTFILE") {
        history.write_all(&histfile);
    }
    Ok(())

}


fn command_specfn(mut command_tokens: Vec<String>, builtin: &HashSet<String>) -> CommandSpec {

    let redirect_index = command_tokens.iter().position(|r|
            r == &">" || r == &"1>" || r == &"2>" || r == &">>" || r == &"1>>" || r == &"2>>"
        );

        let mut redirection_part = Vec::new(); 

        let mut out_append : bool = false;
        let mut err_append : bool = false;

        match redirect_index {
            Some(index) => {
                redirection_part = command_tokens.split_off(index);
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
        
        let stdin = Stdio::inherit();
        let builtin_in = Box::new(io::stdin());

        let cmd: String = command_tokens.get(0).cloned().unwrap_or_default();
        let args = command_tokens.into_iter().skip(1).collect::<Vec<_>>();
        let isbuiltin = builtin.contains(&cmd);
        CommandSpec { cmd, args, stdin, stdout, stderr, builtin_in, builtin_out, builtin_err, isbuiltin }
}

