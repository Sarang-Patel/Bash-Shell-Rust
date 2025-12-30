use std::io::Write;
use std::collections::HashSet;

use crate::history_helper::History;

pub struct BuiltinContext<'a> {
    pub args: &'a [String],
    pub builtin_out: &'a mut dyn Write,
    pub builtin_err: &'a mut dyn Write,
    pub builtin_set: &'a HashSet<String>,
    pub history: &'a mut History,
}

pub mod exit;
pub mod echo;
pub mod pwd;
pub mod rtype;
pub mod cd;
pub mod history;

pub fn run(cmd: &str, ctx: BuiltinContext) -> bool {
    match cmd {
        "exit" => exit::run(ctx),
        "echo" => echo::run(ctx),
        "pwd" => pwd::run(ctx),
        "cd" => cd::run(ctx),
        "type" => rtype::run(ctx),
        "history" => history::run(ctx),
        _ => false,
    }
}
