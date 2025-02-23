use parking_lot::{Condvar, Mutex};
use std::{
    ffi::c_ulong,
    time::{Duration, Instant},
};

pub struct FixedSizeObjectPool {
    pool: Mutex<Vec<c_ulong>>,
    cond: Condvar,
    timeout: Duration,
}

impl FixedSizeObjectPool {
    fn new() -> Self {
        Self {
            pool: Mutex::new(vec![]),
            cond: Condvar::new(),
            timeout: Duration::MAX,
        }
    }

    fn init(
        &mut self,
        max_size: usize,
        timeout_in_ms: u64,
        rb_make_obj: extern "C" fn(c_ulong) -> c_ulong,
    ) {
        let mut pool = Vec::with_capacity(max_size);
        for _ in 0..max_size {
            pool.push((rb_make_obj)(0));
        }
        self.pool = Mutex::new(pool);
        self.timeout = Duration::from_millis(timeout_in_ms);
    }

    fn mark(&self, f: extern "C" fn(c_ulong)) {
        let pool = self.pool.lock();
        for item in pool.iter() {
            f(*item);
        }
    }

    fn pop(&mut self) -> Option<c_ulong> {
        let start = Instant::now();
        let end = start + self.timeout;

        // fast path
        if let Some(mut pool) = self.pool.try_lock() {
            if let Some(popped) = pool.pop() {
                return Some(popped);
            }
        }

        // slow path
        loop {
            let mut pool = self.pool.lock();
            let timed_out = self.cond.wait_until(&mut pool, end).timed_out();
            if timed_out {
                return None;
            }
            if let Some(popped) = pool.pop() {
                return Some(popped);
            }
        }
    }

    fn push(&mut self, value: c_ulong) {
        let mut pool = self.pool.lock();
        pool.push(value);
        self.cond.notify_one();
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

pub const FIXED_SIZE_OBJECT_POOL_SIZE: usize = 56;

#[test]
fn test_concurrent_hash_map() {
    assert_eq!(
        FIXED_SIZE_OBJECT_POOL_SIZE,
        std::mem::size_of::<FixedSizeObjectPool>(),
        "size mismatch"
    );
}
