mod add_job;
mod bg_job;
mod fg_job;
mod list_jobs;
mod reap_zombies;

use std::process::Child;

#[derive(Clone, Copy, PartialEq)]
pub enum JobStatus {
    Running,
    Stopped,
}

pub struct Job {
    pub id: usize,
    pub pid: u32,
    pub command: String,
    pub status: JobStatus,
    child: Option<Child>,
}

pub struct JobManager {
    jobs: Vec<Job>,
    next_id: usize,
}

impl JobManager {
    pub fn new() -> Self {
        JobManager {
            jobs: Vec::new(),
            next_id: 1,
        }
    }

    fn parse_job_id<'a>(mut args: impl Iterator<Item = &'a str>) -> Option<usize> {
        let s = args.next()?;
        s.trim_start_matches('%').parse().ok()
    }
}
