use super::MemoryManager;

pub fn access_page<'a>(
    manager: &mut MemoryManager,
    mut args: impl Iterator<Item = &'a str>,
) {
    let process_id: usize = match args.next() {
        Some(p) => match p.parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Usage: accesspage <process_id> <virtual_page_number>");
                return;
            }
        },
        None => {
            println!("Usage: accesspage <process_id> <virtual_page_number>");
            return;
        }
    };

    let page_number: usize = match args.next() {
        Some(c) => match c.parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Usage: accesspage <process_id> <virtual_page_number>");
                return;
            }
        },
        None => {
            println!("Usage: accesspage <process_id> <virtual_page_number>");
            return;
        }
    };

    manager.access_page(process_id, page_number);
}
