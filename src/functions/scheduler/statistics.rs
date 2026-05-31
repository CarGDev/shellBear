use super::process::SimulatedProcess;
use super::process::ProcessStatus;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum EventType {
    ProcessStarted,
    ProcessResumed,
    ProcessPaused,
    ProcessCompleted,
    ContextSwitch { from: usize, to: usize },
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SchedulingEvent {
    pub timestamp: u128,
    pub event_type: EventType,
    pub process_id: usize,
    pub process_name: String,
}

#[derive(Debug, Clone)]
pub struct SchedulerStats {
    pub context_switches: usize,
    pub events: Vec<SchedulingEvent>,
}

impl SchedulerStats {
    pub fn new() -> Self {
        SchedulerStats {
            context_switches: 0,
            events: Vec::new(),
        }
    }

    pub fn record_event(
        &mut self,
        elapsed_ms: u128,
        event_type: EventType,
        process_id: usize,
        process_name: String,
    ) {
        self.events.push(SchedulingEvent {
            timestamp: elapsed_ms,
            event_type,
            process_id,
            process_name,
        });
    }

    pub fn print_summary(&self, processes: &[SimulatedProcess], total_time_ms: u128) {
        println!("\n=== Scheduling Statistics ===");
        println!("Total Processes: {}", processes.len());
        println!("Context Switches: {}", self.context_switches);
        println!("Total Execution Time: {}ms\n", total_time_ms);

        println!("Process Details:");
        let completed: Vec<&SimulatedProcess> = processes
            .iter()
            .filter(|p| p.status == ProcessStatus::Completed)
            .collect();

        for proc in &completed {
            if let (Some(start), Some(end)) = (proc.start_time, proc.completion_time) {
                let wait_time = start.duration_since(proc.arrival_time).as_millis();
                let turnaround = end.duration_since(proc.arrival_time).as_millis();

                println!("[{}] {}", proc.id, proc.name);
                println!(
                    "    Arrival: 0ms | Start: {}ms | Completion: {}ms",
                    wait_time,
                    wait_time + proc.total_duration as u128
                );
                println!(
                    "    Duration: {}ms | Wait: {}ms | Turnaround: {}ms",
                    proc.total_duration, wait_time, turnaround
                );
            }
        }

        if !completed.is_empty() {
            let avg_wait: u128 = completed
                .iter()
                .filter_map(|p| {
                    p.start_time
                        .map(|s| s.duration_since(p.arrival_time).as_millis())
                })
                .sum::<u128>()
                / completed.len() as u128;

            let avg_turnaround: u128 = completed
                .iter()
                .filter_map(|p| {
                    p.completion_time
                        .map(|e| e.duration_since(p.arrival_time).as_millis())
                })
                .sum::<u128>()
                / completed.len() as u128;

            println!("\nAverage Wait Time: {}ms", avg_wait);
            println!("Average Turnaround Time: {}ms", avg_turnaround);
        }
    }
}
