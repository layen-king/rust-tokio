use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Once, RwLock};
use std::thread;

static INIT_ONCE: Once = Once::new();

static GLOBAL_MAP: Lazy<Arc<RwLock<HashMap<String, i32>>>> = Lazy::new(|| {
    INIT_ONCE.call_once(|| {
        let map = Arc::new(RwLock::new(HashMap::new()));
        map.write().unwrap().insert("key1".to_string(), 10);
        map.write().unwrap().insert("key2".to_string(), 20);
    });
    Arc::new(RwLock::new(HashMap::new()))
});

pub fn insert(key: String, value: i32) {
    GLOBAL_MAP.write().unwrap().insert(key, value);
}

pub fn get(key: &str) -> Option<i32> {
    GLOBAL_MAP.read().unwrap().get(key).cloned()
}

fn main() {
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                let key = format!("key{}", i);
                let value = i as i32 * 5;
                insert(key, value);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    print!("{:?}", get("key1"));
}
