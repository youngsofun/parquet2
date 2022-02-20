use aes::cipher::StreamCipher;
use aes::{Aes128, Aes192, Aes256};
use crypto_common::{Iv, Key, KeyIvInit};
use std::marker::PhantomData;
use std::sync::Arc;

use crate::error::{ParquetError, Result};

use crate::crypto::module_cipher::BlockCipher;

struct AesCtrCipher<C: StreamCipher + KeyIvInit> {
    key: Key<C>,
    phantom: PhantomData<C>,
}

pub struct AesCtrCipherFactory;

impl AesCtrCipherFactory {
    pub fn new(key: &[u8]) -> Result<Arc<dyn BlockCipher>> {
        match key.len() {
            16 => Ok(Arc::new(AesCtrCipher::<ctr::Ctr32BE<Aes128>>::new(key))),
            24 => Ok(Arc::new(AesCtrCipher::<ctr::Ctr32BE<Aes192>>::new(key))),
            32 => Ok(Arc::new(AesCtrCipher::<ctr::Ctr32BE<Aes256>>::new(key))),
            _ => Err(general_err!("wrong key size")),
        }
    }
}

impl<C: StreamCipher + KeyIvInit> AesCtrCipher<C> {
    pub fn new(key: &[u8]) -> Self {
        //pub fn new(&key: &Key<C>) -> Self {
        let key = Key::<C>::clone_from_slice(key);
        AesCtrCipher {
            key,
            phantom: PhantomData,
        }
    }
}

impl<C: StreamCipher + KeyIvInit> BlockCipher for AesCtrCipher<C> {
    fn encrypt_in_place(&self, buf: &mut Vec<u8>, nonce: [u8; 12], _aad: &[u8]) -> Result<()> {
        let mut iv = [0u8; 16];
        iv[15] = 1;
        iv[..12].copy_from_slice(&nonce);
        let mut cipher = C::new(&self.key, &Iv::<C>::from_slice(&iv));
        cipher.apply_keystream(&mut buf[..]);
        Ok(())
    }

    fn decrypt_in_place(&self, buf: &mut Vec<u8>, nonce: [u8; 12], aad: &[u8]) -> Result<()> {
        self.encrypt_in_place(buf, nonce, aad)
    }
}
