use std::thread;

struct ThreadPool {
    workers: Vec<Worker>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        Worker {
            id: id,
            thread: thread::spawn(|| {}),
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id))
        }

        ThreadPool { workers: workers }
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

    #[test]
    fn execute_one_success() {
        let tp = ThreadPool::new(1);
        tp.execute(|| println!("Hello world!"))
    }
}
