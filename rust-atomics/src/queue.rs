use parking_lot::{Condvar, Mutex};
use std::ffi::c_ulong;

pub struct Queue {
    queue: scc::Queue<c_ulong>,
    mutex: Mutex<()>,
    cond: Condvar,
}

impl Queue {
    fn new() -> Self {
        Self {
            queue: scc::Queue::default(),
            mutex: Mutex::new(()),
            cond: Condvar::new(),
        }
    }

    fn mark(&self, f: extern "C" fn(c_ulong)) {
        let guard = scc::ebr::Guard::new();
        for item in self.queue.iter(&guard) {
            f(*item);
        }
    }

    fn push(&self, value: c_ulong) {
        self.queue.push(value);
        self.cond.notify_one();
    }

    fn pop(&self) -> c_ulong {
        if let Some(value) = self.queue.pop() {
            return **value;
        }

        loop {
            let mut guard = self.mutex.lock();
            self.cond.wait(&mut guard);
            if let Some(value) = self.queue.pop() {
                return **value;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn queue_init(queue: *mut Queue) {
    unsafe { queue.write(Queue::new()) }
}

#[unsafe(no_mangle)]
pub extern "C" fn queue_drop(queue: *mut Queue) {
    unsafe { std::ptr::drop_in_place(queue) };
}

#[unsafe(no_mangle)]
pub extern "C" fn queue_mark(queue: *const Queue, f: extern "C" fn(c_ulong)) {
    let queue = unsafe { queue.as_ref().unwrap() };
    queue.mark(f);
}

#[unsafe(no_mangle)]
pub extern "C" fn queue_pop(queue: *mut Queue) -> c_ulong {
    let queue = unsafe { queue.as_mut().unwrap() };
    queue.pop()
}

#[unsafe(no_mangle)]
pub extern "C" fn queue_push(queue: *mut Queue, value: c_ulong) {
    let queue = unsafe { queue.as_mut().unwrap() };
    queue.push(value);
}

pub const QUEUE_SIZE: usize = 640;

#[test]
fn test_queue() {
    assert_eq!(QUEUE_SIZE, std::mem::size_of::<Queue>(), "size mismatch");
    assert!(crate::is_sync_and_send::<Queue>());
}
