use super::ModuleCipher;
use super::ParquetCipherType;
use super::AAD;
use parquet_format_async_temp::{EncryptionAlgorithm, FileCryptoMetaData};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ColumnEncryptOptions {
    encrypted: bool,
    encrypted_with_footer_key: bool,
    key: Vec<u8>,
    key_metadata: Option<Vec<u8>>,
    column_path: Vec<String>,
}

pub struct FileEncryptOptions {
    algorithm: ParquetCipherType,
    encrypted_footer: bool,

    columns: HashMap<String, ColumnEncryptOptions>,

    add_prefix: Vec<u8>,
    store_add_prefix_in_file: bool,

    footer_key: Vec<u8>,
    footer_key_metadata: Option<Vec<u8>>,
}

pub struct Encryptor {
    // options: FileEncryptOptions,
// column_ciphers: HashMap<String, Option<Arc<ModuleCipher>>>,
// footer_cipher: Arc<ModuleCipher>,
}

impl Encryptor {
    pub fn new() -> Self {
        todo!()
    }

    pub fn encrypted_footer(&self) -> bool {
        todo!()
    }
    pub fn sign(&self, buf: &[u8]) -> Vec<u8> {
        todo!()
    }

    pub fn file_aad(&self) -> AAD {
        todo!()
    }

    pub fn get_file_crypto_metadata(&self) -> FileCryptoMetaData {
        todo!()
    }

    pub fn get_encryption_algorithm(&self) -> EncryptionAlgorithm {
        todo!()
    }

    pub fn get_footer_signing_key_metadata(&self) -> Option<Vec<u8>> {
        todo!()
    }

    pub(crate) fn get_footer_cipher(&self) -> ModuleCipher {
        todo!()
    }

    pub(crate) fn get_column_cipher(&self) -> ModuleCipher {
        todo!()
    }
}
