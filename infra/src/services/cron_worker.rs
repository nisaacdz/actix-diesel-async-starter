use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tracing::info;

#[derive(Clone)]
pub struct CronWorker {
    stop_tx: broadcast::Sender<()>,
    handles: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl Default for CronWorker {
    fn default() -> Self {
        Self::new()
    }
}

impl CronWorker {
    pub fn new() -> Self {
        let (stop_tx, _) = broadcast::channel(1);
        Self {
            stop_tx,
            handles: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Spawns an infinitely looping task spanning a given `Duration`.
    /// Automatically wiretaps the internal shutdown signal, making it impossible to hang!
    pub fn spawn<F, Fut>(&self, duration: std::time::Duration, mut work: F)
    where
        F: FnMut() -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut rx = self.stop_tx.subscribe();
        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(duration);
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        work().await;
                    }
                    _ = rx.recv() => {
                        tracing::info!("Scheduled cron stopped gracefully.");
                        break;
                    }
                }
            }
        });

        if let Ok(mut handles) = self.handles.lock() {
            handles.push(handle);
        }
    }

    /// Broadcast the stop signal to all active listeners telling them to break their scheduled loops.
    pub fn stop(&self) {
        if self.stop_tx.receiver_count() > 0 {
            info!(
                "Sending stop signal to {} scheduled task(s).",
                self.stop_tx.receiver_count()
            );
            let _ = self.stop_tx.send(());
        }
    }

    /// Wait for all successfully spawned background tasks to actually finish.
    pub async fn join_all(&self) {
        let handles = {
            let mut lock = self.handles.lock().unwrap();
            std::mem::take(&mut *lock)
        };

        info!(
            "Waiting for {} scheduled task(s) to completely drain...",
            handles.len()
        );
        futures_util::future::join_all(handles).await;
    }
}
