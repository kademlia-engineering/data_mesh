/*
Last updated: 11-16-2023

Description: This crate defines a thread safe interface for file i/o

Author: James Dean
*/
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use std::path::Path;

pub struct ThreadSafeFileIO {
    file: Arc<Mutex<File>>,
}

impl ThreadSafeFileIO {
    // Initializes a new ThreadSafeFileIO with the given file path
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<ThreadSafeFileIO> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        Ok(ThreadSafeFileIO {
            file: Arc::new(Mutex::new(file)),
        })
    }

    // Writes data to the file in a thread-safe manner
    pub fn write(&self, data: &[u8]) -> io::Result<()> {
        let mut file = self.file.lock().unwrap();
        file.write_all(data)?;
        Ok(())
    }

    // Reads data from the file in a thread-safe manner
    pub fn read(&self) -> io::Result<Vec<u8>> {
        let mut file = self.file.lock().unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}
