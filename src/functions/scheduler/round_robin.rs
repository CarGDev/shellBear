use std::collections::VecDeque;
use std::thread;
use std::time::{Duration, Instant};

use super::process::ProcessStatus;
use super::statistics::EventType;
use super::SchedulerManager;

pub fn run_round_robin(manager: &mut SchedulerManager) {
    if manager.processes.is_empty() {
        println!("No processes to schedule.");
        return;
    }

    println!(
        "[Scheduler] Starting Round-Robin (Quantum: {}ms)",
        manager.quantum
    );
    println!(
        "[Scheduler] {} processes in queue\n",
        manager.processes.len()
    );

    let mut ready_queue: VecDeque<usize> = (0..manager.processes.len()).collect();
    let start_time = Instant::now();
    manager.start_time = Some(start_time);

    while !ready_queue.is_empty() {
        let process_idx = ready_queue.pop_front().unwrap();
        let process_completed;
        let process_name;

        {
            let process = &mut manager.processes[process_idx];

            if process.is_complete() {
                continue;
            }

            let elapsed_ms = start_time.elapsed().as_millis();

            if process.status == ProcessStatus::Ready || process.total_run_time == 0 {
                println!(
                    "[{}ms] Process #{} '{}' started (0ms/{}ms)",
                    elapsed_ms, process.id, process.name, process.total_duration
                );
                process.status = ProcessStatus::Running;
            } else {
                println!(
                    "[{}ms] Process #{} '{}' resumed ({}ms/{}ms)",
                    elapsed_ms,
                    process.id,
                    process.name,
                    process.total_run_time,
                    process.total_duration
                );
                process.status = ProcessStatus::Running;
            }

            let run_time = manager.quantum.min(process.remaining_time);

            thread::sleep(Duration::from_millis(run_time));

            let _actual_run = process.run_for(run_time);

            let elapsed_ms = start_time.elapsed().as_millis();
            println!(
                "[{}ms] Process #{} '{}' progress ({}ms/{}ms)",
                elapsed_ms,
                process.id,
                process.name,
                process.total_run_time,
                process.total_duration
            );

            process_completed = process.is_complete();
            process_name = process.name.clone();

            if process_completed {
                println!(
                    "[{}ms] Process #{} '{}' COMPLETED",
                    elapsed_ms, process.id, process.name
                );
                manager.stats.record_event(
                    elapsed_ms,
                    EventType::ProcessCompleted,
                    process.id,
                    process.name.clone(),
                );
            } else {
                process.status = ProcessStatus::Waiting;
                ready_queue.push_back(process_idx);
            }
        }

        if !process_completed && ready_queue.len() > 1 {
            let next_idx = ready_queue[0];
            let next_name = manager.processes[next_idx].name.clone();
            let elapsed_ms = start_time.elapsed().as_millis();
            println!(
                "[{}ms] Switching: {} -> {}",
                elapsed_ms, process_name, next_name
            );
            manager.stats.context_switches += 1;
        }
    }

    let total_time = start_time.elapsed().as_millis();
    manager.stats.print_summary(&manager.processes, total_time);
}
