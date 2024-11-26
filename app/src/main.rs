use core::monitor::memory_watcher::{watcher, Config, Warning};
use core::monitor::sys_observer::Monitor;
use signal_hook::consts::signal::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let monitor = Monitor::new();
    let mut config = Config::new();
    config.add_warning(Warning::new(20.0, "hello 20.0"));

    let stop = Arc::new(AtomicBool::new(false));
    let stop_clone = stop.clone();
    let mut signals = Signals::new(&[SIGINT, SIGTERM]).expect("Failed to setup signal handler");

    let signal_thread = thread::spawn(move || {
        for signal in signals.forever() {
            match signal {
                SIGTERM => {
                    println!("sigterm");
                    stop_clone.store(true, Ordering::SeqCst);
                    break;
                }
                SIGINT => {
                    println!("sigint");
                    stop_clone.store(true, Ordering::SeqCst);
                    break;
                }
                _ => unreachable!(),
            }
        }
    });

    let watcher_thread = thread::spawn(move || {
        watcher(monitor, config, stop);
    });

    signal_thread.join().unwrap();
    watcher_thread.join().unwrap();

    println!("Monitoring stopped gracefully.");
}
