pub mod aad;
pub mod block_encryptor;
pub mod file_encryptor;
mod cipher;

use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead};

