use super::BuiltinContext;
use std::env;

pub fn run(mut ctx: BuiltinContext) -> bool {
    
    false // signal exit
}


pub fn info() -> &'static str {
    "exit\n\
    Exit the shell.\n"
}
