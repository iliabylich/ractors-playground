use std::{ffi::c_ulong, time::Duration};

pub struct SlowObject {
    n: u64,
}

impl SlowObject {
    fn alloc() -> Self {
        Self { n: 0 }
    }

    fn init(&mut self, n: u64) {
        self.n = n;
    }

    fn mark(&self, _: extern "C" fn(c_ulong)) {
        eprintln!("[mark] started");
        std::thread::sleep(Duration::from_secs(2));
        eprintln!("[mark] finished");
    }

    fn slow_op(&self) {
        eprintln!("[slow_op] started");
        for i in 1..=10 {
            eprintln!("tick {i}");
            std::thread::sleep(Duration::from_millis(100));
        }
        eprintln!("[slow_op] finished");
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn slow_object_alloc(slow: *mut SlowObject) {
    unsafe { slow.write(SlowObject::alloc()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn slow_object_init(slow: *mut SlowObject, n: u64) {
    let slow = unsafe { slow.as_mut().unwrap() };
    slow.init(n);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn slow_object_drop(slow: *mut SlowObject) {
    unsafe { std::ptr::drop_in_place(slow) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn slow_object_mark(slow: *const SlowObject, f: extern "C" fn(c_ulong)) {
    let slow = unsafe { slow.as_ref().unwrap() };
    slow.mark(f);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn slow_object_slow_op(slow: *const SlowObject) {
    let slow = unsafe { slow.as_ref().unwrap() };
    slow.slow_op();
}

pub const SLOW_OBJECT_SIZE: usize = 8;

#[test]
fn test_concurrent_hash_map() {
    assert_eq!(
        SLOW_OBJECT_SIZE,
        std::mem::size_of::<SlowObject>(),
        "size mismatch"
    );

    assert!(crate::is_sync_and_send::<SlowObject>());
}
