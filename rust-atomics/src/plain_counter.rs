#[derive(Debug)]
pub struct PlainCounter {
    value: u64,
}

impl PlainCounter {
    pub fn new(n: u64) -> Self {
        Self { value: n }
    }

    pub fn inc(&mut self) {
        self.value += 1;
    }

    pub fn read(&self) -> u64 {
        self.value
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn plain_counter_init(counter: *mut PlainCounter, n: u64) {
    unsafe { counter.write(PlainCounter::new(n)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn plain_counter_increment(counter: *mut PlainCounter) {
    let counter = unsafe { counter.as_mut().unwrap() };
    counter.inc();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn plain_counter_read(counter: *const PlainCounter) -> u64 {
    let counter = unsafe { counter.as_ref().unwrap() };
    counter.read()
}

pub const PLAIN_COUNTER_SIZE: usize = 8;

#[test]
fn test_plain_counter() {
    assert_eq!(PLAIN_COUNTER_SIZE, std::mem::size_of::<PlainCounter>());
}
