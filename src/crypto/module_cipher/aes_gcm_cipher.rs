use crate::crypto::module_cipher::BlockCipher;
use crate::error::{ParquetError, Result};
use aes_gcm::aead::{AeadInPlace, NewAead};
use aes_gcm::{aes::Aes192, Aes128Gcm, Aes256Gcm, AesGcm, Nonce};
use std::sync::Arc;
use typenum::U12;

pub struct AesGcmCipherFactory;

struct AesGcmCipher<T: AeadInPlace + NewAead> {
    inner: T,
}

impl AesGcmCipherFactory {
    pub fn new(key: &[u8]) -> Result<Arc<dyn BlockCipher>> {
        match key.len() {
            16 => Ok(Arc::new(AesGcmCipher::<Aes128Gcm>::new(key))),
            32 => Ok(Arc::new(AesGcmCipher::<Aes256Gcm>::new(key))),
            24 => Ok(Arc::new(AesGcmCipher::<AesGcm<Aes192, U12>>::new(key))),
            _ => Err(general_err!("wrong key size")),
        }
    }
}

impl<T: AeadInPlace + NewAead> AesGcmCipher<T> {
    pub fn new(key: &[u8]) -> Self {
        AesGcmCipher {
            inner: T::new_from_slice(key).unwrap(),
        }
    }
}

impl<C: AeadInPlace + NewAead> BlockCipher for AesGcmCipher<C> {
    fn encrypt_in_place(&self, buf: &mut Vec<u8>, nonce: [u8; 12], aad: &[u8]) -> Result<()> {
        self.inner
            .encrypt_in_place(Nonce::from_slice(&nonce), &aad, buf)
            .map_err(|e| general_err!("{:?}", e))
    }

    fn decrypt_in_place(&self, buf: &mut Vec<u8>, nonce: [u8; 12], aad: &[u8]) -> Result<()> {
        self.inner
            .decrypt_in_place(Nonce::from_slice(&nonce), &aad, buf)
            .map_err(|e| general_err!("{:?}", e))
    }
}
