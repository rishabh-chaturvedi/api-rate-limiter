use rate_limiter::limiter::RateLimiter;
use std::time::Duration;
use std::thread::sleep;

#[test]
fn test_rate_limiter() {
    let limiter = RateLimiter::new(3, Duration::from_secs(1));
    let mut limiter = limiter.lock().unwrap();

    assert!(limiter.allow());
    assert!(limiter.allow());
    assert!(limiter.allow());
    assert!(!limiter.allow()); // Should be empty now

    sleep(Duration::from_secs(1));
    assert!(limiter.allow()); // Refilled
}
