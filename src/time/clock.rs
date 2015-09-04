#[derive(Debug, Clone, Copy)]
pub struct HighResolutionClock {
    ticks_per_second: i64,
    ticks_at_start: i64,
    fticks_per_second: f64,
}

impl HighResolutionClock {
    pub fn new() -> HighResolutionClock {
        let mut ticks_per_second: i64 = 0;
        let mut ticks_at_start = 0;
        unsafe {
            QueryPerformanceFrequency(&mut ticks_per_second);
            QueryPerformanceCounter(&mut ticks_at_start);
        }

        HighResolutionClock {
            ticks_per_second: ticks_per_second,
            ticks_at_start: ticks_at_start,
            fticks_per_second: ticks_per_second as f64,
        }
    }

    pub fn raw_ticks(&self) -> i64 {
        let mut count: i64 = 0;
        unsafe { QueryPerformanceCounter(&mut count); }
        count
    }

    pub fn ticks_from_start(&self) -> i64 {
        self.raw_ticks() - self.ticks_at_start
    }

    pub fn seconds_from_start(&self) -> f64 {
        (self.ticks_from_start() as f64) / self.fticks_per_second
    }
}

#[link(name = "kernel32")]
extern {
    fn QueryPerformanceCounter(pcount: *mut i64) -> i32;
    fn QueryPerformanceFrequency(pfreq: *mut i64) -> i32;
}