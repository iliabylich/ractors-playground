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
    capacity: usize,
    write_sem: Semaphore,
    read_sem: Semaphore,
    consuming_locked: AtomicBool,
    consumers_count: AtomicUsize,
}

impl SpmcQueue {
    pub fn alloc() -> Self {
        Self {
            ring: vec![],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            capacity: 0,
            write_sem: Semaphore::alloc(),
            read_sem: Semaphore::alloc(),
            consuming_locked: AtomicBool::new(false),
            consumers_count: AtomicUsize::new(0),
        }
    }

    pub fn init(&mut self, capacity: usize, default: c_ulong) {
        self.capacity = capacity;
        self.ring = vec![];
        self.write_sem.init(self.size() as u32);
        self.read_sem.init(0);

        for _ in 0..self.size() {
            self.ring.push(Slot::new(default));
        }
    }

    fn size(&self) -> usize {
        1 << self.capacity
    }
    fn mask(&self) -> usize {
        (1 << self.capacity) - 1
    }

    pub fn try_push(&self, item: c_ulong) -> bool {
        let idx = self.tail.fetch_add(1, Ordering::SeqCst) & self.mask();
        if self.ring[idx].valid.load(Ordering::SeqCst) {
            self.tail.fetch_sub(1, Ordering::SeqCst);
            false
        } else {
            self.ring[idx].set(item);
            self.ring[idx].valid.store(true, Ordering::SeqCst);
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
        self.consumers_count.fetch_add(1, Ordering::SeqCst);
        println!("[{:?}] try_pop", std::thread::current().id());
        std::thread::sleep(std::time::Duration::from_millis(100));
        let idx = self.head.fetch_add(1, Ordering::SeqCst) & self.mask();
        let out = if !self.ring[idx].valid.load(Ordering::SeqCst) {
            self.head.fetch_sub(1, Ordering::SeqCst);
            None
        } else {
            let out = self.ring[idx].get();
            self.ring[idx].valid.store(false, Ordering::SeqCst);
            self.write_sem.post();
            Some(out)
        };
        self.consumers_count.fetch_sub(1, Ordering::SeqCst);
        out
    }

    pub fn pop(&self) -> c_ulong {
        while self.consuming_locked.load(Ordering::SeqCst) {
            // spin
        }

        if let Some(item) = self.try_pop() {
            return item;
        }

        loop {
            while self.consuming_locked.load(Ordering::SeqCst) {
                // spin
            }

            self.read_sem.wait();
            if let Some(item) = self.try_pop() {
                return item;
            }
        }
    }

    pub fn with_locked_consuming<T>(&self, f: impl FnOnce() -> T) -> T {
        self.consuming_locked.store(true, Ordering::SeqCst);
        loop {
            let consumers_count = self.consumers_count.load(Ordering::SeqCst);
            if consumers_count == 0 {
                break;
            } else {
                // spin until they are done
                println!("[producer] waiting for {consumers_count} consumers to finish");
            }
        }
        let out = f();
        self.consuming_locked.store(false, Ordering::SeqCst);
        out
    }

    fn foreach(&self, mut f: impl FnMut(c_ulong)) {
        let start_idx = self.head.load(Ordering::SeqCst);
        let end_idx = self.tail.load(Ordering::SeqCst);
        let mut i = start_idx;
        while i != end_idx {
            f(self.ring[i].get());
            i += 1;
            if i == self.size() {
                i = 0;
            }
        }
    }

    fn mark(&self, mark: extern "C" fn(c_ulong)) {
        self.with_locked_consuming(|| {
            self.foreach(|e| mark(e));
        })
    }

    pub fn serialize(&self) -> Vec<c_ulong> {
        let mut out = vec![];
        self.foreach(|e| {
            out.push(e);
        });
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
        assert_eq!(q.serialize(), vec![]);

        assert!(q.try_push(1));
        assert_eq!(q.serialize(), vec![1]);

        assert!(q.try_push(2));
        assert_eq!(q.serialize(), vec![1, 2]);

        assert!(q.try_push(3));
        assert_eq!(q.serialize(), vec![1, 2, 3]);

        assert_eq!(q.try_pop(), Some(1));
        assert_eq!(q.serialize(), vec![2, 3]);

        assert_eq!(q.try_pop(), Some(2));
        assert_eq!(q.serialize(), vec![3]);

        assert_eq!(q.try_pop(), Some(3));
        assert_eq!(q.serialize(), vec![]);

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
