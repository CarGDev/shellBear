pub mod dining;
pub mod producer_consumer;

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Mutex {
    pub name: String,
    pub locked: bool,
    pub owner: Option<usize>,
    pub wait_queue: VecDeque<usize>,
}

impl Mutex {
    pub fn new(name: &str) -> Self {
        Mutex {
            name: name.to_string(),
            locked: false,
            owner: None,
            wait_queue: VecDeque::new(),
        }
    }

    pub fn lock(&mut self, process_id: usize) -> bool {
        if !self.locked {
            self.locked = true;
            self.owner = Some(process_id);
            println!("Mutex '{}' acquired by Process #{}", self.name, process_id);
            true
        } else {
            self.wait_queue.push_back(process_id);
            println!(
                "Mutex '{}' is held by Process #{}. Process #{} added to wait queue.",
                self.name,
                self.owner.unwrap(),
                process_id
            );
            false
        }
    }

    pub fn unlock(&mut self, process_id: usize) {
        if self.owner != Some(process_id) {
            println!(
                "Error: Process #{} does not hold mutex '{}'",
                process_id, self.name
            );
            return;
        }

        self.locked = false;
        self.owner = None;
        println!("Mutex '{}' released by Process #{}", self.name, process_id);

        if let Some(next) = self.wait_queue.pop_front() {
            self.locked = true;
            self.owner = Some(next);
            println!(
                "Mutex '{}' immediately acquired by Process #{} (from wait queue)",
                self.name, next
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct Semaphore {
    pub name: String,
    pub count: isize,
    pub wait_queue: VecDeque<usize>,
}

impl Semaphore {
    pub fn new(name: &str, initial: isize) -> Self {
        Semaphore {
            name: name.to_string(),
            count: initial,
            wait_queue: VecDeque::new(),
        }
    }

    pub fn wait(&mut self, process_id: usize) {
        self.count -= 1;
        if self.count < 0 {
            self.wait_queue.push_back(process_id);
            println!(
                "Semaphore '{}': Process #{} blocked (count={})",
                self.name, process_id, self.count
            );
        } else {
            println!(
                "Semaphore '{}': Process #{} acquired (count={})",
                self.name, process_id, self.count
            );
        }
    }

    pub fn signal(&mut self) -> Option<usize> {
        self.count += 1;
        let awakened = if self.count <= 0 {
            self.wait_queue.pop_front()
        } else {
            None
        };

        if let Some(pid) = awakened {
            println!(
                "Semaphore '{}': Process #{} awakened (count={})",
                self.name, pid, self.count
            );
        } else {
            println!(
                "Semaphore '{}': signal complete (count={})",
                self.name, self.count
            );
        }

        awakened
    }
}

#[derive(Debug, Clone)]
pub struct SyncManager {
    pub mutexes: Vec<Mutex>,
    pub semaphores: Vec<Semaphore>,
}

impl SyncManager {
    pub fn new() -> Self {
        SyncManager {
            mutexes: Vec::new(),
            semaphores: Vec::new(),
        }
    }

    pub fn create_mutex(&mut self, name: &str) {
        if self.mutexes.iter().any(|m| m.name == name) {
            println!("Mutex '{}' already exists", name);
            return;
        }
        self.mutexes.push(Mutex::new(name));
        println!("Created mutex '{}'", name);
    }

    pub fn create_semaphore(&mut self, name: &str, initial: isize) {
        if self.semaphores.iter().any(|s| s.name == name) {
            println!("Semaphore '{}' already exists", name);
            return;
        }
        self.semaphores.push(Semaphore::new(name, initial));
        println!("Created semaphore '{}' with initial count {}", name, initial);
    }

    pub fn mutex_lock(&mut self, name: &str, process_id: usize) {
        match self.mutexes.iter_mut().find(|m| m.name == name) {
            Some(m) => {
                m.lock(process_id);
            }
            None => println!("Error: Mutex '{}' not found", name),
        }
    }

    pub fn mutex_unlock(&mut self, name: &str, process_id: usize) {
        match self.mutexes.iter_mut().find(|m| m.name == name) {
            Some(m) => {
                m.unlock(process_id);
            }
            None => println!("Error: Mutex '{}' not found", name),
        }
    }

    pub fn sem_wait(&mut self, name: &str, process_id: usize) {
        match self.semaphores.iter_mut().find(|s| s.name == name) {
            Some(s) => s.wait(process_id),
            None => println!("Error: Semaphore '{}' not found", name),
        }
    }

    pub fn sem_signal(&mut self, name: &str) {
        match self.semaphores.iter_mut().find(|s| s.name == name) {
            Some(s) => {
                s.signal();
            }
            None => println!("Error: Semaphore '{}' not found", name),
        }
    }

    pub fn list(&self) {
        println!("\n=== Synchronization Objects ===");
        println!("Mutexes:");
        if self.mutexes.is_empty() {
            println!("  (none)");
        } else {
            for m in &self.mutexes {
                let status = if m.locked {
                    format!("LOCKED by Process #{}", m.owner.unwrap())
                } else {
                    "UNLOCKED".to_string()
                };
                println!(
                    "  '{}' - {} (wait queue: {})",
                    m.name,
                    status,
                    m.wait_queue.len()
                );
            }
        }

        println!("Semaphores:");
        if self.semaphores.is_empty() {
            println!("  (none)");
        } else {
            for s in &self.semaphores {
                println!(
                    "  '{}' - count: {} (wait queue: {})",
                    s.name,
                    s.count,
                    s.wait_queue.len()
                );
            }
        }
    }
}
