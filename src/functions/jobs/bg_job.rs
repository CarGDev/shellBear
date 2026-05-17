use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;

use super::JobManager;
use super::JobStatus;

impl JobManager {
    pub fn bg_job<'a>(&mut self, args: impl Iterator<Item = &'a str>) {
        self.reap_zombies();
        let job_id = Self::parse_job_id(args);
        let job_id = job_id.unwrap_or_else(|| self.jobs.last().map_or(0, |j| j.id));
        if let Some(job) = self.jobs.iter_mut().find(|j| j.id == job_id) {
            if job.status == JobStatus::Stopped {
                let _ = signal::kill(Pid::from_raw(job.pid as i32), Signal::SIGCONT);
                job.status = JobStatus::Running;
                println!("[{}] {}", job.id, job.command);
            }
        } else {
            eprintln!("bg: job not found: {}", job_id);
        }
    }
}
