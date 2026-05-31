use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::thread;
use std::time::{Duration, Instant};

use super::process::ProcessStatus;
use super::statistics::EventType;
use super::SchedulerManager;

#[derive(Eq)]
struct PriorityProcess {
    priority: u8,
    arrival_order: usize,
    index: usize,
}

impl Ord for PriorityProcess {
    fn cmp(&self, other: &Self) -> Ordering {
        self
            .priority
            .cmp(&other.priority)
            .then_with(|| other.arrival_order.cmp(&self.arrival_order))
    }
}

impl PartialOrd for PriorityProcess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PriorityProcess {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.arrival_order == other.arrival_order
    }
}

pub fn run_priority_scheduler(manager: &mut SchedulerManager) {
    if manager.processes.is_empty() {
        println!("No processes to schedule.");
        return;
    }

    println!("[Scheduler] Starting Priority-Based Scheduling");
    println!(
        "[Scheduler] {} processes in queue\n",
        manager.processes.len()
    );

    let mut heap: BinaryHeap<PriorityProcess> = manager
        .processes
        .iter()
        .enumerate()
        .map(|(idx, proc)| PriorityProcess {
            priority: proc.priority,
            arrival_order: idx,
            index: idx,
        })
        .collect();

    let start_time = Instant::now();
    manager.start_time = Some(start_time);

    while !heap.is_empty() {
        let priority_proc = heap.pop().unwrap();
        let process_idx = priority_proc.index;
        let process = &mut manager.processes[process_idx];

        if process.is_complete() {
            continue;
        }

        let elapsed_ms = start_time.elapsed().as_millis();

        if process.status == ProcessStatus::Ready || process.total_run_time == 0 {
            println!(
                "[{}ms] Process #{} '{}' started (priority: {}, 0ms/{}ms)",
                elapsed_ms,
                process.id,
                process.name,
                process.priority,
                process.total_duration
            );
            process.status = ProcessStatus::Running;
        } else {
            println!(
                "[{}ms] Process #{} '{}' resumed (priority: {}, {}ms/{}ms)",
                elapsed_ms,
                process.id,
                process.name,
                process.priority,
                process.total_run_time,
                process.total_duration
            );
            process.status = ProcessStatus::Running;
        }

        let run_time = process.remaining_time;

        thread::sleep(Duration::from_millis(run_time));

        process.run_for(run_time);

        let elapsed_ms = start_time.elapsed().as_millis();
        println!(
            "[{}ms] Process #{} '{}' COMPLETED (priority: {})",
            elapsed_ms, process.id, process.name, process.priority
        );

        manager.stats.record_event(
            elapsed_ms,
            EventType::ProcessCompleted,
            process.id,
            process.name.clone(),
        );
    }

    let total_time = start_time.elapsed().as_millis();
    manager.stats.print_summary(&manager.processes, total_time);
}
