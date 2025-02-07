use std::ffi::c_ulong;

pub struct ConcurrentHashMap {
    map: dashmap::DashMap<c_ulong, c_ulong>,
}

impl ConcurrentHashMap {
    fn new() -> Self {
        Self {
            map: dashmap::DashMap::new(),
        }
    }

    fn get(&self, key: c_ulong) -> Option<c_ulong> {
        self.map.get(&key).map(|v| *v)
    }

    fn set(&self, key: c_ulong, value: c_ulong) {
        self.map.insert(key, value);
    }

    fn clear(&self) {
        self.map.clear()
    }

    fn fetch_and_modify(&self, key: c_ulong, f: extern "C" fn(c_ulong) -> c_ulong) {
        self.map.alter(&key, |_, v| f(v));
    }

    fn mark(&self, f: extern "C" fn(c_ulong)) {
        for pair in self.map.iter() {
            f(*pair.key());
            f(*pair.value());
        }
    }
}

#[no_mangle]
pub extern "C" fn concurrent_hash_map_init(hashmap: *mut ConcurrentHashMap) {
    unsafe { hashmap.write(ConcurrentHashMap::new()) }
}

#[no_mangle]
pub extern "C" fn concurrent_hash_map_drop(hashmap: *mut ConcurrentHashMap) {
    unsafe { std::ptr::drop_in_place(hashmap) };
}

#[no_mangle]
pub extern "C" fn concurrent_hash_map_clear(hashmap: *const ConcurrentHashMap) {
    let hashmap = unsafe { hashmap.as_ref().unwrap() };
    hashmap.clear();
}

#[no_mangle]
pub extern "C" fn concurrent_hash_map_get(
    hashmap: *const ConcurrentHashMap,
    key: c_ulong,
    fallback: c_ulong,
) -> c_ulong {
    let hashmap = unsafe { hashmap.as_ref().unwrap() };
    hashmap.get(key).unwrap_or(fallback)
}

#[no_mangle]
pub extern "C" fn concurrent_hash_map_set(
    hashmap: *const ConcurrentHashMap,
    key: c_ulong,
    value: c_ulong,
) {
    let hashmap = unsafe { hashmap.as_ref().unwrap() };
    hashmap.set(key, value);
}

#[no_mangle]
pub extern "C" fn concurrent_hash_map_mark(
    hashmap: *const ConcurrentHashMap,
    f: extern "C" fn(c_ulong),
) {
    let hashmap = unsafe { hashmap.as_ref().unwrap() };
    hashmap.mark(f);
}

#[no_mangle]
pub extern "C" fn concurrent_hash_map_fetch_and_modify(
    hashmap: *const ConcurrentHashMap,
    key: c_ulong,
    f: extern "C" fn(c_ulong) -> c_ulong,
) {
    let hashmap = unsafe { hashmap.as_ref().unwrap() };
    hashmap.fetch_and_modify(key, f);
}

pub const CONCURRENT_HASH_MAP_SIZE: usize = 40;

#[test]
fn test_concurrent_hash_map() {
    assert_eq!(
        CONCURRENT_HASH_MAP_SIZE,
        std::mem::size_of::<ConcurrentHashMap>(),
        "size mismatch"
    );
}
