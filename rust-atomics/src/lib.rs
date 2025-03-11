#![expect(clippy::missing_safety_doc)]

mod plain_counter;
pub use plain_counter::*;

mod counter;
pub use counter::*;

mod hashmap;
pub use hashmap::*;

mod fixed_size_object_pool;
pub use fixed_size_object_pool::*;

mod queue;
pub use queue::*;

mod slow_object;
pub use slow_object::*;

mod spmc_queue;
pub use spmc_queue::*;

mod sem;

#[cfg(test)]
pub(crate) fn is_sync_and_send<T: Sync + Send>() -> bool {
    true
}
