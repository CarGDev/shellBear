use super::SchedulerManager;

pub fn add_process<'a>(
    manager: &mut SchedulerManager,
    mut args: impl Iterator<Item = &'a str>,
) {
    let name = match args.next() {
        Some(n) => n.to_string(),
        None => {
            println!("Usage: addproc <name> <duration_ms> [priority]");
            return;
        }
    };

    let duration: u64 = match args.next() {
        Some(d) => match d.parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Error: duration must be a positive integer (milliseconds)");
                return;
            }
        },
        None => {
            println!("Usage: addproc <name> <duration_ms> [priority]");
            return;
        }
    };

    let priority: u8 = match args.next() {
        Some(p) => match p.parse::<u8>() {
            Ok(val) => val,
            Err(_) => {
                println!("Error: priority must be an integer (0-255)");
                return;
            }
        },
        None => 128,
    };

    manager.add_process(name, duration, priority);
}
