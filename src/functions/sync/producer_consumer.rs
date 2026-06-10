use super::SyncManager;

pub fn run_producer_consumer(manager: &mut SyncManager) {
    println!("\n=== Producer-Consumer Problem ===");
    println!("Buffer size: 5 items");
    println!("Using semaphores for synchronization\n");

    manager.create_semaphore("empty", 5);
    manager.create_semaphore("full", 0);
    manager.create_mutex("buffer_mutex");

    let mut buffer: Vec<usize> = Vec::new();
    let mut next_produce = 1;

    for round in 0..7 {
        println!("\n--- Round {} ---", round + 1);

        // Producer (process 1)
        let item = next_produce;
        next_produce += 1;
        println!("Producer wants to produce item {}", item);

        manager.sem_wait("empty", 1);
        manager.mutex_lock("buffer_mutex", 1);

        buffer.push(item);
        println!("Producer produced item {} (buffer: {:?})", item, buffer);

        manager.mutex_unlock("buffer_mutex", 1);
        manager.sem_signal("full");

        // Consumer (process 2)
        println!("Consumer wants to consume");
        manager.sem_wait("full", 2);
        manager.mutex_lock("buffer_mutex", 2);

        let consumed = buffer.remove(0);
        println!("Consumer consumed item {} (buffer: {:?})", consumed, buffer);

        manager.mutex_unlock("buffer_mutex", 2);
        manager.sem_signal("empty");
    }

    println!("\nProducer-Consumer simulation completed successfully.");
    println!("No race conditions occurred — mutex ensured exclusive buffer access.");
    println!("Semaphores 'empty' and 'full' prevented buffer overflow/underflow.");
}
