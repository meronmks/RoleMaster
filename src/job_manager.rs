use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use tokio::sync::{Mutex, OnceCell};
use tokio_cron_scheduler::{Job, JobScheduler};
use core::time::Duration;

#[derive(Clone)]
pub struct JobManager {
    scheduler: Arc<JobScheduler>,
    jobs: Arc<Mutex<HashMap<String, Job>>>
}

static JOB_MANAGER: OnceCell<JobManager> = OnceCell::const_new();

impl JobManager {
    pub async fn initialize() -> Result<(), Box<dyn std::error::Error>> {
        let scheduler = JobScheduler::new().await?;
        scheduler.start().await?;

        let job_manager = JobManager {
            scheduler: Arc::new(scheduler),
            jobs: Arc::new(Mutex::new(HashMap::new())),
        };

        JOB_MANAGER.set(job_manager).map_err(|_| "JobManager is already initialized".into())
    }

    pub fn instance() -> &'static JobManager {
        JOB_MANAGER.get().expect("JobManager is not initialized")
    }

    pub async fn add_one_shot_job<F>(job_name: &str, scheduled_time: Duration, task: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let job = Job::new_one_shot(scheduled_time, move |_id, _lock| {
            task();
        })?;

        let instance = Self::instance();

        instance.scheduler.add(job.clone()).await?;

        instance.jobs.lock().await.insert(job_name.to_string(), job);
        Ok(())
    }

    pub async fn cancel_job(job_name: &str) -> Result<(), String> {
        let instance = Self::instance();
        let mut jobs = instance.jobs.lock().await;
        if let Some(job) = jobs.remove(job_name) {
            if instance.scheduler.remove(&job.guid()).await.is_ok() {
                Ok(())
            } else {
                Err(format!("Failed to cancel job '{}'", job_name))
            }
        } else {
            Err(format!("Job '{}' not found", job_name))
        }
    }

    pub async fn get_scheduled_jobs(prefix: Option<&str>) -> Vec<String> {
        let instance = Self::instance();
        let jobs = instance.jobs.lock().await;
        jobs.keys()
            .filter(|name| match prefix {
                Some(p) => name.starts_with(p),
                None => true,
            })
            .cloned()
            .collect()
    }
}