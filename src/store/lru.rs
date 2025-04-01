use crate::store::types::{CacheEntry, CacheKey};
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

    fn evict_if_needed(&mut self) {
        if self.store.len() >= self.capacity {
            if let Some(least_used) = self.usage_order.pop_back() {
                self.store.remove(&least_used);
            }
        }
    }
}
