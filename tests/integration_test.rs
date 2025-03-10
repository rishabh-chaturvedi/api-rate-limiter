use std::sync::Arc;
use std::time::Duration;
use std::thread;
use api_rate_limiter::limiter::RateLimiter;
use api_rate_limiter::cache::in_memory::InMemoryCache;

#[test]
fn test_rate_limiter_basic() {
    // Create an in-memory cache instance.
    let cache = Arc::new(InMemoryCache::new());
    // Create a rate limiter allowing 3 requests per 1-second window.
    let limiter = RateLimiter::new(cache, 3, Duration::from_secs(1));

    // For the IP "127.0.0.1", the first 3 requests should be allowed.
    assert!(limiter.allow("127.0.0.1"));
    assert!(limiter.allow("127.0.0.1"));
    assert!(limiter.allow("127.0.0.1"));
    // 4th request should be blocked.
    assert!(!limiter.allow("127.0.0.1"));

    // After waiting for TTL to expire, requests should be allowed again.
    thread::sleep(Duration::from_secs(1));
    assert!(limiter.allow("127.0.0.1"));
}

#[test]
fn test_rate_limiter_zero_capacity() {
    let cache = Arc::new(InMemoryCache::new());
    // Create a rate limiter with zero capacity.
    let limiter = RateLimiter::new(cache, 0, Duration::from_secs(1));

    // With zero capacity, all requests should be blocked.
    assert!(!limiter.allow("127.0.0.1"));
}

#[test]
fn test_partial_refill() {
    let cache = Arc::new(InMemoryCache::new());
    // Create a rate limiter with 5 requests per 3-second window.
    let limiter = RateLimiter::new(cache, 5, Duration::from_secs(3));

    // Use the IP "127.0.0.1".
    for _ in 0..5 {
        assert!(limiter.allow("127.0.0.1"));
    }
    // Limit reached.
    assert!(!limiter.allow("127.0.0.1"));

    // Wait for 1 second (TTL not expired yet).
    thread::sleep(Duration::from_secs(1));
    // Still blocked.
    assert!(!limiter.allow("127.0.0.1"));

    // Wait for an additional 2 seconds (total 3 sec, TTL expired).
    thread::sleep(Duration::from_secs(2));
    // Now, the rate limiter should allow requests again.
    assert!(limiter.allow("127.0.0.1"));
}

#[test]
fn test_concurrent_access() {
    let cache = Arc::new(InMemoryCache::new());
    // Create a rate limiter allowing 10 requests per second.
    let limiter = Arc::new(RateLimiter::new(cache, 10, Duration::from_secs(1)));
    let mut handles = vec![];

    // Spawn 5 threads, each making a request from the same IP.
    for _ in 0..5 {
        let limiter_clone = Arc::clone(&limiter);
        handles.push(thread::spawn(move || {
            assert!(limiter_clone.allow("127.0.0.1"));
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_large_capacity() {
    let cache = Arc::new(InMemoryCache::new());
    // Create a rate limiter with a high capacity (500,000 requests) over a 2-second window.
    let limiter = RateLimiter::new(cache, 500_000, Duration::from_secs(4));

    // Issue 500,000 requests from the IP "127.0.0.1".
    for _ in 0..500_000 {
        assert!(limiter.allow("127.0.0.1"));
    }

    // Additional request should be blocked.
    assert!(!limiter.allow("127.0.0.1"));

    // Wait for 1 second (TTL not yet fully expired).
    thread::sleep(Duration::from_secs(1));
    // Still blocked.
    assert!(!limiter.allow("127.0.0.1"));

    // Wait for the TTL to fully expire.
    thread::sleep(Duration::from_secs(5));
    // Now the counter resets and a new request is allowed.
    assert!(limiter.allow("127.0.0.1"));
}
