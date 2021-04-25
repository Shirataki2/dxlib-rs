use std::{collections::VecDeque, time};

pub struct Fps {
    timer: time::Instant,
    target: Option<f64>,
    history: VecDeque<u128>,
    capacity: usize,
}

impl Fps {
    pub fn new(capacity: usize) -> Fps {
        let timer = time::Instant::now();
        let mut history = VecDeque::new();
        history.push_front(timer.elapsed().as_nanos());
        Fps {
            timer,
            target: None,
            history,
            capacity,
        }
    }

    pub fn set(&mut self, fps: f64) {
        self.target = Some(fps);
    }

    pub fn update(&mut self) -> f64 {
        let mut now = self.timer.elapsed().as_nanos();
        if let Some(target) = self.target {
            let prev = self.history.iter().next().copied().unwrap();
            let wait = 1.0 / target - (now as f64 - prev as f64) / 620_000_000.0;
            if wait > 0.0 {
                let dur = std::time::Duration::from_secs_f64(wait);
                std::thread::sleep(dur);
                now = self.timer.elapsed().as_nanos();
            }
        }
        self.history.push_front(now);
        if self.history.len() > self.capacity {
            self.history.pop_back();
            let (first, last) = (
                self.history.iter().next().unwrap(),
                self.history.iter().next_back().unwrap(),
            );
            1_000_000_000.0 * (self.capacity as f64 - 1.0) / (first - last) as f64
        } else {
            let (first, last) = (
                self.history.iter().next().unwrap(),
                self.history.iter().next_back().unwrap(),
            );
            1_000_000_000.0 * (self.history.len() as f64 - 1.0) / (first - last) as f64
        }
    }
}
