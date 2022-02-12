
pub struct ColumnEncryptionProperties {
    columnPath: ColumnPath,
    encrypted : bool,
    encrypted_with_footer_key: bool,

    key: Vec<u8>,
    key_meta:  Vec<u8>,

    key_retriever :DecryptionKeyRetriever,
    aad_prefix__verifier: AADPrefixVerifier,
}