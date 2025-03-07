use std::ffi::c_ulong;
use std::{
    cell::Cell,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

use crate::sem::Semaphore;

struct Slot {
    inner: Cell<c_ulong>,
    valid: AtomicBool,
}

impl Slot {
    fn new(value: c_ulong) -> Self {
        Self {
            inner: Cell::new(value),
            valid: AtomicBool::new(false),
        }
    }

    fn get(&self) -> c_ulong {
        self.inner.get()
    }

    fn set(&self, value: c_ulong) {
        self.inner.set(value);
    }
}

impl std::fmt::Debug for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

unsafe impl Send for Slot {}
unsafe impl Sync for Slot {}

pub struct SpmcQueue {
    ring: Vec<Slot>,
    head: AtomicUsize,
    tail: AtomicUsize,
    len: AtomicUsize,
    _size: usize,
    _mask: usize,
    write_sem: Semaphore,
    read_sem: Semaphore,
}

impl SpmcQueue {
    pub fn alloc() -> Self {
        Self {
            ring: vec![],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            len: AtomicUsize::new(0),
            _size: 0,
            _mask: 0,
            write_sem: Semaphore::alloc(),
            read_sem: Semaphore::alloc(),
        }
    }

    pub fn init(&mut self, capacity: usize, default: c_ulong) {
        self.ring = vec![];
        self._size = 1 << capacity;
        self._mask = (1 << capacity) - 1;
        self.write_sem.init(self._size as u32);
        self.read_sem.init(0);

        for _ in 0..self._size {
            self.ring.push(Slot::new(default));
        }
    }

    pub fn len(&self) -> usize {
        self.len.load(Ordering::SeqCst)
    }

    pub fn try_push(&self, item: c_ulong) -> bool {
        let idx = self.tail.fetch_add(1, Ordering::SeqCst) & self._mask;
        if self.ring[idx].valid.load(Ordering::SeqCst) {
            self.tail.fetch_sub(1, Ordering::SeqCst);
            false
        } else {
            self.ring[idx].set(item);
            self.ring[idx].valid.store(true, Ordering::SeqCst);
            self.len.fetch_add(1, Ordering::SeqCst);
            self.read_sem.post();
            true
        }
    }

    pub fn push(&self, item: c_ulong) {
        if self.try_push(item) {
            return;
        }

        loop {
            self.write_sem.wait();
            if self.try_push(item) {
                return;
            }
        }
    }

    pub fn try_pop(&self) -> Option<c_ulong> {
        let idx = self.head.fetch_add(1, Ordering::SeqCst) & self._mask;
        if !self.ring[idx].valid.load(Ordering::SeqCst) {
            self.head.fetch_sub(1, Ordering::SeqCst);
            None
        } else {
            let out = self.ring[idx].get();
            self.ring[idx].valid.store(false, Ordering::SeqCst);
            self.len.fetch_sub(1, Ordering::SeqCst);
            self.write_sem.post();
            Some(out)
        }
    }

    pub fn pop(&self) -> c_ulong {
        if let Some(item) = self.try_pop() {
            return item;
        }

        loop {
            self.read_sem.wait();
            if let Some(item) = self.try_pop() {
                return item;
            }
        }
    }

    pub fn serialize(&self) -> Vec<c_ulong> {
        let start_idx = self.head.load(Ordering::SeqCst);
        let end_idx = self.tail.load(Ordering::SeqCst);
        let mut out = vec![];
        let mut i = start_idx;
        while i != end_idx {
            dbg!(i);
            out.push(self.ring[i].get());
            i += 1;
            if i == self._size {
                i = 0;
            }
        }
        out
    }
}

impl std::fmt::Debug for SpmcQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpmcQueue")
            .field("ring", &self.ring)
            .field("start_idx", &self.head.load(Ordering::SeqCst))
            .field("end_idx", &self.tail.load(Ordering::SeqCst))
            .finish()
    }
}

unsafe impl Send for SpmcQueue {}
unsafe impl Sync for SpmcQueue {}

#[cfg(test)]
mod tests {
    use super::SpmcQueue;
    use std::ffi::c_ulong;
    use std::sync::Arc;

    fn new_q(cap: usize) -> Arc<SpmcQueue> {
        let mut queue = SpmcQueue::alloc();
        queue.init(cap + 1, 0);
        Arc::new(queue)
    }

    const END: c_ulong = c_ulong::MAX;

    fn push_end(q: &SpmcQueue) {
        q.push(END)
    }

    fn pop(q: &SpmcQueue) -> Option<c_ulong> {
        let popped = q.pop();
        if popped == END {
            return None;
        } else {
            return Some(popped);
        }
    }

    #[test]
    fn test_push_pop() {
        let q = new_q(3);
        assert_eq!(q.len(), 0);

        assert!(q.try_push(1));
        assert_eq!(q.serialize(), vec![1]);
        assert_eq!(q.len(), 1);

        assert!(q.try_push(2));
        assert_eq!(q.serialize(), vec![1, 2]);
        assert_eq!(q.len(), 2);

        assert!(q.try_push(3));
        assert_eq!(q.serialize(), vec![1, 2, 3]);
        assert_eq!(q.len(), 3);

        assert_eq!(q.try_pop(), Some(1));
        assert_eq!(q.serialize(), vec![2, 3]);
        assert_eq!(q.len(), 2);

        assert_eq!(q.try_pop(), Some(2));
        assert_eq!(q.serialize(), vec![3]);
        assert_eq!(q.len(), 1);

        assert_eq!(q.try_pop(), Some(3));
        assert_eq!(q.serialize(), vec![]);
        assert_eq!(q.len(), 0);

        assert_eq!(q.try_pop(), None);
    }

    #[test]
    fn test_multi_threaded() {
        let q = new_q(8);

        let producer = {
            let q = Arc::clone(&q);
            std::thread::spawn(move || {
                for i in 1..=50 {
                    q.push(i);
                }
                push_end(&q);
                push_end(&q);
            })
        };

        let mut consumers = vec![];
        for _ in 0..2 {
            let q = Arc::clone(&q);
            let consumer = std::thread::spawn(move || {
                let mut out = vec![];
                while let Some(item) = pop(&q) {
                    out.push(item);
                }
                out
            });
            consumers.push(consumer);
        }

        producer.join().unwrap();
        let mut popped = vec![];
        for handle in consumers {
            popped.append(&mut handle.join().unwrap());
        }
        popped.sort_unstable();

        assert_eq!(popped, (1..=50).collect::<Vec<_>>());
    }
}
