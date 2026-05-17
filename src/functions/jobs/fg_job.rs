use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;

use super::JobManager;
use super::JobStatus;

impl JobManager {
    pub fn fg_job<'a>(&mut self, args: impl Iterator<Item = &'a str>) {
        self.reap_zombies();
        let job_id = Self::parse_job_id(args);
        let job_id = job_id.unwrap_or_else(|| self.jobs.last().map_or(0, |j| j.id));
        if let Some(pos) = self.jobs.iter().position(|j| j.id == job_id) {
            let mut job = self.jobs.remove(pos);
            if job.status == JobStatus::Stopped {
                let _ = signal::kill(Pid::from_raw(job.pid as i32), Signal::SIGCONT);
            }
            if let Some(mut child) = job.child.take() {
                let _ = child.wait();
            }
        } else {
            eprintln!("fg: job not found: {}", job_id);
        }
    }
}
