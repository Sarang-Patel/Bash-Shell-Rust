use std::{env, path::Path, io::Write};
use is_executable::IsExecutable;

use super::BuiltinContext;

pub fn run(mut ctx: BuiltinContext) -> bool {
    let path_var = env::var("PATH").unwrap_or_default();

    if ctx.args.is_empty() {
        let _ = ctx
            .builtin_err
            .write_all(b"type: missing operand\n");
        return true;
    }

    for arg in ctx.args {
        if ctx.builtin_set.contains(arg) {
            let _ = ctx
                .builtin_out
                .write_all(format!("{arg} is a shell builtin\n").as_bytes());
        } else {
            if let Some(full_path) = path_var
                .split(':')
                .map(|dir| Path::new(dir).join(arg))
                .find(|p| p.exists() && p.is_executable())
            {
                let _ = ctx.builtin_out.write_all(
                    format!("{arg} is {}\n", full_path.display()).as_bytes(),
                );
            } else {
                let _ = ctx
                    .builtin_err
                    .write_all(format!("{arg}: not found\n").as_bytes());
            }
        }
    }

    true
}
