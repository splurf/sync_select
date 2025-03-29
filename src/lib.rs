use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{JoinHandle, Thread, current, park, spawn},
};

pub struct SyncSelect {
    inner: Thread,
    any: Arc<AtomicBool>,
}

impl SyncSelect {
    pub fn new() -> Self {
        Self {
            inner: current(),
            any: Default::default(),
        }
    }

    pub fn thread(&self) -> Thread {
        self.inner.clone()
    }

    pub fn join(&self) {
        // don't park if a child is already dead
        if !self.any.load(Ordering::Acquire) {
            park();
        }
    }

    pub fn spawn<T, F>(&self, f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self._spawn(f)
    }

    pub fn spawn_with<F, T>(&self, f: F) -> JoinHandle<T>
    where
        F: FnOnce(&SyncSelect) -> T + Send + 'static,
        T: Send + 'static,
    {
        self._spawn(|| f(&SyncSelect::new()))
    }

    fn _spawn<T, F>(&self, f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let any = self.any.clone();
        let cnt = self.inner.clone();

        // spawn thread
        spawn(move || {
            // run task
            let result = f();

            // prevent incorrect park ordering
            any.store(true, Ordering::Release);

            // unpark root thread
            cnt.unpark();

            result
        })
    }
}

impl Default for SyncSelect {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SyncSelect {
    fn drop(&mut self) {
        self.join();
    }
}
