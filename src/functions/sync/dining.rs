use super::SyncManager;

pub fn run_dining_philosophers(manager: &mut SyncManager) {
    println!("\n=== Dining Philosophers Problem ===");
    println!("5 philosophers, 5 forks");
    println!("Using mutexes for forks to prevent deadlock\n");

    for i in 0..5 {
        manager.create_mutex(&format!("fork{}", i));
    }

    let states = ["THINKING", "HUNGRY", "EATING"];

    for round in 0..7 {
        println!("\n--- Round {} ---", round + 1);
        for phil in 0..5 {
            let left = phil;
            let right = (phil + 1) % 5;
            let state = states[(round + phil) % 3];

            println!("Philosopher {} is {}", phil, state);

            if state == "EATING" {
                let left_fork = format!("fork{}", left);
                let right_fork = format!("fork{}", right);

                manager.mutex_lock(&left_fork, phil);
                manager.mutex_lock(&right_fork, phil);
                println!(
                    "Philosopher {} picked up forks {} and {} and is EATING",
                    phil, left, right
                );
                manager.mutex_unlock(&right_fork, phil);
                manager.mutex_unlock(&left_fork, phil);
                println!("Philosopher {} put down forks and is THINKING", phil);
            }
        }
    }

    println!("\nDining Philosophers simulation completed.");
    println!("Each philosopher acquires both forks (mutexes) before eating.");
    println!("Mutexes prevent two philosophers from using the same fork simultaneously.");
}
