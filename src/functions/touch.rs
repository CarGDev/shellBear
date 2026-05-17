use std::fs::OpenOptions;

pub fn touch_process<'a>(args: impl Iterator<Item = &'a str>) {
    for path in args {
        match OpenOptions::new().create(true).write(true).open(path) {
            Ok(_) => {}
            Err(e) => eprintln!("touch: {}: {}", path, e),
        }
    }
}
