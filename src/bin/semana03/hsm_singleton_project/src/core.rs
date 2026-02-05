use std::sync::atomic::AtomicU32;

#[derive(Debug)]
pub(crate) enum Status {
    Ready,
    Busy,
}

#[derive(Debug)]
pub(crate) struct HSMCore {
    pub operation_count: AtomicU32,
    pub status: Status,
}

impl HSMCore {
    pub(crate) fn new() -> Self {
        Self {
            operation_count: AtomicU32::new(0),
            status: Status::Ready,
        }
    }
}
