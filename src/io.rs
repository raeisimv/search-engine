use crate::errors::MyResult;
use crate::fs::*;

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
