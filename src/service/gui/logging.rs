use log::{LevelFilter, Log, Metadata, Record};
use std::sync::OnceLock;
use tokio::sync::mpsc;

// This is the global "pipe" to the GUI
static LOG_SENDER: OnceLock<mpsc::UnboundedSender<String>> = OnceLock::new();

struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!("[{}] {}", record.level(), record.args());

            println!("{}", msg);

            if let Some(sender) = LOG_SENDER.get() {
                let _ = sender.send(msg);
            }
        }
    }
    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

pub fn init() -> mpsc::UnboundedReceiver<String> {
    let (tx, rx) = mpsc::unbounded_channel();

    let _ = LOG_SENDER.set(tx);

    log::set_logger(&LOGGER).expect("Failed to set logger");
    log::set_max_level(LevelFilter::Trace);

    rx
}
