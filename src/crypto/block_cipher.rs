pub trait BlockDecryptor{
    fn encrypt(plaintext :&[u8], AAD :&[u8]) -> Vec<u8>;
}