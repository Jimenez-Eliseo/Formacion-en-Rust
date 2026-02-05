#[derive(Debug)]
pub enum HSMError {
    Busy,
    LockPoisoned,
}

