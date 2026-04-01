# shell

An interactive mini shell written in Rust. It provides a colored prompt (current directory + git branch when available), in-terminal line editing, command history navigation, and a small set of built-in commands.

## Build & run

```bash
cargo run
```

To build a release binary:

```bash
cargo build --release
```

## Features

- Prompt shows the current directory (uses `~` for your home) and the current git branch when inside a repository.
- Line editing in the terminal (insert, delete, left/right, home/end).
- History navigation with ↑/↓.
- Multiline input using a trailing `\`.
- Basic quoting and escaping:
  - Single quotes `'...'` and double quotes `"..."` keep spaces inside arguments.
  - Backslash `\` escapes the next character.

## Built-in commands

This shell only executes built-ins (it does not launch external programs).

- `echo [-n]... [TEXT...]`
  - Supports `-n`, `-nnn`, … to suppress the trailing newline.
- `pwd`
- `clear`
- `exit [CODE]`
- `cd [DIR]`
  - `cd` or `cd ~` goes to your home directory.
  - `cd ~/path` expands `~`.
  - `cd -` jumps to the previous directory and prints the new path.
- `ls [OPTIONS]... [PATH]...`
  - `-l` long format
  - `-a` show hidden entries
  - `-F` append type indicators (`/`, `*`, `@`, `|`, `=`)
  - `--help` prints usage for `ls`
- `cat [FILE]...`
  - With no args, reads stdin and writes to stdout.
  - `cat -` reads stdin in place.
- `mkdir DIR...`
- `cp SRC DST`
  - Copies files only (directories are rejected).
- `mv SRC DST`
  - Renames when possible; falls back to copy+remove for files.
- `rm [-r|-R] PATH...`
  - Removes files; use `-r`/`-R` for recursive directory removal.

## Key bindings

- Enter: run command
- Ctrl+C: cancel the current input line
- Ctrl+D: exit (only when the current line is empty)
- ↑/↓: previous/next command from history
- ←/→, Home/End, Backspace/Delete: move/edit within the line

## Project structure

- `src/main.rs`: entry point
- `src/shell/`
  - `parse/`: tokenization + flag parsing into `Cmd`
  - `exec/`: built-in registry + command dispatch
  - `shell.rs`: prompt rendering, input loop, and line editor

## Notes

- This project uses Unix-specific APIs (`libc` + `std::os::unix`), so it targets Unix-like systems (macOS/Linux).
