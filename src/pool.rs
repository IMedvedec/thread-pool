use std::sync::{mpsc, Arc, Mutex};
use std::thread;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker #{} got a job; executing!", id);
                    job.call_box()
                }
                Message::Terminate => {
                    println!("Worker #{} is terminating!", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down workers!");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap()
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap()
            }
        }
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

    #[test]
    fn execute_five_success() {
        let tp = ThreadPool::new(3);
        tp.execute(|| println!("First (#1) job done!"));
        tp.execute(|| println!("Second (#2) job done!"));
        tp.execute(|| println!("Third (#3) job done!"));
        tp.execute(|| println!("Fourth (#4) job done!"));
        tp.execute(|| println!("Fifth (#5) job done!"));
    }
}
