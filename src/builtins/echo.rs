use super::BuiltinContext;
use std::io::Write;

pub fn run(ctx: BuiltinContext) -> bool {
    if ctx.args.first().map(|s| s == "-z").unwrap_or(false) {
        let _ = ctx.builtin_err.write_all(b"echo: invalid option '-z'\n");
        return true;
    }

    let mut text = ctx.args.join(" ");
    text.push('\n');
    let _ = ctx.builtin_out.write_all(text.as_bytes());

    true
}
