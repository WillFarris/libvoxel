use std::{thread::JoinHandle, sync::{Mutex, Arc}};


pub struct ChunkWorker<T> {
    handle: JoinHandle<T>
}

impl<T> ChunkWorker<T> {
    fn new(handle: JoinHandle<T>) -> Self {
        Self {
            handle
        }
    }
}