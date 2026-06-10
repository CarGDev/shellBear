use super::MemoryManager;

pub fn alloc_pages<'a>(
    manager: &mut MemoryManager,
    mut args: impl Iterator<Item = &'a str>,
) {
    let process_id: usize = match args.next() {
        Some(p) => match p.parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Usage: allocpages <process_id> <num_pages>");
                return;
            }
        },
        None => {
            println!("Usage: allocpages <process_id> <num_pages>");
            return;
        }
    };

    let count: usize = match args.next() {
        Some(c) => match c.parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Usage: allocpages <process_id> <num_pages>");
                return;
            }
        },
        None => {
            println!("Usage: allocpages <process_id> <num_pages>");
            return;
        }
    };

    manager.allocate_pages(process_id, count);
}
