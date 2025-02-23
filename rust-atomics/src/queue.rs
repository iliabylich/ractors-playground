use parking_lot::{Condvar, Mutex};
use std::{collections::VecDeque, ffi::c_ulong};

pub struct Queue {
    queue: Mutex<VecDeque<c_ulong>>,
    cond: Condvar,
}

impl Queue {
    fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            cond: Condvar::new(),
        }
    }

    fn mark(&self, f: extern "C" fn(c_ulong)) {
        for item in self.queue.lock().iter() {
            f(*item);
        }
    }

    fn push(&self, value: c_ulong) {
        let mut queue = self.queue.lock();
        queue.push_back(value);
        self.cond.notify_one();
    }

    fn pop(&self) -> c_ulong {
        loop {
            let mut queue = self.queue.lock();
            if let Some(value) = queue.pop_front() {
                return value;
            }
            self.cond.wait(&mut queue);
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

pub const QUEUE_SIZE: usize = 40;

#[test]
fn test_concurrent_hash_map() {
    assert_eq!(QUEUE_SIZE, std::mem::size_of::<Queue>(), "size mismatch");
}
