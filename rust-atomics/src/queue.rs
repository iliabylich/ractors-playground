use parking_lot::Mutex;
use std::{collections::VecDeque, ffi::c_ulong};

struct Inner {
    queue: VecDeque<c_ulong>,
    cap: usize,
}

impl Inner {
    fn alloc() -> Self {
        Self {
            queue: VecDeque::new(),
            cap: 0,
        }
    }

    fn init(&mut self, cap: usize) {
        self.cap = cap;
    }

    fn try_push(&mut self, value: c_ulong) -> bool {
        if self.queue.len() < self.cap {
            self.queue.push_back(value);
            true
        } else {
            false
        }
    }

    fn try_pop(&mut self) -> Option<c_ulong> {
        self.queue.pop_front()
    }

    fn for_each(&self, f: extern "C" fn(c_ulong)) {
        for item in self.queue.iter() {
            f(*item);
        }
    }
}

pub struct Queue {
    inner: Mutex<Inner>,
}

impl Queue {
    fn alloc() -> Self {
        Self {
            inner: Mutex::new(Inner::alloc()),
        }
    }

    fn init(&mut self, cap: usize) {
        let mut inner = self.inner.lock();
        inner.init(cap);
    }

    fn mark(&self, f: extern "C" fn(c_ulong)) {
        let inner = self.inner.lock();
        inner.for_each(f);
    }

    fn try_push(&self, value: c_ulong) -> bool {
        if let Some(mut inner) = self.inner.try_lock() {
            if inner.try_push(value) {
                return true;
            }
        }
        false
    }

    fn try_pop(&self) -> Option<c_ulong> {
        if let Some(mut inner) = self.inner.try_lock() {
            if let Some(value) = inner.try_pop() {
                return Some(value);
            }
        }

        None
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn queue_alloc(queue: *mut Queue) {
    unsafe { queue.write(Queue::alloc()) }
}

#[unsafe(no_mangle)]
pub extern "C" fn queue_init(queue: *mut Queue, cap: usize) {
    let queue = unsafe { queue.as_mut().unwrap() };
    queue.init(cap);
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
pub extern "C" fn queue_try_pop(queue: *mut Queue, fallback: c_ulong) -> c_ulong {
    let queue = unsafe { queue.as_mut().unwrap() };
    queue.try_pop().unwrap_or(fallback)
}

#[unsafe(no_mangle)]
pub extern "C" fn queue_try_push(queue: *mut Queue, value: c_ulong) -> bool {
    let queue = unsafe { queue.as_mut().unwrap() };
    queue.try_push(value)
}

pub const QUEUE_SIZE: usize = 48;

#[test]
fn test_queue() {
    assert_eq!(QUEUE_SIZE, std::mem::size_of::<Queue>(), "size mismatch");
    assert!(crate::is_sync_and_send::<Queue>());
}
