use std::{
    cell::RefCell,
    collections::HashSet,
    env,
    io::{self, Write},
    path::Path,
};

use rustyline::{
    completion::Completer,
    Context, Helper,
};

use is_executable::IsExecutable;
use crate::tokenizer::longest_common_prefix;

pub struct MyHelper {
    builtins: HashSet<String>,
    last_prefix: RefCell<String>,
    last_matches: RefCell<Vec<String>>,
    tab_count: RefCell<u8>,         
}

impl MyHelper {
    pub fn new(builtins: HashSet<String>) -> Self {
        Self {
            builtins,
            last_prefix: RefCell::new(String::new()),
            last_matches: RefCell::new(Vec::new()),
            tab_count: RefCell::new(0),
        }
    }

    fn find_matches(&self, prefix: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();

        
        for word in &self.builtins {
            if word.starts_with(prefix) && seen.insert(word.to_string()) {
                result.push(word.clone());
            }
        }

        
        let path_var = env::var("PATH").unwrap_or_default();
        let separator = if cfg!(windows) { ";" } else { ":" };

        for dir in path_var.split(separator) {
            if let Ok(entries) = Path::new(dir).read_dir() {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file()
                        && path.is_executable()
                        && let Some(name) = path.file_name().and_then(|n| n.to_str())
                        && name.starts_with(prefix)
                    {
                        if seen.insert(name.to_string()) {
                            result.push(name.to_string());
                        }
                    }
                }
            }
        }

        result.sort();
        result
    }
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
        let prefix = &line[..pos];

        let mut last_prefix = self.last_prefix.borrow_mut();
        let mut last_matches = self.last_matches.borrow_mut();
        let mut tab_count = self.tab_count.borrow_mut();

        
        let matches = if *last_prefix == prefix {
            last_matches.clone()
        } else {
            let new_matches = self.find_matches(prefix);
            *last_prefix = prefix.to_string();
            *last_matches = new_matches.clone();
            *tab_count = 0; 
            new_matches
        };

        if matches.len() == 0 {
            return Ok((0, vec![]));
        }

        if matches.len() == 1 {
            return Ok((0, vec![format!("{} ", matches[0])]));
        }

        let lcf = longest_common_prefix(&matches);

        if lcf != prefix {
            return Ok((0, vec![format!("{}", lcf)]));
        }

        if matches.len() > 1 {
            if *tab_count < 2 {*tab_count += 1};

            if *tab_count == 1 {
                
                print!("\x07");
                return Ok((0, Vec::new()));
            }

            if *tab_count == 2 {
                println!();
                println!("{}", matches.join("  "));

                
                print!("$ {}", prefix);
                io::stdout().flush().unwrap();

                return Ok((0, Vec::new()));
            }
        }

        Ok((0, Vec::new()))
    }
}

impl rustyline::hint::Hinter for MyHelper { 
    type Hint = &'static str;
}
impl rustyline::highlight::Highlighter for MyHelper {}
impl rustyline::validate::Validator for MyHelper {}