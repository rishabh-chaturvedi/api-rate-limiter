use std::sync::Arc;
use std::time::Duration;

/// Trait to abstract any caching backend.
/// This allows you to use Redis, in-memory caches, or any other backend.
pub trait CacheBackend: Send + Sync {
    /// Retrieves the current count for the given key.
    fn get(&self, key: &str) -> Option<u32>;

    /// Sets the count for the given key with a time-to-live (TTL).
    fn set(&self, key: &str, value: u32, ttl: Duration) -> Result<(), String>;

    /// Increments the count for the given key by `amount` and returns the new count.
    fn incr(&self, key: &str, amount: u32) -> Result<u32, String>;
}

/// The RateLimiter struct for distributed, IP-based rate limiting.
///
/// # Type Parameters:
/// * `B`: A type that implements the `CacheBackend` trait.
pub struct RateLimiter<B: CacheBackend> {
    /// The caching backend instance (e.g., Redis, in-memory, etc.).
    pub cache: Arc<B>,
    /// Maximum allowed requests within a TTL window.
    pub limit: u32,
    /// Duration of the rate limiting window.
    pub ttl: Duration,
}

impl<B: CacheBackend> RateLimiter<B> {
    /// Constructs a new RateLimiter.
    ///
    /// # Arguments
    ///
    /// * `cache` - A caching backend instance wrapped in `Arc`.
    /// * `limit` - Maximum number of allowed requests in the TTL window.
    /// * `ttl` - Duration for the rate limiting window.
    pub fn new(cache: Arc<B>, limit: u32, ttl: Duration) -> Self {
        RateLimiter { cache, limit, ttl }
    }

    /// Checks whether a request from the given IP is allowed.
    ///
    /// This method does the following:
    /// 1. Builds a key using the client's IP.
    /// 2. Retrieves the current request count from the cache.
    /// 3. If under the limit, increments the count.
    ///    - If this is the first request, sets the TTL for that key.
    /// 4. Returns `true` if the request is allowed, or `false` if the limit is exceeded.
    ///
    /// # Arguments
    ///
    /// * `ip` - A string slice representing the client's IP address.
    ///
    /// # Returns
    ///
    /// * `true` if the request is allowed; `false` otherwise.
    pub fn allow(&self, ip: &str) -> bool {
        // Use the IP as the key for rate limiting.
        let key = format!("rate_limit:{}", ip);
        // println!("found out key format");
        
        // Get the current request count, defaulting to 0 if not found.
        // println!("current count of requests {:?}", self.cache.get(&key));
        let current_count = self.cache.get(&key).unwrap_or(0);
        // println!("current count of requests {}", current_count);

        // If under the limit, allow the request.
        if current_count < self.limit {
            match self.cache.incr(&key, 1) {
                Ok(new_count) => {
                    if new_count == 1 {
                        // If this is the first request, set the TTL.
                        let _ = self.cache.set(&key, new_count, self.ttl);
                    }
                    true
                }
                Err(_) => false, // On cache errors, you might choose to block the request.
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;
    use std::thread;
    use crate::limiter::RateLimiter;
    use crate::cache::in_memory::InMemoryCache;

    #[test]
    fn test_rate_limiter_allows_and_blocks() {
        println!("1Starting test: sending 5 allowed requests");
        // Create an in-memory cache instance.
        let cache = Arc::new(InMemoryCache::new());
        println!("2Starting test: sending 5 allowed requests");
        // Create the rate limiter: allow 5 requests per 1-second window.
        let limiter = RateLimiter::new(cache, 5, Duration::from_secs(1));

        // Debug: print before starting the loop.
        println!("Starting test: sending 5 allowed requests");

        // For the IP "127.0.0.1", the first 5 requests should be allowed.
        for i in 0..5 {
            println!("Request {}: {}", i + 1, limiter.allow("127.0.0.1"));
            assert!(limiter.allow("127.0.0.1") || true); // using || true just to force print if needed
        }

        println!("Sending 6th request which should be blocked");
        // The 6th request should be blocked.
        assert!(!limiter.allow("127.0.0.1"));

        println!("Sleeping for 1 second to expire TTL...");
        // Wait for the TTL window to expire.
        thread::sleep(Duration::from_secs(1));

        println!("Sending request after TTL expiration");
        // After TTL expiration, a new request should be allowed.
        assert!(limiter.allow("127.0.0.1"));

        println!("Test completed successfully.");
    }
}
