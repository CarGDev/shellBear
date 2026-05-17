use std::fs;

pub fn cat_process<'a>(args: impl Iterator<Item = &'a str>) {
    for path in args {
        match fs::read_to_string(path) {
            Ok(contents) => print!("{}", contents),
            Err(e) => eprintln!("cat: {}: {}", path, e),
        }
    }
}
