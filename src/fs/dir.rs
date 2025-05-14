use crate::{
    cipher::Cipher,
    errors::MyResult,
    fs::io::{ClearFile, EncryptFile},
};
use std::path::{Path, PathBuf};

pub struct Directory {
    pub path_buf: PathBuf,
    pub cipher: Cipher,
}
impl Directory {
    pub fn make_clear_file(&self, relative: impl AsRef<Path>) -> ClearFile {
        ClearFile::new(self.path_buf.join(relative).into())
    }
    pub fn make_encrypted_file(&self, relative: impl AsRef<Path>) -> EncryptFile {
        EncryptFile::new(self.path_buf.join(relative).into(), self.cipher.clone())
    }

    pub async fn create(&self) -> MyResult {
        async_fs::create_dir_all(&self.path_buf).await?;
        Ok(())
    }

    pub async fn get_files(&self) -> MyResult<Vec<PathBuf>> {
        use futures_lite::stream::StreamExt;

        let mut files = async_fs::read_dir(&self.path_buf).await?;
        let mut paths = vec![];
        while let Ok(Some(x)) = files.try_next().await {
            let path = x.path();
            if path.is_file() {
                paths.push(path);
            }
        }
        Ok(paths)
    }
}
