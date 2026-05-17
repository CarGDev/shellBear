# shellBear

A Unix shell written in Rust with built-in commands and job control.

## Features

### Built-in Commands
| Command | Description |
|---------|-------------|
| `cd` | Change current directory |
| `pwd` | Print working directory |
| `exit` | Exit the shell |
| `echo` | Print text (supports `\n`, `\t`, `\a`) |
| `clear` | Clear terminal screen |
| `ls` | List files (colorized: blue for dirs, cyan for symlinks) |
| `cat` | Display file contents |
| `mkdir` | Create a directory |
| `rmdir` | Remove an empty directory |
| `rm` | Remove a file |
| `touch` | Create an empty file |
| `kill` | Terminate a process by PID |


### Piping
Commands can be chained with `|`:
```
ls | grep Cargo
```

### Job Control
Run commands in the background with `&`:
```
sleep 5 &
```

| Command | Description |
|---------|-------------|
| `jobs` | List background jobs |
| `fg <id>` | Bring a job to foreground |
| `bg <id>` | Resume a stopped job in background |

### External Commands
Any command not listed above is executed via the system PATH (e.g., `grep`, `sort`, `find`, `git`).

## Requirements

- Rust 2024 edition
- [nix](https://crates.io/crates/nix) crate (for `kill` and job control signals)

## Getting Started

### Build
```bash
cargo build
```

### Run
```bash
cargo run
```

Or run the compiled binary directly:
```bash
./target/debug/shellBear
```

### Usage Example
```
> pwd
/home/user/projects
> ls
Cargo.toml  src  README.md
> cd /tmp
> pwd
/tmp
> echo hello world
hello world
> sleep 3 &
[1] 12345
> jobs
[1]  Running    sleep 3
> fg 1
```

## Project Structure

```
src/
├── main.rs                  # Shell loop, parsing, dispatch
└── functions/
    ├── mod.rs               # Module registry
    ├── cat.rs               # cat built-in
    ├── cd.rs                # cd built-in
    ├── clear.rs             # clear built-in
    ├── command.rs           # External command execution + piping
    ├── echo.rs              # echo built-in
    ├── jobs/
    │   ├── mod.rs           # Job, JobManager definitions
    │   ├── add_job.rs       # Background job creation
    │   ├── bg_job.rs        # bg command
    │   ├── fg_job.rs        # fg command
    │   ├── list_jobs.rs     # jobs command
    │   └── reap_zombies.rs  # Cleanup finished jobs
    ├── kill.rs              # kill built-in
    ├── ls.rs                # ls built-in
    ├── mkdir.rs             # mkdir built-in
    ├── pwd.rs               # pwd built-in
    ├── rm.rs                # rm built-in
    ├── rmdir.rs             # rmdir built-in
    └── touch.rs             # touch built-in
```
