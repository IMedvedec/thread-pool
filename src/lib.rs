struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        f()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_success() {
        ThreadPool::new(5);
    }

    #[test]
    #[should_panic]
    fn new_panic() {
        ThreadPool::new(0);
    }
}
