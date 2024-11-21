use sysinfo::{ProcessExt, System, SystemExt};

pub struct Monitor {
    system: System,
}

pub enum Info {
    RamUsage,
    TopProcess,
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self) {
        self.system.refresh_all();
    }

    pub fn ram_usage(&self) -> (u64, u64) {
        (self.system.used_memory(), self.system.total_memory())
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
                print!("used_memory: {}, total_memory: {}", result.0, result.1)
            }
            Info::TopProcess => {
                let result = self
                    .top_process()
                    .map(|(name, memory)| format!("Process: {}, Memory usage: {} KB", name, memory))
                    .unwrap_or("No process found".to_string());
                print!("process: {}", result)
            }
        }
    }
}
