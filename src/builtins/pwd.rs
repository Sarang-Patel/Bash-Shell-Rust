use super::BuiltinContext;
use std::env;

pub fn run(_ctx: BuiltinContext) -> bool {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("{}", e),
    }
    true
}
