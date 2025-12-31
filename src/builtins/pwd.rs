use super::BuiltinContext;
use std::env;
use std::io::Write;

pub fn run(ctx: BuiltinContext) -> bool {
    match env::current_dir() {
        Ok(path) => {
            let _ = writeln!(
                ctx.builtin_out,
                "{}",
                path.display()
            );
        }
        Err(e) => {
            let _ = writeln!(
                ctx.builtin_err,
                "{}",
                e
            );
        }
    }
    true
}


pub fn info() -> &'static str {
    "pwd\n\
    Print the current working directory.\n"
}
