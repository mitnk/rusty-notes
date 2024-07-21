use lazy_static::lazy_static;
use std::time::Duration;
use std::sync::Mutex;
use ttl_cache::TtlCache;

lazy_static! {
    static ref CACHE: Mutex<TtlCache<String, String>> = {
        Mutex::new(TtlCache::new(1000))
    };
}

pub fn cache_delete(k: &str) {
    CACHE.lock().unwrap().remove(k);
}

pub fn cache_get(k: &str) -> Option<String> {
    let cache = CACHE.lock().unwrap();
    if let Some(v) = cache.get(k) {
        return Some(v.clone());
    }
    None
}

pub fn cache_set_1h(k: &str, v: &str) {
    CACHE.lock().unwrap().insert(k.to_string(), v.to_string(), Duration::from_secs(3600));
}
