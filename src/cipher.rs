use crate::errors::MyResult;
use aes_gcm::{
    AeadCore, KeyInit,
    aead::consts::U12,
    aead::{Aead, OsRng},
};
use std::sync::Arc;

#[derive(Debug, Copy, Clone)]
pub enum CipherError {
    InvalidKeyLength,
    EncryptionFailed,
    DecryptionFailed,
}
#[derive(Clone)]
pub struct Cipher(Arc<aes_gcm::Aes256Gcm>);

impl Cipher {
    pub fn from_key(input: &[u8]) -> MyResult<Self, CipherError> {
        aes_gcm::Aes256Gcm::new_from_slice(input)
            .map(|x| Self(Arc::new(x)))
            .map_err(|_| CipherError::InvalidKeyLength)
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, CipherError> {
        let nonce = aes_gcm::Aes256Gcm::generate_nonce(OsRng);
        let mut ciphered = self
            .0
            .encrypt(&nonce, data)
            .map_err(|_| CipherError::EncryptionFailed)?;

        let mut result = nonce.to_vec();
        result.append(&mut ciphered);
        Ok(result)
    }
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, CipherError> {
        let (nonce, payload) = data
            .split_at_checked(12)
            .ok_or(CipherError::DecryptionFailed)?;
        let nonce = aes_gcm::Nonce::<U12>::from_slice(nonce);
        self.0
            .decrypt(&nonce, payload)
            .map_err(|_| CipherError::DecryptionFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aes_gcm::KeyInit;
    #[test]
    fn should_encrypt_and_decrypt_text() {
        let key = aes_gcm::Aes256Gcm::generate_key(OsRng);
        let cipher = Cipher::from_key(&key).unwrap();
        let encrypted = cipher.encrypt(b"hello world").unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, b"hello world");
    }
}
