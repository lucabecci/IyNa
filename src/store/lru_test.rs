mod lru_test {
    use crate::store::lru::LruCache;
    use crate::store::{CacheEntry, Store};
    use std::time::{Duration, Instant};
    fn make_entry(value: &str, ttl: Option<Duration>) -> CacheEntry {
        CacheEntry {
            value: value.to_string(),
            created_at: Instant::now(),
            ttl,
            frequency: 0,
        }
    }

    #[test]
    fn test_insert_and_get() {
        let mut cache = LruCache::new(1);
        let key = "key1".to_string();
        let value = "v1";
        let entry = make_entry(value.clone(), None);

        cache.put(key.clone(), entry.clone());
        let result = cache.get(&key);
        assert!(result.is_some());
        assert_eq!(result.unwrap().value, value);
    }

    #[test]
    fn test_eviction_least_recently_used() {
        let mut cache = LruCache::new(2);
        cache.put("a".into(), make_entry("1", None));
        cache.put("b".into(), make_entry("2", None));
        cache.get(&"a".into());
        cache.put("c".into(), make_entry("3", None));

        assert!(cache.get(&"a".into()).is_some());
        assert!(cache.get(&"b".into()).is_none());
        assert!(cache.get(&"c".into()).is_some());
    }

    #[test]
    fn test_eviction_multiple_access() {
        let mut cache = LruCache::new(2);
        cache.put("a".into(), make_entry("1", None));
        cache.put("b".into(), make_entry("2", None));

        cache.get(&"a".into());
        cache.get(&"a".into());

        cache.put("c".into(), make_entry("3", None));

        assert!(cache.get(&"a".into()).is_some());
        assert!(cache.get(&"c".into()).is_some());
        assert!(cache.get(&"b".into()).is_none());
    }

    #[test]
    fn test_multiple_put_eviction(){
        let mut cache = LruCache::new(2);
        cache.put("a".into(), make_entry("1", None));
        cache.put("b".into(), make_entry("2", None));
        let first_value = cache.get(&"a".into());
        assert!(first_value.is_some());
        assert_eq!(first_value.unwrap().value, "1");
        assert!(cache.get(&"b".into()).is_some());

        cache.put("c".into(), make_entry("3", None));
        cache.put("a".into(), make_entry("-1", None));

        let second_value = cache.get(&"a".into());
        assert_eq!(second_value.unwrap().value, "-1");
        assert!(cache.get(&"b".into()).is_none());
        assert!(cache.get(&"a".into()).is_some());
        assert!(cache.get(&"c".into()).is_some());
    }

    #[test]
    fn test_delete() {
        let mut cache = LruCache::new(2);
        cache.put("a".into(), make_entry("1", None));
        assert!(cache.get(&"a".into()).is_some());

        cache.remove(&"a".into());
        assert!(cache.get(&"a".into()).is_none());
    }

    #[test]
    fn test_len(){
        let mut cache = LruCache::new(2);
        cache.put("a".into(), make_entry("1", None));
        cache.put("b".into(), make_entry("2", None));

        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_is_full(){
        let mut cache = LruCache::new(1);
        cache.put("a".into(), make_entry("1", None));
        assert!(cache.is_full())
    }
}
