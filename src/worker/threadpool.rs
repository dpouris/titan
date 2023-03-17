#![allow(unused)]
use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::{self, Duration},
};

use super::{Job, Worker};

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Sender<Job>,
}

// implement send for threadpool

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        if size == 0 {
            panic!("wrong size 0");
        }

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let mut threads: Vec<Worker> = Vec::with_capacity(size);

        for idx in 0..size {
            let rx = Arc::clone(&rx);
            let worker = Worker::new(idx, rx);
            threads.push(worker);
        }

        Self {
            threads,
            sender: tx,
        }
    }

    pub fn join_all(self) {
        drop(self.sender);
        for job in self.threads.into_iter() {
            job.thread.join().unwrap();
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + Sync + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job);
    }
}
