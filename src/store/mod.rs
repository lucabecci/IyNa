pub mod lfu;
pub mod lru;
pub mod types;

pub trait Store {
    fn get(&mut self, key: &CacheKey) -> Result<$CacheEntry>;
    fn put(&mut self, key: CacheKey, value: CacheEntry);
    fn remove(&mut self, key: &CacheKey);
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
    fn is_full(&self) -> bool;
}
