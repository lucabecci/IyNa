use crate::store::types::{CacheEntry, CacheKey};

pub mod lfu;
pub mod lru;
pub mod types;

pub trait Store {
    fn get(&mut self, key: &CacheKey) -> Option<&CacheEntry>;
    fn put(&mut self, key: CacheKey, value: CacheEntry);
    fn remove(&mut self, key: &CacheKey);
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
    fn is_full(&self) -> bool;
}
