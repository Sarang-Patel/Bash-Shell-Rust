use std::{env, path::Path};
use is_executable::IsExecutable;

use super::BuiltinContext;


pub fn run(ctx: BuiltinContext) -> bool {
    let path_var = env::var("PATH").unwrap_or_default();
    if ctx.args.is_empty() {
        println!("type: missing operand");
        return true;
    }
    for arg in ctx.args {
        if ctx.builtin_set.contains(arg) {
            println!("{arg} is a shell builtin");
        }else {
            if let Some(full_path) = path_var.split(":")
            .map(|dir| Path::new(dir).join(arg))
            .find(|p| p.exists() && p.is_executable()) {
                println!("{arg} is {}", full_path.display());
            }else{
                println!("{arg}: not found");
            }
        }
    }

    true
}