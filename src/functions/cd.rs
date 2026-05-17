use std::env;
use std::path::Path;

pub fn cd_process<'a>(
    args: &mut std::iter::Peekable<impl Iterator<Item = &'a str>>,
    previous_command: &mut Option<std::process::Child>,
) {
    let new_dir = args.peek().map_or("/", |x| *x);
    let root = Path::new(new_dir);
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }
    *previous_command = None;
}
