pub extern crate tokio;

use std::sync::Arc;
use tokio::sync::Mutex;

thread_local! {
    static DATA: Arc<Mutex<String>> = Arc::new(Mutex::new("Alec".into()));
}

use std::future::Future;
use tokio::task::JoinHandle;

pub fn spawn<T, F>(task: F) -> JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
    F: Fn(String) -> T + Send + 'static,
{
    tokio::spawn(task("alec".into()))
}
