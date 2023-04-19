pub struct ThreadPool;
pub struct PoolCreationError;

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
        Self
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
        Ok(Self)
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + 'static + Send,
    {
    }
}