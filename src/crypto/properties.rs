use std::collections::HashMap;
use parquet_format_async_temp::EncryptionAlgorithm;


struct ColumnPath;

pub struct ColumnEncryptionProperties {
    encrypted : bool,
    encryptedWithFooterKey: bool,
    columnPath: ColumnPath,

    keyBytes: Vec<u8>,
    keyMetaBytes:  Vec<u8>,
}

pub struct FileDecryptionProperties {
    footerKey: Vec<[u8]>,
    aadPrefix:  Vec<[u8]>,

    checkPlaintextFooterIntegrity: bool,
    plaintextAllowed             : bool,
    utilized                      : bool,

    algorithm:  EncryptionAlgorithm,
    columnPropertyMap: HashMap<ColumnPath, ColumnEncryptionProperties>,
    //Verifier                      AADPrefixVerifier
    //KeyRetriever                  DecryptionKeyRetriever
}