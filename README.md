[![progress-banner](https://backend.codecrafters.io/progress/shell/e3a2d1b8-ac02-4e67-9aab-df8569a33be2)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)
This shell was built as part of the CodeCrafters “Build Your Own Shell” challenge.


# Rust Shell

A minimal Unix-like shell implemented in Rust, designed to be small, fast, and easy to understand while supporting core interactive shell features.

This project focuses on correctness, clarity, and idiomatic Rust rather than full POSIX compatibility.

---

## Features

### Built-in Commands
The shell implements the following builtins internally (no external processes required):

- `echo [args..]`  
  Print arguments to standard output.

- `cd [dir]`  
  Change the current working directory.

- `pwd`  
  Print the current working directory.

- `exit`  
  Exit the shell.

- `type [name..]`  
  Display whether a command is a builtin or an external executable.

- `history [n]`  
  Display command history.

- `history -a|-r|-w [filename]`  
  Append, read, or write history to a file.

- `help [name..]`  
  Display help for built-in commands.

---

### Tab Completion
- Bash-like tab completion for commands and paths
- Rings the terminal bell when completion is ambiguous
- Displays all possible completions on repeated `<Tab>`
- Automatically completes when a single match is available
- Works seamlessly with both builtins and external commands

---

### I/O Redirection
Supports standard shell redirections with Bash-compatible behavior:

- `> | 1>`   Redirect standard output (overwrite)
- `>> | 1>>`  Redirect standard output (append)
- `2>`  Redirect standard error (overwrite)
- `2>>` Redirect standard error (append)
- Combined redirections in a single command

---

### Pipelines
- Full pipeline support using `|`
- Compatible with **builtin-to-builtin**, **builtin-to-external**, and **external-to-builtin** pipelines
- Correct stream handling across pipeline stages
- Behavior closely mirrors Bash pipeline semantics

---

### Command Execution
- Executes external commands using the system `PATH`
- Differentiates builtins from external programs
- Supports argument passing, redirections, and pipelines
- Proper standard input/output/error wiring

---

### History Management
- Maintains an in-memory command history
- Supports reading and writing history to files
- Compatible with common shell history workflows

---

### Help System
- Builtin-aware `help` command
- Lists all available builtins when called without arguments
- Displays per-command usage and descriptions
- Bash-like error handling for unknown help topics

---

## Versioning

The project follows semantic versioning and is currently in **initial development**:

- **v0.1.0** – First usable release

---

## Goals

- Learn how real shells work internally
- Explore process execution, I/O, and environment handling
- Practice clean Rust architecture for systems programming

---

## Non-Goals

- Full POSIX shell compliance
- Job control or advanced scripting
- Feature parity with Bash or Zsh

---

## Building and Running

Rust version : 1.91

```bash
cargo build
cargo run
```


