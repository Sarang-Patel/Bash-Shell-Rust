use std::fs::{self, OpenOptions};
use std::io::Write;

use crate::builtins::BuiltinContext;

pub struct History {
    history: Vec<String>,
    new_commands: Vec<String>,
}

impl History {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            new_commands: Vec::new(),
        }
    }

    pub fn read_from_file(&mut self, path: &str) {
        if let Ok(contents) = fs::read_to_string(path) {
            for line in contents.lines() {
                self.history.push(line.to_string());
            }
        }
    }

    pub fn add_command(&mut self, cmd: String) {
        self.history.push(cmd.clone());
        self.new_commands.push(cmd);
    }

    pub fn print<W: Write>(&self, mut out: W, n: Option<usize>) {
        let total = self.history.len();
        let n = n.unwrap_or(total);
        let start = total.saturating_sub(n);

        for (i, line) in self.history[start..].iter().enumerate() {
            let _ = writeln!(out, "\t{}  {}", start + i + 1, line);
        }
    }

    pub fn write_all(&self, path: &str) {
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
        {
            for line in &self.history {
                let _ = writeln!(file, "{}", line);
            }
        }
    }

    pub fn append_new(&mut self, path: &str) {
        if let Ok(mut file) = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
        {
            for line in &self.new_commands {
                let _ = writeln!(file, "{}", line);
            }
        }

        self.new_commands.clear();
    }
}
