# Rate Limiter

A simple and efficient distributed rate limiter for Rust applications, designed to limit request rates in REST APIs. This crate implements an IP‑based rate limiting strategy using a pluggable cache backend architecture. By default, an in‑memory cache is provided, but you can easily integrate other caching solutions like Redis.

## Features

- **Pluggable Caching Backend:**  
  Use the built‑in in‑memory cache or plug in your own (e.g., Redis via `redis-rs`).
- **IP‑Based Rate Limiting:**  
  Rate limits are applied per client IP.
- **Distributed Design:**  
  Built to work in distributed environments via a shared cache.
- **High Performance:**  
  Uses concurrent data structures (e.g., DashMap) for fast, thread‑safe operations.
- **Synchronous API:**  
  Can be extended to support async (e.g., with Tokio).

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
api-rate-limiter = "0.1.2"
```

If you wish to use the in‑memory cache, no extra dependencies are required. For Redis support, add the appropriate Redis crate dependency in your own project and implement the `CacheBackend` trait accordingly.

## Usage

### Using the Built‑in In‑Memory Cache

Below is an example of how to instantiate the rate limiter with the built‑in in‑memory cache:

```rust
use std::sync::Arc;
use std::time::Duration;
use rate_limiter::limiter::RateLimiter;
use rate_limiter::cache::in_memory_cache::InMemoryCache;

fn main() {
    // Create an in-memory cache instance.
    let cache = Arc::new(InMemoryCache::new());
    // Create a rate limiter that allows 5 requests per 1-second window.
    let limiter = RateLimiter::new(cache, 5, Duration::from_secs(1));

    // Example usage for IP "127.0.0.1"
    if limiter.allow("127.0.0.1") {
        println!("Request allowed");
    } else {
        println!("Rate limit exceeded");
    }
}
```

### Using a Custom Cache Backend (e.g., Redis)

To use a different caching solution, implement the `CacheBackend` trait. For example, a Redis backend might look like this (implementation details are up to you):

```rust
use std::sync::Arc;
use std::time::Duration;
use rate_limiter::limiter::{RateLimiter, CacheBackend};

struct RedisBackend {
    // Redis connection details here
}

impl CacheBackend for RedisBackend {
    fn get(&self, key: &str) -> Option<u32> {
        // Implement retrieval from Redis
        unimplemented!()
    }

    fn set(&self, key: &str, value: u32, ttl: Duration) -> Result<(), String> {
        // Implement setting a value in Redis with TTL
        unimplemented!()
    }

    fn incr(&self, key: &str, amount: u32) -> Result<u32, String> {
        // Implement an atomic increment in Redis
        unimplemented!()
    }
}

fn main() {
    // Create a Redis backend instance (wrapped in Arc for shared access)
    let redis_backend = Arc::new(RedisBackend { /* connection details */ });
    // Create a rate limiter that allows 100 requests per minute.
    let limiter = RateLimiter::new(redis_backend, 100, Duration::from_secs(60));

    // Usage for a given IP.
    if limiter.allow("192.168.1.100") {
        println!("Request allowed");
    } else {
        println!("Rate limit exceeded");
    }
}
```

## API Reference

### `RateLimiter::new(cache: Arc<B>, limit: u32, ttl: Duration) -> RateLimiter<B>`

Creates a new rate limiter instance.

- **`cache`**: An instance of a type implementing `CacheBackend` (e.g., `InMemoryCache` or a custom Redis backend).
- **`limit`**: Maximum number of allowed requests within the TTL window.
- **`ttl`**: Duration of the rate limiting window.

### `allow(&self, ip: &str) -> bool`

Checks if a request from the specified IP is allowed.

- **`ip`**: The client's IP address used as the key for rate limiting.
- **Returns**: `true` if the request is allowed; `false` if the limit is exceeded.

## Example Output

```
Request allowed
Request allowed
Request allowed
Request allowed
Request allowed
Rate limit exceeded
Rate limit exceeded
...
```

## Running Tests

Ensure your environment is set up with `cargo` and run:

```bash
cargo test
```

## Roadmap

- [ ] Async support with `tokio`.
- [ ] More advanced distributed features (e.g., shared counters across instances).
- [ ] Customizable backoff and penalty strategies.

## Contributing

Contributions are welcome! Feel free to open issues and submit pull requests.

1. Fork the repository.
2. Create a new branch (`feature/my-feature`).
3. Commit your changes.
4. Open a pull request.

## License

This project is licensed under the MIT License. See [LICENSE](license.md) for details.
