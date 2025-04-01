use crate::store::types::{CacheEntry, CacheKey};
use crate::store::Store;
use std::collections::{HashMap, VecDeque};

pub struct LruCache {
    capacity: usize,
    store: HashMap<CacheKey, CacheEntry>,
    usage_order: VecDeque<CacheKey>,
}

impl LruCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            store: HashMap::new(),
            usage_order: VecDeque::new(),
        }
    }

    fn touch(&mut self, key: &CacheKey) {
        if let Some(pos) = self.usage_order.iter().position(|k| k == key) {
            self.usage_order.remove(pos);
        }
        self.usage_order.push_front(key.clone());
    }

    //O(1)
    fn evict_if_needed(&mut self) {
        if self.store.len() >= self.capacity {
            if let Some(least_used) = self.usage_order.pop_back() {
                self.store.remove(&least_used);
            }
        }
    }
}

impl Store for LruCache {
    fn get(&mut self, key: &CacheKey) -> Option<&CacheEntry> {
        //O(n)
        if let Some(entry) = self.store.get(key) {
            if entry.is_expired() {
                self.remove(key);
                return None;
            }
            self.touch(key);
            return self.store.get(key);
        }
        return None;
    }

    //O(n)
    fn put(&mut self, key: CacheKey, value: CacheEntry) {
        self.evict_if_needed();
        self.store.insert(key.clone(), value);
        self.touch(&key);
    }

    fn remove(&mut self, key: &CacheKey) {
        self.store.remove(key);
        if let Some(pos) = self.usage_order.iter().position(|k| k == key) {
            self.usage_order.remove(pos);
        }
    }

    fn len(&self) -> usize {
        self.store.len()
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn is_full(&self) -> bool {
        self.store.len() >= self.capacity
    }
}
