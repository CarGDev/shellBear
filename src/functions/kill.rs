use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;

pub fn kill_process<'a>(args: impl Iterator<Item = &'a str>) {
    for arg in args {
        let pid: i32 = match arg.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("kill: {}: invalid pid", arg);
                continue;
            }
        };
        if let Err(e) = signal::kill(Pid::from_raw(pid), Signal::SIGTERM) {
            eprintln!("kill: {}: {}", pid, e);
        }
    }
}
