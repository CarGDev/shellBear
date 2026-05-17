use super::JobManager;

impl JobManager {
    pub(super) fn reap_zombies(&mut self) {
        let done: Vec<usize> = self
            .jobs
            .iter_mut()
            .enumerate()
            .filter_map(|(i, job)| {
                if let Some(ref mut child) = job.child {
                    child.try_wait().ok().flatten().map(|_| i)
                } else {
                    None
                }
            })
            .collect();
        for i in done.into_iter().rev() {
            self.jobs.remove(i);
        }
    }
}
