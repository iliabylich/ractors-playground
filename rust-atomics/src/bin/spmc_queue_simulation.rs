use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use libc::c_ulong;
use rust_atomics::SpmcQueue;

const PUSH_DELAY: Duration = Duration::from_millis(10);
const RUN_GC_EVERY: Duration = Duration::from_millis(500);
const PUSH_ITERATIONS: u64 = 5;
const THREADS_COUNT: u8 = 10;

fn main() {
    let q = make_q(5);

    let mut consumers = vec![];
    for _ in 0..THREADS_COUNT {
        consumers.push(start_consumer(Arc::clone(&q)));
    }

    start_producer(Arc::clone(&q));

    for consumer in consumers {
        consumer.join().unwrap();
    }
}

fn make_q(capacity: usize) -> Arc<SpmcQueue> {
    let mut q = SpmcQueue::alloc();
    q.init(capacity, 0);
    Arc::new(q)
}

const END: c_ulong = c_ulong::MAX;
fn push_end(q: &SpmcQueue) {
    q.push(END);
}
fn pop(q: &SpmcQueue) -> Option<c_ulong> {
    match q.pop() {
        END => None,
        other => Some(other),
    }
}

fn start_consumer(q: Arc<SpmcQueue>) -> std::thread::JoinHandle<Vec<c_ulong>> {
    std::thread::spawn(move || {
        let mut popped = vec![];

        while let Some(value) = pop(&q) {
            println!("[{:?}] popped {value}", std::thread::current().id());
            popped.push(value);
        }

        popped
    })
}

fn start_producer(q: Arc<SpmcQueue>) {
    let mut value = 1;

    for _ in 0..PUSH_ITERATIONS {
        // push for `RUN_GC_EVERY`
        let start = Instant::now();
        while Instant::now() - start < RUN_GC_EVERY {
            q.try_push(value);
            value += 1;
            std::thread::sleep(PUSH_DELAY);
        }

        // simulate gc
        q.with_locked_consuming(|| {
            println!("===== GC START ======");
            std::thread::sleep(Duration::from_millis(1000));
            // q.mark();
            println!("===== GC END ========");
        });
    }

    for _ in 0..THREADS_COUNT {
        push_end(&q);
    }
}
