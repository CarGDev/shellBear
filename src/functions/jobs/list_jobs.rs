use super::JobManager;
use super::JobStatus;

impl JobManager {
    pub fn list_jobs(&mut self) {
        self.reap_zombies();
        if self.jobs.is_empty() {
            return;
        }
        for job in &self.jobs {
            let status_str = match job.status {
                JobStatus::Running => "Running",
                JobStatus::Stopped => "Stopped",
            };
            println!("[{}]  {}    {}", job.id, status_str, job.command);
        }
    }
}
