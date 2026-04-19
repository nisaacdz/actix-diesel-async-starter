use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use tracing::info;

#[derive(Clone, Default)]
pub struct BgWorker {
    handles: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl BgWorker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn<Fut>(&self, fut: Fut)
    where
        Fut: Future<Output = ()> + Send + 'static,
    {
        let handle = tokio::spawn(fut);

        if let Ok(mut handles) = self.handles.lock() {
            handles.push(handle);
        }
    }

    /// Wait for all successfully spawned background tasks to actually finish finishing.
    pub async fn join_all(&self) {
        let handles = {
            let mut lock = self.handles.lock().unwrap();
            std::mem::take(&mut *lock)
        };

        info!(
            "Waiting for {} background task(s) to completely drain...",
            handles.len()
        );
        futures_util::future::join_all(handles).await;
    }
}
