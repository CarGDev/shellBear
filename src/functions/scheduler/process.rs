use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessStatus {
    Ready,
    Running,
    Waiting,
    Completed,
}

#[derive(Debug, Clone)]
pub struct SimulatedProcess {
    pub id: usize,
    pub name: String,
    pub total_duration: u64,
    pub remaining_time: u64,
    pub priority: u8,
    pub arrival_time: Instant,
    pub start_time: Option<Instant>,
    pub completion_time: Option<Instant>,
    pub total_run_time: u64,
    pub status: ProcessStatus,
}

impl SimulatedProcess {
    pub fn new(id: usize, name: String, duration: u64, priority: u8) -> Self {
        SimulatedProcess {
            id,
            name,
            total_duration: duration,
            remaining_time: duration,
            priority,
            arrival_time: Instant::now(),
            start_time: None,
            completion_time: None,
            total_run_time: 0,
            status: ProcessStatus::Ready,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.remaining_time == 0
    }

    pub fn run_for(&mut self, duration_ms: u64) -> u64 {
        let actual_time = duration_ms.min(self.remaining_time);
        self.remaining_time -= actual_time;
        self.total_run_time += actual_time;

        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }

        if self.remaining_time == 0 {
            self.status = ProcessStatus::Completed;
            self.completion_time = Some(Instant::now());
        }

        actual_time
    }
}
