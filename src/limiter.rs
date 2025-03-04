use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct RateLimiter {
    capacity: u32,
    tokens: u32,
    last_refill: Instant,
    refill_rate: Duration,
}

impl RateLimiter {
    pub fn new(capacity: u32, refill_rate: Duration) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            capacity,
            tokens: capacity,
            last_refill: Instant::now(),
            refill_rate,
        }))
    }

    pub fn allow(&mut self) -> bool {
        self.refill();

        if self.tokens > 0 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let time_elapsed = now.duration_since(self.last_refill);

        let new_tokens = (time_elapsed.as_secs_f64() / self.refill_rate.as_secs_f64()) * self.capacity as f64;
        self.tokens = (self.tokens as f64 + new_tokens).min(self.capacity as f64) as u32;
        self.last_refill = now;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new(5, Duration::from_secs(1));
        let mut limiter = limiter.lock().unwrap();

        for _ in 0..5 {
            assert!(limiter.allow());
        }

        assert!(!limiter.allow());

        sleep(Duration::from_secs(1));

        assert!(limiter.allow());
    }
}