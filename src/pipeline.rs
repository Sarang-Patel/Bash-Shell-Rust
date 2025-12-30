use std::{io::Write, process::{Command, Stdio}};


pub struct CommandSpec {
    pub cmd: String,
    pub args: Vec<String>,
    pub stdout: Stdio,
    pub stderr: Stdio,
    pub builtin_out: Box<dyn Write>,
    pub builtin_err: Box<dyn Write>,
}


pub fn execute_pipeline(commands: Vec<CommandSpec>) {
    let mut previous_stdout = None;
    let mut children = Vec::new();

    for (i, cmd) in commands.iter().enumerate() {
        let mut command = Command::new(&cmd.cmd);
        command.args(&cmd.args);

        // stdin
        if let Some(stdin) = previous_stdout {
            command.stdin(stdin);
        }

        // stdout
        if i < commands.len() - 1 {
            command.stdout(Stdio::piped());
        }

        let mut child = command.spawn().expect("failed to spawn");

        previous_stdout = child.stdout.take().map(Stdio::from);
        children.push(child);
    }

    // wait for all
    for mut child in children {
        let _ = child.wait();
    }
}
