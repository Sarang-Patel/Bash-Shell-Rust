use std::{collections::HashSet, io::{Read, Write}, process::{Command, Stdio}};

use os_pipe::{PipeReader, pipe};

use crate::{builtins::{self, BuiltinContext}, history_helper::History};


pub struct CommandSpec {
    pub cmd: String,
    pub args: Vec<String>,
    pub stdin: Stdio,
    pub stdout: Stdio,
    pub stderr: Stdio,
    pub builtin_in: Box<dyn Read>,
    pub builtin_out: Box<dyn Write>,
    pub builtin_err: Box<dyn Write>,
    pub isbuiltin: bool,
}


pub fn execute_pipeline(commands: Vec<CommandSpec>, builtins: &HashSet<String>, history: &mut History) {
    let mut prev_reader: Option<PipeReader> = None;
    let mut children = Vec::new();
    let total = commands.len();

    for (i, mut cmd) in commands.into_iter().enumerate() {
        let is_last = i + 1 == total;

        // Create pipe for next stage
        let (next_reader, next_writer) = if !is_last {
            let (r, w) = pipe().unwrap();
            (Some(r), Some(w))
        } else {
            (None, None)
        };

        if cmd.isbuiltin {
            // ----- builtin stdin -----
            let mut builtin_in: Box<dyn Read> = match prev_reader.take() {
                Some(r) => Box::new(r),
                None => Box::new(std::io::stdin()),
            };

            // ----- builtin stdout -----
            let mut builtin_out: Box<dyn Write> = match next_writer {
                Some(w) => Box::new(w),
                None => Box::new(std::io::stdout()),
            };

            let mut builtin_err: Box<dyn Write> =
                Box::new(std::io::stderr());

            let ctx = BuiltinContext {
                    args: &cmd.args,
                    builtin_out: &mut builtin_out,
                    builtin_err: &mut builtin_err,
                    builtin_set: builtins,
                    history: history,
                };

            builtins::run(&cmd.cmd, ctx);


        } else {
            // ----- external stdin -----
            let stdin = match prev_reader.take() {
                Some(r) => Stdio::from(r),
                None => cmd.stdin,
            };

            // ----- external stdout -----
            let stdout = match next_writer {
                Some(w) => Stdio::from(w),
                None => cmd.stdout,
            };

            let child = Command::new(&cmd.cmd)
                .args(&cmd.args)
                .stdin(stdin)
                .stdout(stdout)
                .stderr(cmd.stderr)
                .spawn()
                .expect("spawn failed");

            children.push(child);
        }

        prev_reader = next_reader;
    }

    for mut child in children {
        let _ = child.wait();
    }
}