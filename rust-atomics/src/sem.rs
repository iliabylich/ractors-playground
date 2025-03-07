use libc::{sem_destroy, sem_init, sem_post, sem_t, sem_wait};

pub(crate) struct Semaphore {
    inner: *mut sem_t,
}

impl Semaphore {
    pub(crate) fn alloc() -> Self {
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn init(&mut self, initial: u32) {
        let ptr = Box::into_raw(Box::new(unsafe { std::mem::zeroed() }));

        let res = unsafe { sem_init(ptr, 0, initial) };
        if res != 0 {
            panic!(
                "failed to create semaphore: {:?}",
                std::io::Error::last_os_error()
            )
        }

        self.inner = ptr;
    }

    pub(crate) fn post(&self) {
        let res = unsafe { sem_post(self.inner) };
        if res != 0 {
            panic!(
                "failed to post to semaphore: {:?}",
                std::io::Error::last_os_error()
            )
        }
    }

    pub(crate) fn wait(&self) {
        let res = unsafe { sem_wait(self.inner) };
        if res != 0 {
            panic!(
                "failed to wait for semaphore: {:?}",
                std::io::Error::last_os_error()
            )
        }
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe {
            sem_destroy(self.inner);
            drop(Box::from_raw(self.inner));
        }
    }
}
