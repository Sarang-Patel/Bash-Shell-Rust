use std::{env, path::PathBuf};

use super::BuiltinContext;


pub fn run(ctx: BuiltinContext) -> bool {
    let dir = ctx.args.get(0);

    let dir = if let Some(arg) = dir {
        if arg == "~"  {
            env::home_dir()
        }else {
            Some(PathBuf::from(arg))
        }
    }else{
        None
    };

    if let Some(path) = dir {
    
        if let Err(_) = env::set_current_dir(&path) {
        writeln!(ctx.builtin_err, "cd: {}: No such file or directory", path.display());
        }
    } else {
        writeln!(ctx.builtin_err, "{}", "cd: missing operand");
    }
    
    true
}

pub fn info() -> &'static str {
    "cd [dir]\n\
    Change the current working directory.\n"
}
