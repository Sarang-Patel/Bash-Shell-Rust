use super::BuiltinContext;
use std::io::Write;
use std::collections::HashSet;

use crate::builtins::{exit, echo, pwd, rtype, cd, history, help};

static HELP_TEXT: &str = "
echo [args..]
cd [dir]
exit
history [n] or history -arw [filename]
help [name..]
pwd
type [name..]\n
";

fn builtins() -> HashSet<&'static str> {
    [
        "exit",
        "echo",
        "pwd",
        "type",
        "cd",
        "history",
        "help",
    ]
    .into_iter()
    .collect()
}

pub fn run(mut ctx: BuiltinContext) -> bool {
    let builtins = builtins();

    // No arguments → print general help
    if ctx.args.is_empty() {
        let _ = ctx.builtin_out.write_all(HELP_TEXT.as_bytes());
        return true;
    }

    // help <name>
    let name = &ctx.args[0];

    if !builtins.contains(name.as_str()) {
        let msg = format!(
            "help: no help topics match `{}`\n",
            name
        );
        let _ = ctx.builtin_err.write_all(msg.as_bytes());
        return true;
    }

    // Dispatch to builtin info()
    let info = match name.as_str() {
        "exit" => exit::info(),
        "echo" => echo::info(),
        "pwd" => pwd::info(),
        "type" => rtype::info(),
        "cd" => cd::info(),
        "history" => history::info(),
        "help" => help::info(),
        _ => unreachable!(),
    };

    let output = format!("\n{}\n", info);

    let _ = ctx.builtin_out.write_all(output.as_bytes());
    true
}


pub fn info() -> &'static str {
    "help [name..]\n\
    Display information about builtin commands.\n"
}
