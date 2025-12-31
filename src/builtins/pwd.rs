use super::BuiltinContext;
use std::env;
use std::io::Write;

pub fn run(ctx: BuiltinContext) -> bool {
    match env::current_dir() {
        Ok(path) => {
            ctx.builtin_out
                .write_all(path.display().to_string().as_bytes())
                .unwrap();
        }
        Err(e) => {
            ctx.builtin_err
                .write_all(e.to_string().as_bytes())
                .unwrap();
        }
    }
    true
}
