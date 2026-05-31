pub mod add_process;
pub mod priority;
pub mod process;
pub mod round_robin;
pub mod set_scheduler;
pub mod start_scheduler;
pub mod statistics;

use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum SchedulingAlgorithm {
    None,
    RoundRobin,
    Priority,
}

pub struct SchedulerManager {
    pub processes: Vec<process::SimulatedProcess>,
    pub algorithm: SchedulingAlgorithm,
    pub quantum: u64,
    next_id: usize,
    pub start_time: Option<Instant>,
    pub stats: statistics::SchedulerStats,
}

impl SchedulerManager {
    pub fn new() -> Self {
        SchedulerManager {
            processes: Vec::new(),
            algorithm: SchedulingAlgorithm::None,
            quantum: 100,
            next_id: 1,
            start_time: None,
            stats: statistics::SchedulerStats::new(),
        }
    }

    pub fn add_process(&mut self, name: String, duration: u64, priority: u8) {
        let p = process::SimulatedProcess::new(self.next_id, name.clone(), duration, priority);
        println!(
            "Added: Process #{} '{}' ({}ms, priority {})",
            p.id, p.name, duration, priority
        );
        self.processes.push(p);
        self.next_id += 1;
    }

    pub fn set_algorithm(&mut self, algorithm: SchedulingAlgorithm, quantum: Option<u64>) {
        self.algorithm = algorithm;
        if let Some(q) = quantum {
            self.quantum = q;
        }
    }

    pub fn list_processes(&self) {
        if self.processes.is_empty() {
            println!("No processes in queue.");
            return;
        }

        println!("Processes in queue ({:?}):", self.algorithm);
        for proc in &self.processes {
            let quantum_info = if self.algorithm == SchedulingAlgorithm::RoundRobin {
                format!(", quantum: {}ms", self.quantum)
            } else {
                String::new()
            };
            println!(
                "[{}] {} - {}ms (priority: {}) - {:?}{}",
                proc.id, proc.name, proc.total_duration, proc.priority, proc.status, quantum_info
            );
        }
    }

    pub fn clear_processes(&mut self) {
        self.processes.clear();
        self.next_id = 1;
        self.stats = statistics::SchedulerStats::new();
        println!("All processes cleared.");
    }
}
