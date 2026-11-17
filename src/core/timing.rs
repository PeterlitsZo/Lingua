use std::time::{Duration, Instant};

pub(crate) struct Timings {
    start: Option<Instant>,
    stages: Vec<(&'static str, Duration)>,
    pub(crate) cache_status: &'static str,
}

impl Timings {
    pub(crate) fn new(enabled: bool, start: Instant) -> Self {
        let mut timings = Self {
            start: enabled.then_some(start),
            stages: Vec::new(),
            cache_status: "not attempted",
        };
        if enabled {
            timings.stages.push(("CLI initialization", start.elapsed()));
        }
        timings
    }

    pub(crate) fn measure<T>(&mut self, name: &'static str, work: impl FnOnce() -> T) -> T {
        let start = self.start.map(|_| Instant::now());
        let result = work();
        if let Some(start) = start {
            self.stages.push((name, start.elapsed()));
        }
        result
    }

    pub(crate) fn report(&self) {
        let Some(start) = self.start else { return };
        let total = start.elapsed();
        eprintln!("Timings (cache: {}):", self.cache_status);
        for name in [
            "CLI initialization",
            "Path validation",
            "Cache lookup",
            "Cache read",
            "Cache decode",
            "Source read",
            "Parse",
            "Compile",
            "Cache write",
            "Execute",
            "Output write",
        ] {
            if let Some((_, elapsed)) = self.stages.iter().find(|(stage, _)| *stage == name) {
                eprintln!("  {name:<20} {:>12.6} ms", elapsed.as_secs_f64() * 1000.0);
            } else {
                eprintln!("  {name:<20}      skipped");
            }
        }
        eprintln!(
            "  {:<20} {:>12.6} ms",
            "Total",
            total.as_secs_f64() * 1000.0
        );
    }
}
