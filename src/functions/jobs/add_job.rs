use std::process::Child;

use super::Job;
use super::JobManager;
use super::JobStatus;

impl JobManager {
    pub fn add_job(&mut self, child: Child, command: String) {
        let pid = child.id();
        self.jobs.push(Job {
            id: self.next_id,
            pid,
            command,
            status: JobStatus::Running,
            child: Some(child),
        });
        println!("[{}] {}", self.next_id, pid);
        self.next_id += 1;
    }
}
