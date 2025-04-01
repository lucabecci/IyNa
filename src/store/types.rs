use std::time::{Duration, Instant};

pub type CacheKey = String;

#[derive(Clone, Debug)]
pub struct CacheEntry {
    pub value: String,
    pub created_at: Instant,
    pub ttl: Option<Duration>,
    pub frequency: usize, // used for LFU cache eviction
}

impl CacheEntry {
    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl {
            self.created_at.elapsed() > ttl
        } else {
            false
        }
    }
}
