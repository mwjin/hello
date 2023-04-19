use std::thread;

pub struct PoolCreationError;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        Self {
            workers: Worker::create_workers(size),
        }
    }

    /// Create a new ThreadPool.
    ///
    /// This behavior is the same with the `new` function
    ///
    /// Except that the `build` function consider a case where the size is zero
    ///
    /// as a recoverable error.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError);
        }
        Ok(Self {
            workers: Worker::create_workers(size),
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + 'static + Send,
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        Self {
            id,
            thread: thread::spawn(|| {}),
        }
    }

    fn create_workers(size: usize) -> Vec<Self> {
        let mut result = Vec::with_capacity(size);

        for id in 0..size {
            result.push(Worker::new(id));
        }

        result
    }
}
