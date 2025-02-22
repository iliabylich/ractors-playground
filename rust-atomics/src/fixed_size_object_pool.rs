use parking_lot::Mutex;
use std::{
    ffi::c_ulong,
    time::{Duration, Instant},
};

pub struct FixedSizeObjectPool {
    mutex: Mutex<()>,
    pool: Vec<c_ulong>,
    timeout: Duration,
}

impl FixedSizeObjectPool {
    fn new() -> Self {
        Self {
            mutex: Mutex::new(()),
            pool: vec![],
            timeout: Duration::MAX,
        }
    }

    fn init(
        &mut self,
        max_size: usize,
        timeout_in_ms: u64,
        rb_make_obj: extern "C" fn(c_ulong) -> c_ulong,
    ) {
        self.pool.clear();
        self.pool.reserve(max_size);
        for _ in 0..max_size {
            self.pool.push((rb_make_obj)(0));
        }
        self.timeout = Duration::from_millis(timeout_in_ms);
    }

    fn mark(&self, f: extern "C" fn(c_ulong)) {
        for item in self.pool.iter() {
            f(*item);
        }
    }

    fn pop(&mut self) -> Option<c_ulong> {
        let start = Instant::now();
        let end = start + self.timeout;

        // fast path
        if let Some(_guard) = self.mutex.try_lock() {
            if let Some(popped) = self.pool.pop() {
                return Some(popped);
            }
        }

        // slow path
        while Instant::now() < end {
            if let Some(_guard) = self.mutex.try_lock_until(end) {
                if let Some(popped) = self.pool.pop() {
                    return Some(popped);
                }
            }
            std::hint::spin_loop();
        }

        // failed both ways
        None
    }

    fn push(&mut self, value: c_ulong) {
        let _guard = self.mutex.lock();
        self.pool.push(value);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fixed_size_object_pool_alloc(pool: *mut FixedSizeObjectPool) {
    unsafe { pool.write(FixedSizeObjectPool::new()) }
}

#[unsafe(no_mangle)]
pub extern "C" fn fixed_size_object_pool_init(
    pool: *mut FixedSizeObjectPool,
    max_size: usize,
    timeout_in_ms: u64,
    rb_make_obj: extern "C" fn(c_ulong) -> c_ulong,
) {
    let pool = unsafe { pool.as_mut().unwrap() };
    pool.init(max_size, timeout_in_ms, rb_make_obj);
}

#[unsafe(no_mangle)]
pub extern "C" fn fixed_size_object_pool_drop(pool: *mut FixedSizeObjectPool) {
    unsafe { std::ptr::drop_in_place(pool) };
}

#[unsafe(no_mangle)]
pub extern "C" fn fixed_size_object_pool_mark(
    pool: *const FixedSizeObjectPool,
    f: extern "C" fn(c_ulong),
) {
    let pool = unsafe { pool.as_ref().unwrap() };
    pool.mark(f);
}

#[unsafe(no_mangle)]
pub extern "C" fn fixed_size_object_pool_pop(
    pool: *mut FixedSizeObjectPool,
    fallback: c_ulong,
) -> c_ulong {
    let pool = unsafe { pool.as_mut().unwrap() };
    pool.pop().unwrap_or(fallback)
}

#[unsafe(no_mangle)]
pub extern "C" fn fixed_size_object_pool_push(pool: *mut FixedSizeObjectPool, value: c_ulong) {
    let pool = unsafe { pool.as_mut().unwrap() };
    pool.push(value);
}

pub const FIXED_SIZE_OBJECT_POOL_SIZE: usize = 48;

#[test]
fn test_concurrent_hash_map() {
    assert_eq!(
        FIXED_SIZE_OBJECT_POOL_SIZE,
        std::mem::size_of::<FixedSizeObjectPool>(),
        "size mismatch"
    );
}
