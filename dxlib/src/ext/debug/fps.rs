use std::{collections::VecDeque, time};

pub struct Fps {
    timer: time::Instant,
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
            history,
            capacity,
        }
    }

    pub fn update(&mut self) -> f64 {
        let now = self.timer.elapsed().as_nanos();
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
