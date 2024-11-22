use clap::Command;
use core::monitor::sys_observer::{Info, Monitor};
use once_cell::sync::Lazy;

struct CliComand {
    cmd: Command,
    name: &'static str,
    execute: Box<dyn Fn()>,
}

impl CliComand {
    fn new(name: &'static str, about: &'static str, execute: Box<dyn Fn()>) -> Self {
        let cmd = Command::new(name).about(about);
        Self { cmd, name, execute }
    }
}

struct Cli {
    items: Vec<CliComand>,
}

impl Cli {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn register(&mut self, cmd: CliComand) -> &mut Self {
        self.items.push(cmd);
        return self;
    }

    pub fn build(&self) -> Command {
        let mut main_cmd = Command::new("main").about("Main command");

        for sub_command in &self.items {
            main_cmd = main_cmd.subcommand(sub_command.cmd.clone());
        }

        main_cmd
    }

    fn get_command_by_name(&self, name: &str) -> Option<&CliComand> {
        for item in &self.items {
            if item.name == name {
                return Some(item);
            }
        }
        None
    }
}

static MONITOR: Lazy<Monitor> = Lazy::new(|| Monitor::new());

fn init_cli() -> Cli {
    let mut cli = Cli::new();

    let ram_usage = CliComand::new(
        "ram-usage",
        "A tool to monitor RAM usage",
        Box::new(|| {
            MONITOR.display_info(Info::RamUsage);
        }),
    );

    let list_process = CliComand::new(
        "top-process",
        "Show most used RAM process",
        Box::new(|| {
            MONITOR.display_info(Info::TopProcess);
        }),
    );

    cli.register(ram_usage).register(list_process);
    cli
}

pub fn run() {
    let cli = init_cli();
    let cmd = cli.build();

    let matches = cmd.get_matches();

    if let Some(subcomand_name) = matches.subcommand_name() {
        if let Some(sub_command) = cli.get_command_by_name(subcomand_name) {
            (sub_command.execute)();
        }
    }
}
