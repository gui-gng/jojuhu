use std::time::Duration;
use tokio::time::interval;

use sqlx::PgPool;
use tracing::{error, info};

use super::account_deletion::AccountDeletionJob;
use super::data_export::DataExportJob;
use super::scheduled_posts::ScheduledPostsJob;

pub struct JobRegistry {
    pool: PgPool,
}

impl JobRegistry {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn start(&self) {
        let scheduled_posts_job = ScheduledPostsJob::new(self.pool.clone());
        let data_export_job = DataExportJob::new(self.pool.clone());
        let account_deletion_job = AccountDeletionJob::new(self.pool.clone());

        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(60));
            
            loop {
                ticker.tick().await;
                
                if let Err(e) = scheduled_posts_job.process_pending_posts().await {
                    error!("Scheduled posts job error: {}", e);
                }
            }
        });

        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(300));
            
            loop {
                ticker.tick().await;
                
                if let Err(e) = data_export_job.process_pending_exports().await {
                    error!("Data export job error: {}", e);
                }
            }
        });

        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(600));
            
            loop {
                ticker.tick().await;
                
                if let Err(e) = account_deletion_job.process_pending_deletions().await {
                    error!("Account deletion job error: {}", e);
                }
            }
        });

        info!("Background jobs started");
    }
}

pub fn start_background_jobs(pool: PgPool) {
    let registry = JobRegistry::new(pool);
    tokio::spawn(async move {
        registry.start().await;
    });
}