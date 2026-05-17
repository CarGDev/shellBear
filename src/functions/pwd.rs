use std::env;

pub fn pwd_process() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("{}", e),
    }
}
