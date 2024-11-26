use super::sys_observer::Monitor;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct Warning {
    usage: f64,
    message: String,
}

impl Warning {
    pub fn new(usage: f64, message: &str) -> Self {
        Self {
            usage,
            message: message.to_string(),
        }
    }
}

pub struct Config {
    warnings: Vec<Warning>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            warnings: Vec::new(),
        }
    }

    pub fn add_warning(&mut self, warning: Warning) {
        self.warnings.push(warning);
    }
}

fn check_for_warnigs(current_usage: &f64, threshold: &[Warning]) -> Option<Warning> {
    threshold
        .iter()
        .filter(|warning| warning.usage <= *current_usage)
        .max_by(|a, b| {
            a.usage
                .partial_cmp(&b.usage)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
}

pub fn watcher(mut monitor: Monitor, config: Config, stop: Arc<AtomicBool>) {
    while !stop.load(Ordering::SeqCst) {
        monitor.update();
        let usage = monitor.ram_consumed();
        if let Some(warning) = check_for_warnigs(&usage, &config.warnings) {
            println!(
                "You already used {} of your memory. {}",
                usage, warning.message
            );
        }

        thread::sleep(Duration::from_secs(1));
    }
}
