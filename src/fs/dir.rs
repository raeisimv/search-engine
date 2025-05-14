use crate::{
    cipher::Cipher,
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
}
