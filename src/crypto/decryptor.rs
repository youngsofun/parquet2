pub struct ColumnDecryptOptions {
    encrypted: bool,
    encrypted_with_footer_key: bool,
    key: Vec<u8>,
    key_metadata: Option<Vec<u8>>,
}
