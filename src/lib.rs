use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct PoolCreationError;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

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

        let receiver = Arc::new(Mutex::new(receiver));

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

        let receiver = Arc::new(Mutex::new(receiver));

        Ok(Self {
            workers: Worker::create_workers(size, receiver),
            sender,
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + 'static + Send,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        Self {
            id,
            thread: Some(thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {id} got a job; executing.");

                job();
            })),
        }
    }

    fn create_workers(size: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Vec<Self> {
        let mut result = Vec::with_capacity(size);

        for id in 0..size {
            result.push(Worker::new(id, Arc::clone(&receiver)));
        }

        result
    }
}
