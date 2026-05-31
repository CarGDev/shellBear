use super::SchedulingAlgorithm;
use super::SchedulerManager;

pub fn set_scheduler<'a>(
    manager: &mut SchedulerManager,
    mut args: impl Iterator<Item = &'a str>,
) {
    let algo = match args.next() {
        Some(a) => a,
        None => {
            println!("Usage: setscheduler <algorithm> [quantum]");
            println!("Algorithms: rr <quantum_ms>, priority");
            return;
        }
    };

    match algo {
        "rr" | "roundrobin" | "round-robin" => {
            let quantum: u64 = match args.next() {
                Some(q) => match q.parse() {
                    Ok(val) if val > 0 => val,
                    _ => {
                        println!("Error: quantum must be a positive integer (milliseconds)");
                        return;
                    }
                },
                None => {
                    println!("Error: Round-Robin requires a quantum value");
                    println!("Usage: setscheduler rr <quantum_ms>");
                    return;
                }
            };

            manager.set_algorithm(SchedulingAlgorithm::RoundRobin, Some(quantum));
            println!(
                "Scheduler set to: Round-Robin (Quantum: {}ms)",
                manager.quantum
            );
        }
        "priority" | "prio" => {
            manager.set_algorithm(SchedulingAlgorithm::Priority, None);
            println!("Scheduler set to: Priority-Based");
        }
        _ => {
            println!("Unknown algorithm: {}", algo);
            println!("Available: rr <quantum_ms>, priority");
        }
    }
}
