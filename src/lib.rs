use std::{sync::mpsc, thread};

pub struct PoolCreationError;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

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

        let (sender, receiver) = mpsc::channel();

        Self {
            workers: Worker::create_workers(size, receiver),
            sender,
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
        let (sender, receiver) = mpsc::channel();

        Ok(Self {
            workers: Worker::create_workers(size, receiver),
            sender,
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
    fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        Self {
            id,
            thread: thread::spawn(|| {
                receiver;
            }),
        }
    }

    fn create_workers(size: usize, receiver: mpsc::Receiver<Job>) -> Vec<Self> {
        let mut result = Vec::with_capacity(size);

        for id in 0..size {
            result.push(Worker::new(id, receiver));
        }

        result
    }
}
