use std::sync::{Arc, Mutex};

pub struct CustomLogger {
    storage: Arc<Mutex<Vec<String>>>,
}

impl CustomLogger {
    pub fn new(storage: Arc<Mutex<Vec<String>>>) -> Self {
        Self { storage }
    }
}

impl log::Log for CustomLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let mut logs = self.storage.lock().unwrap();
            logs.push(format!("[{}] {}", record.level(), record.args()));
        }
    }

    fn flush(&self) {}
}
