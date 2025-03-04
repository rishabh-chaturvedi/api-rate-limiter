use api_rate_limiter::limiter::RateLimiter;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn test_rate_limiter_basic() {
    let limiter = RateLimiter::new(3, Duration::from_secs(1));
    let mut limiter = limiter.lock().unwrap();

    assert!(limiter.allow());
    assert!(limiter.allow());
    assert!(limiter.allow());
    assert!(!limiter.allow()); // Limit reached

    thread::sleep(Duration::from_secs(1));
    assert!(limiter.allow()); // Token refilled
}

#[test]
fn test_rate_limiter_zero_capacity() {
    let limiter = RateLimiter::new(0, Duration::from_secs(1));
    let mut limiter = limiter.lock().unwrap();

    assert!(!limiter.allow()); // No requests should be allowed
}

#[test]
fn test_partial_refill() {
    let limiter = RateLimiter::new(5, Duration::from_secs(3));
    let mut limiter = limiter.lock().unwrap();

    for _ in 0..5 {
        assert!(limiter.allow());
    }

    assert!(!limiter.allow()); // Exhausted

    thread::sleep(Duration::from_secs(1));

    assert!(limiter.allow()); // Not fully refilled yet

    thread::sleep(Duration::from_secs(1));

    assert!(limiter.allow()); // Now it should be refilled
}

#[test]
fn test_concurrent_access() {
    let limiter = Arc::new(RateLimiter::new(10, Duration::from_secs(1)));
    let mut handles = vec![];

    for _ in 0..5 {
        let limiter = Arc::clone(&limiter);
        handles.push(thread::spawn(move || {
            let mut limiter = limiter.lock().unwrap();
            assert!(limiter.allow());
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_large_capacity() {
    let limiter = RateLimiter::new(500000, Duration::from_secs(2));
    let mut limiter = limiter.lock().unwrap();

    for _ in 0..500_000 {
        assert!(limiter.allow());
    }

    thread::sleep(Duration::from_secs(1));

    for _ in 0..250_000 {
        assert!(limiter.allow()); // Would be reclaimed after 1 secs
    }

    thread::sleep(Duration::from_secs(1));

    assert!(limiter.allow()); // Token refilled
}
