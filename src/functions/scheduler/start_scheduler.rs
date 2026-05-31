use super::round_robin::run_round_robin;
use super::priority::run_priority_scheduler;
use super::SchedulingAlgorithm;
use super::SchedulerManager;

pub fn start_scheduler(manager: &mut SchedulerManager) {
    match manager.algorithm {
        SchedulingAlgorithm::None => {
            println!("No scheduling algorithm set.");
            println!("Use 'setscheduler rr <quantum_ms>' or 'setscheduler priority' first.");
        }
        SchedulingAlgorithm::RoundRobin => {
            run_round_robin(manager);
        }
        SchedulingAlgorithm::Priority => {
            run_priority_scheduler(manager);
        }
    }
}
