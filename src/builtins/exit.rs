use super::BuiltinContext;
use std::env;

pub fn run(mut ctx: BuiltinContext) -> bool {
    if let Ok(histfile) = env::var("HISTFILE") {
        ctx.history.write_all(&histfile);
    }
    false // signal exit
}
