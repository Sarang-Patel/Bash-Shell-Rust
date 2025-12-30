use super::BuiltinContext;

pub fn run(ctx: BuiltinContext) -> bool {
    let history = ctx.history;

    if ctx.args.is_empty() || ctx.args[0].parse::<usize>().is_ok() {
        let n = ctx.args.get(0).and_then(|s| s.parse().ok());
        history.print(n);
        return true;
    }

    match ctx.args[0].as_str() {
        "-r" => {
            if let Some(path) = ctx.args.get(1) {
                history.read_from_file(path);
            }
        }
        "-w" => {
            if let Some(path) = ctx.args.get(1) {
                history.write_all(path);
            }
        }
        "-a" => {
            if let Some(path) = ctx.args.get(1) {
                history.append_new(path);
            }
        }
        _ => {
            eprintln!("invalid flag");
        }
    }

    true
}
