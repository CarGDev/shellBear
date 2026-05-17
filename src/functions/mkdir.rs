use std::fs;

pub fn mkdir_process<'a>(args: impl Iterator<Item = &'a str>) {
    for path in args {
        if let Err(e) = fs::create_dir(path) {
            eprintln!("mkdir: {}: {}", path, e);
        }
    }
}
