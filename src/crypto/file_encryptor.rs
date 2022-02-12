use super::block_encryptor::BlockEncryptor;
use crate::crypto::aad::AAD;
use std::collections::HashMap;

enum AlgorithmName {
    AesGcmV1,
    AesGcmCtrV1,
}

pub struct ColumnEncryptOptions {
    encrypted: bool,
    encrypted_with_footer_key: bool,
    key: Vec<u8>,
    key_metadata: Vec<u8>,
}

pub struct FileEncryptOptions {
    algorithm: AlgorithmName,
    encrypted_footer: bool, // it is required to sign footer if not encrypted

    columns: HashMap<String, ColumnEncryptOptions>,

    add_prefix: Vec<u8>,
    store_add_prefix_in_file: bool,

    footer_key: Vec<u8>,
    footer_key_metadata: Vec<u8>,
}

pub struct FileEncryptor {}

impl FileEncryptor {
    pub fn new() -> Self {
        FileEncryptor {}
    }

    pub fn file_aad(&self) -> AAD {
        todo!()
    }

    pub fn get_block_encryptor_file(&self) -> BlockEncryptor {
        todo!()
    }

    pub fn get_block_encryptor_file_sign(&self) -> BlockEncryptor {
        todo!()
    }

    pub fn get_block_encryptor_column(&self) -> BlockEncryptor {
        todo!()
    }
}
