# Rate Limiter

A simple and efficient rate limiter for Rust applications, designed to limit request rates in REST APIs. This crate implements a token bucket algorithm for controlling the flow of incoming requests.

## Features

- Thread-safe using `Arc<Mutex>`.
- Customizable request limits and refill rates.
- Lightweight and easy to integrate with any REST API.
- Supports synchronous rate limiting (can be extended to async).

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
api-rate-limiter = "0.1.1"
```

## Usage

```rust
use rate_limiter::limiter::RateLimiter;
use std::time::Duration;
use std::sync::Arc;
use std::thread;

fn main() {
    let limiter = RateLimiter::new(5, Duration::from_secs(1));
    let limiter = Arc::clone(&limiter);

    for _ in 0..10 {
        let mut limiter = limiter.lock().unwrap();
        if limiter.allow() {
            println!("Request allowed");
        } else {
            println!("Rate limit exceeded");
        }
        thread::sleep(Duration::from_millis(200));
    }
}
```

## API Reference

### `RateLimiter::new(capacity: u32, refill_rate: Duration) -> Arc<Mutex<RateLimiter>>`

Creates a new rate limiter instance.

- **`capacity`**: Maximum number of tokens (allowed requests).
- **`refill_rate`**: Duration to refill tokens.

### `allow(&mut self) -> bool`

Checks if a request is allowed. Returns `true` if allowed, `false` otherwise.

## Example Output

```
Request allowed
Request allowed
Request allowed
Request allowed
Request allowed
Rate limit exceeded
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
- [ ] IP-based rate limiting.
- [ ] Customizable backoff strategies.

## Contributing

Contributions are welcome! Feel free to open issues and submit pull requests.

1. Fork the repository.
2. Create a new branch (`feature/my-feature`).
3. Commit your changes.
4. Open a pull request.

## License

This project is licensed under the MIT License. See [LICENSE](license.md) for details.

