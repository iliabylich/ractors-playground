use crossbeam_channel::{Receiver, Sender};
use std::{ffi::c_ulong, time::Duration};

pub struct Queue {
    tx: Sender<c_ulong>,
    rx: Receiver<c_ulong>,
}

impl Queue {
    fn alloc() -> Self {
        let (tx, rx) = crossbeam_channel::bounded(10);
        Self { tx, rx }
    }

    fn init(&mut self, cap: usize) {
        (self.tx, self.rx) = crossbeam_channel::bounded(cap);
    }

    fn mark(&self, f: extern "C" fn(c_ulong)) {
        for item in self.rx.try_iter() {
            f(item);
        }
    }

    fn push(&self, value: c_ulong) -> bool {
        self.tx
            .send_timeout(value, Duration::from_millis(100))
            .is_ok()
    }

    fn try_pop(&self) -> Option<c_ulong> {
        self.rx.recv_timeout(Duration::from_millis(100)).ok()
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
    queue.push(value)
}

pub const QUEUE_SIZE: usize = 32;

#[test]
fn test_queue() {
    assert_eq!(QUEUE_SIZE, std::mem::size_of::<Queue>(), "size mismatch");
    assert!(crate::is_sync_and_send::<Queue>());
}
