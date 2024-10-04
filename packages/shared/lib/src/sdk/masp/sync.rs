use namada_sdk::control_flow::ShutdownSignal;
use namada_sdk::io::ProgressBar;
use namada_sdk::task_env::{TaskEnvironment, TaskSpawner};
use wasm_bindgen_futures::spawn_local;

pub struct TaskSpawnerWeb {}

impl TaskSpawner for TaskSpawnerWeb {
    fn spawn_async<F>(&self, fut: F)
    where
        F: std::future::Future<Output = ()> + 'static,
    {
        // In browsers we use `spawn_local` which runs the future in the WebAssembly context.
        spawn_local(fut);
    }

    fn spawn_sync<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        job();
    }
}

pub struct TaskEnvWeb {}

impl TaskEnvironment for TaskEnvWeb {
    type Spawner = TaskSpawnerWeb;

    async fn run<M, F, R>(self, main: M) -> R
    where
        M: FnOnce(Self::Spawner) -> F,
        F: std::future::Future<Output = R>,
    {
        main(TaskSpawnerWeb {}).await
    }
}

// We can't use the real shutdown signal in the browser, so we just use a dummy one.
pub struct ShutdownSignalWeb {}

impl ShutdownSignal for ShutdownSignalWeb {
    async fn wait_for_shutdown(&mut self) {}

    fn received(&mut self) -> bool {
        false
    }
}

pub struct ProgressBarWeb {
    pub total: usize,
    pub current: usize,
}

impl ProgressBar for ProgressBarWeb {
    fn upper_limit(&self) -> u64 {
        self.total as u64
    }

    fn set_upper_limit(&mut self, limit: u64) {
        self.total = limit as usize;
    }

    fn increment_by(&mut self, amount: u64) {
        self.current += amount as usize;
        web_sys::console::log_1(&format!("Progress: {}/{}", self.current, self.total).into());
    }

    fn message(&mut self, message: String) {
        web_sys::console::log_1(&message.into());
    }

    fn finish(&mut self) {
        web_sys::console::log_1(&"Finished".into());
    }
}