use std::time::{Duration, Instant};
use dashmap::DashMap;
use crate::limiter::CacheBackend;

#[derive(Debug)]
struct CacheEntry {
    value: u32,
    expires_at: Instant,
}

/// An in-memory cache implementation of the `CacheBackend` trait.
/// It uses a concurrent DashMap to store keys with their expiration.
pub struct InMemoryCache {
    store: DashMap<String, CacheEntry>,
}

impl InMemoryCache {
    /// Creates a new in-memory cache instance.
    pub fn new() -> Self {
        InMemoryCache {
            store: DashMap::new(),
        }
    }
}

impl CacheBackend for InMemoryCache {
    fn get(&self, key: &str) -> Option<u32> {
        if let Some(entry) = self.store.get(key) {
            if entry.expires_at > Instant::now() {
                // println!("Returning the current entry");
                return Some(entry.value);
            } else {
                // Expired: remove the entry.
                // println!("removing the current entry and returning None");
                drop(entry);
                self.store.remove(key);
                // println!("removed the current entry and returning None");
                return None;
            }
        } else {
            // println!("no entry found and returning None");
            return None;
        }
    }

    fn set(&self, key: &str, value: u32, ttl: Duration) -> Result<(), String> {
        let expires_at = Instant::now() + ttl;
        let entry = CacheEntry { value, expires_at };
        self.store.insert(key.to_string(), entry);
        Ok(())
    }

    fn incr(&self, key: &str, amount: u32) -> Result<u32, String> {
        let now = Instant::now();
        if let Some(mut entry) = self.store.get_mut(key) {
            if entry.expires_at <= now {
                // If the entry is expired, reset it.
                entry.value = amount;
            } else {
                entry.value += amount;
            }
            Ok(entry.value)
        } else {
            // Insert a new entry. The TTL will be set by the caller if needed.
            self.store.insert(key.to_string(), CacheEntry {
                value: amount,
                expires_at: now, // Temporary; caller should update TTL with `set`.
            });
            Ok(amount)
        }
    }
}
