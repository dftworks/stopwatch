use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
pub struct Stopwatch {
    pub tag:     String,
    pub start:   SystemTime,
    pub stop:    SystemTime,
    pub elapsed: Duration,
}

impl Stopwatch {
    pub fn new(tag: &str) -> Stopwatch {
        Stopwatch { tag:     tag.to_string(),
                    start:   SystemTime::now(),
                    stop:    SystemTime::now(),
                    elapsed: Duration::new(0, 0), }
    }

    pub fn reset(&mut self) {
        self.start = SystemTime::now();
    }

    pub fn stop(&mut self) {
        self.stop = SystemTime::now();
        self.elapsed =
            self.elapsed + self.stop.duration_since(self.start).unwrap();
    }

    pub fn elapsed_as_seconds(&self) -> f64 {
        self.elapsed.as_secs_f64()
    }
}

#[test]
fn test_stopwatch() {
    let mut timer = Stopwatch::new("Test");
    std::thread::sleep(std::time::Duration::new(2, 0));
    timer.stop();
    println!("Timer {} : {}", timer.tag, timer.elapsed_as_seconds());
}
