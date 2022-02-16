
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead};


fn gcm(key : &[u8], aad: &[u8]) {
    let nonce = b"";
    let key = Key::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref())
        .expect("encryption failure!"); // NOTE: handle this error to avoid panics!

    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!"); // NOTE: handle this error to avoid panics!
}

