// #[cfg(loom)]
// pub(crate) use loom::{
//     cell::Cell,
//     model,
//     sync::{
//         Arc,
//         atomic::{AtomicBool, AtomicPtr, AtomicUsize, Ordering},
//     },
//     thread,
// };

// #[cfg(not(loom))]
pub(crate) use std::{
    cell::Cell,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread,
};
#[cfg(not(loom))]
pub(crate) fn model<F: Fn() + Sync + Send + 'static>(f: F) {
    f();
}
