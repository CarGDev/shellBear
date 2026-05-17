use std::fs;

pub fn rm_process<'a>(args: impl Iterator<Item = &'a str>) {
    for path in args {
        if let Err(e) = fs::remove_file(path) {
            eprintln!("rm: {}: {}", path, e);
        }
    }
}
