use crate::{cipher::Cipher, errors::MyResult, fs::*};

pub trait File {
    async fn write(&mut self, buf: &[u8]) -> MyResult;
    async fn read(&self) -> MyResult<Vec<u8>>;
    async fn delete(self) -> MyResult;
}

pub struct ClearFile {
    path: std::path::PathBuf,
}
impl ClearFile {
    pub fn new(path: std::path::PathBuf) -> Self {
        Self { path }
    }
}
impl From<std::path::PathBuf> for ClearFile {
    fn from(path: std::path::PathBuf) -> Self {
        Self::new(path)
    }
}

impl File for ClearFile {
    async fn write(&mut self, buf: &[u8]) -> MyResult {
        write(&self.path, buf).await
    }

    async fn read(&self) -> MyResult<Vec<u8>> {
        read(&self.path).await
    }

    async fn delete(self) -> MyResult {
        delete(&self.path).await
    }
}

pub struct EncryptFile {
    path: std::path::PathBuf,
    cipher: Cipher,
}
impl EncryptFile {
    pub fn new(path: std::path::PathBuf, cipher: Cipher) -> Self {
        Self { path, cipher }
    }
}
impl File for EncryptFile {
    async fn write(&mut self, buf: &[u8]) -> MyResult {
        let payload = self.cipher.encrypt(buf).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid data for encryption",
            )
        })?;
        write(&self.path, payload.as_slice()).await
    }

    async fn read(&self) -> MyResult<Vec<u8>> {
        let payload = read(&self.path).await?;
        let x = self.cipher.decrypt(payload.as_slice()).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid data for decryption",
            )
        })?;
        Ok(x)
    }

    async fn delete(self) -> MyResult {
        delete(&self.path).await
    }
}
