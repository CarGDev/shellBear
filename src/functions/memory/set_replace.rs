use super::PageReplacement;
use super::MemoryManager;

pub fn set_page_replace<'a>(
    manager: &mut MemoryManager,
    mut args: impl Iterator<Item = &'a str>,
) {
    let algo = match args.next() {
        Some("fifo") => PageReplacement::FIFO,
        Some("lru") => PageReplacement::LRU,
        _ => {
            println!("Usage: setpagereplace <fifo|lru>");
            return;
        }
    };

    manager.set_algorithm(algo);
}
