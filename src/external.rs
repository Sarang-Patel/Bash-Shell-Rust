use std::{
    env,
    path::Path,
    process::{Command, Stdio},
};

use is_executable::IsExecutable;


pub fn run(cmd: &str, args: &[String], stdout: Stdio, stderr: Stdio) {
    let path_var = env::var("PATH").unwrap_or_default();
    let separator = if cfg!(windows) { ";" } else { ":" };

    if path_var
        .split(separator)
        .map(|dir| Path::new(dir).join(cmd))
        .any(|p| p.exists() && p.is_executable())
    {
        let mut child = Command::new(cmd)
            .args(args)
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .expect("Failed to execute process");

        let _ = child.wait();
    } else {
        println!("{cmd}: command not found");
    }
}
