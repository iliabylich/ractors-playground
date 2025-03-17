#![expect(clippy::missing_safety_doc)]

mod plain_counter;
pub use plain_counter::*;

mod counter;
pub use counter::*;

mod hashmap;
pub use hashmap::*;

mod fixed_size_object_pool;
pub use fixed_size_object_pool::*;

mod queue_with_mutex;
pub use queue_with_mutex::*;

mod slow_object;
pub use slow_object::*;

mod mpmc_queue;
pub use mpmc_queue::*;

mod gc_guard;
pub(crate) use gc_guard::GcGuard;

mod sem;

#[cfg(test)]
pub(crate) fn is_sync_and_send<T: Sync + Send>() -> bool {
    true
}
