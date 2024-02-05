use std::cmp::{max, min};
use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::{available_parallelism};
use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinHandle;
use dashmap::DashMap;

#[derive(Debug)]
pub struct AsyncRuntime {
    async_runtime: Runtime,
    join_handle_map: DashMap<usize, JoinHandle<()>>,
    last_handle_id: AtomicUsize
}

impl AsyncRuntime {
    pub fn new(thread_request: Option<usize>) -> Arc<AsyncRuntime> {
        let core_count = available_parallelism().unwrap().get();
        let thread_count = min(max(thread_request.unwrap_or(1), 1), core_count);
        let async_runtime = Builder::new_multi_thread()
            .worker_threads(thread_count)
            .enable_all()
            .build()
            .unwrap();
        Arc::new(AsyncRuntime {
            async_runtime,
            join_handle_map: DashMap::new(),
            last_handle_id: AtomicUsize::new(0)
        })
    }

    pub fn spawn(&mut self, future: impl Future<Output = ()>) -> usize {
        let handle_id = self.last_handle_id.fetch_add(1, Ordering::SeqCst);
        let handle = self.async_runtime.spawn(future);
        self.join_handle_map.insert(handle_id, handle);
        handle_id
    }

    pub fn cancel(&mut self, handle_id: usize) {
        if let Some((_, handle)) = self.join_handle_map.remove(&handle_id) {
            if !handle.is_finished() {
                handle.abort();
            }
        }
    }

    pub fn stop(&mut self) {
        for (_, handle) in self.join_handle_map.into_iter() {
            if !handle.is_finished() {
                handle.abort();
            }
        }
        self.join_handle_map.clear();
        self.async_runtime.shutdown_background();
    }
}

impl Drop for AsyncRuntime {
    fn drop(&mut self) {

    }
}