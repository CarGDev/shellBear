use std::fs;

pub fn rmdir_process<'a>(args: impl Iterator<Item = &'a str>) {
    for path in args {
        if let Err(e) = fs::remove_dir(path) {
            eprintln!("rmdir: {}: {}", path, e);
        }
    }
}
