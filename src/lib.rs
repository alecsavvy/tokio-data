pub extern crate tokio;

#[macro_export]
macro_rules! tokio_data {
    ($data_type:ty, $data:expr) => {
        type TokioData = $data_type;

        use std::sync::Arc;
        use tokio::sync::{Mutex, MutexGuard};

        thread_local! {
            static DATA: Arc<Mutex<TokioData>> = Arc::new(Mutex::new($data));
        }

        use std::future::Future;
        use tokio::task::JoinHandle;
        pub fn spawn_data<T, F>(task: F) -> JoinHandle<T::Output>
        where
            T: Future + Send + 'static,
            T::Output: Send + 'static,
            F: Fn(Arc<Mutex<TokioData>>) -> T + Send,
        {
            let data_arc = DATA.with(|data| data.clone());
            tokio::spawn(task(data_arc))
        }
    };
}
