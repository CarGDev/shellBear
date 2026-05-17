use std::fs;

pub fn ls_process<'a>(args: impl Iterator<Item = &'a str>) {
    let path = args.peekable().peek().map_or(".", |x| *x);
    match fs::metadata(path) {
        Ok(meta) if meta.is_dir() => match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if entry.file_type().map_or(false, |t| t.is_dir()) {
                        println!("\x1B[34m{}\x1B[0m", name);
                    } else if entry.file_type().map_or(false, |t| t.is_symlink()) {
                        println!("\x1B[36m{}\x1B[0m", name);
                    } else {
                        println!("{}", name);
                    }
                }
            }
            Err(e) => eprintln!("ls: {}", e),
        },
        Ok(_) => println!("{}", path),
        Err(e) => eprintln!("ls: {}: {}", path, e),
    }
}
