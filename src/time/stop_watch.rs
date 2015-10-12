

extern crate std;

#[derive(Debug, Clone, Copy)]
struct StopWatchEntry {
    start_time: f64,
    end_time: f64,
    running: bool,
}

#[derive(Debug, Clone)]
pub struct StopWatch {
    clock: super::HighResolutionClock,
    processes: std::collections::HashMap<String, StopWatchEntry>,
}

impl StopWatch {
    pub fn new() -> StopWatch {
        StopWatch {
            clock: super::HighResolutionClock::new(),
            processes: std::collections::HashMap::new(),
        }
    }

    pub fn start_process(&mut self, name: String) {
        let entry = StopWatchEntry {
            start_time: self.clock.seconds_from_start(),
            end_time: 0.0,
            running: true,
        };

        self.processes.insert(name, entry);
    }

    pub fn stop_process(&mut self, name: &str) {
        if let Some(entry) = self.processes.get_mut(name) {
            if entry.running {
                entry.end_time = self.clock.seconds_from_start();
                entry.running = false;
            }
        }
    }

    pub fn stop_all_processes(&mut self) {
        for (_, entry) in self.processes.iter_mut() {
            if entry.running {
                entry.end_time = self.clock.seconds_from_start();
                entry.running = false;
            }
        }
    }
}

impl std::fmt::Display for StopWatch {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for (name, entry) in self.processes.iter() {
            if !entry.running {
                let diff = entry.end_time - entry.start_time;
                try!(
                    writeln!(fmt, "{}: {} seconds", name, diff)
                );
            } else {
                let diff = self.clock.seconds_from_start() - entry.start_time;
                try!(
                    writeln!(fmt, "{}: {} seconds (still running)", name, diff)
                );
            }
        }

        Ok(())
    }
}