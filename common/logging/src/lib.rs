/*
Last updated: 11-15-2023

Description: This crate defines the logging class

Author: James Dean
*/
use log::{Record, Level, Metadata, LevelFilter};
use chrono::Local;
use std::sync::Mutex;

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now = Local::now();
            println!("{} - [{}] {}", now.format("%Y-%m-%d %H:%M:%S"), record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;
static LOGGER_INIT: Mutex<()> = Mutex::new(());

pub fn init() {
    let _lock_guard = LOGGER_INIT.lock().unwrap(); // Keep the lock for the scope of this block
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("Failed to initialize logger");
    // The lock is released here when _lock_guard goes out of scope
}
