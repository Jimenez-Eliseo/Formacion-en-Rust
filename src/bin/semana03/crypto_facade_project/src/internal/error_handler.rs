#[derive(Debug)]
pub enum CryptoError {
    InvalidKey(String),
    InvalidIv,
    EncryptionFailed(String),
    DecryptionFailed(String),
}

#[derive(Debug)]
pub enum PackError {
    UnPack(String),
}

#[derive(Debug)]
pub enum FacadeError {
    Pack(PackError),
    Crypto(CryptoError),
    InvalidUtf8,
    EncryptError(CryptoError),
}

impl From<CryptoError> for FacadeError {
    fn from(err: CryptoError) -> Self {
        FacadeError::Crypto(err)
    }
}

impl From<PackError> for FacadeError {
    fn from(err: PackError) -> Self {
        FacadeError::Pack(err)
    }
}
