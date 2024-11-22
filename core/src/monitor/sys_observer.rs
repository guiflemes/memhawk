use sysinfo::{ProcessExt, System, SystemExt};

pub struct Monitor {
    system: System,
}

pub enum Info {
    RamUsage,
    TopProcess,
}

const MB: u64 = 1_048_576; // 1 MB = 1024 * 1024 bytes

impl Monitor {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }

    pub fn update(&mut self) {
        self.system.refresh_all();
    }

    pub fn ram_usage(&self) -> (u64, u64) {
        (self.system.used_memory(), self.system.total_memory())
    }

    pub fn ram_consumed(&self) -> f64 {
        let (used, total) = self.ram_usage();
        return (used as f64 * 100.0) / total as f64;
    }

    pub fn top_process(&self) -> Option<(String, u64)> {
        return self
            .system
            .processes()
            .values()
            .max_by_key(|p| p.memory())
            .map(|p| (p.name().to_string(), p.memory()));
    }

    pub fn display_info(&self, info: Info) {
        match info {
            Info::RamUsage => {
                let result = self.ram_usage();
                print!(
                    "used_memory: {} MB, total_memory: {} MB",
                    result.0 / MB,
                    result.1 / MB
                )
            }
            Info::TopProcess => {
                let result = self
                    .top_process()
                    .map(|(name, memory)| {
                        format!("Process: {}, Memory usage: {} MB", name, memory / MB)
                    })
                    .unwrap_or("No process found".to_string());
                print!("process: {}", result)
            }
        }
    }
}

#[derive(Clone)]
pub struct Warning {
    usage: f64,
    message: String,
}

pub struct Config {
    warnings: Vec<Warning>,
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

pub fn monitor_memory(mut monitor: Monitor, config: Config) {
    loop {
        monitor.update();
        let usage = monitor.ram_consumed();
        if let Some(warning) = check_for_warnigs(&usage, &config.warnings) {
            println!(
                "You already used {} of your memory. {}",
                usage, warning.message
            );
        }
    }
}
