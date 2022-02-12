use std::collections::HashMap;
use parquet_format_async_temp::EncryptionAlgorithm;
use super::keytools::key_metadata::KeyMetadata;

trait AADPrefixVerifier {
    fn verify(aadPrefix: &[u8]) -> bool;
}

pub trait DecryptionKeyRetriever {
    fn get_key(key_metadata: &[u8]) -> Vec<u8>;
}

pub struct ColumnDecryptionProperties {
    column_path: ColumnPath,
    key_bytes: Vec<u8>,
}

struct InternalColumnDecryptionSetup {
    column_path: ColumnPath,
    key_bytes: Vec<u8>,
    is_encrypted: bool,
    is_encrypted_with_footer_key: bool,
    data_decryptor: bool,
    meta_decryptor: bool,

    key_metadata:
    // private final BlockCipher.Decryptor dataDecryptor;
    // private final BlockCipher.Decryptor metaDataDecryptor;
    // private final int columnOrdinal;
    // private final byte[] keyMetadata;
}


pub struct FileDecryptionProperties {
    footerKey: Vec<[u8]>,
    aadPrefix: Vec<[u8]>,

    checkPlaintextFooterIntegrity: bool,
    plaintextAllowed: bool,
    utilized: bool,

    algorithm: EncryptionAlgorithm,
    columnPropertyMap: HashMap<ColumnPath, ColumnDecryptionProperties>,
    //Verifier                      AADPrefixVerifier
    //KeyRetriever                  DecryptionKeyRetriever
}




struct InternalFileDecryptor {
    properties: FileDecryptionProperties,

    // private final DecryptionKeyRetriever keyRetriever;
    // private final boolean checkPlaintextFooterIntegrity;
    // private final byte[] aadPrefixInProperties;
    // private final AADPrefixVerifier aadPrefixVerifier;


    // private HashMap < ColumnPath,
    // InternalColumnDecryptionSetup > columnMap;

    // in properties:
    // private EncryptionAlgorithm algorithm;
    // private byte[] fileAAD;
    // private boolean encryptedFooter;

    // private byte[] footerKeyMetaData;
    // private boolean fileCryptoMetaDataProcessed = false;
    // // private BlockCipher.Decryptor aesGcmDecryptorWithFooterKey;
    // // private BlockCipher.Decryptor aesCtrDecryptorWithFooterKey;
    // private boolean plaintextFile;
}

impl InternalFileDecryptor {
    fn getThriftModuleDecryptor(columnKey: Opion<String>) -> {
        if (null == columnKey) { // Decryptor with footer key
            if (null == aesGcmDecryptorWithFooterKey) {
                aesGcmDecryptorWithFooterKey = ModuleCipherFactory.getDecryptor(AesMode.GCM, footerKey);
            }
            return aesGcmDecryptorWithFooterKey;
        } else { // Decryptor with column key
            return ModuleCipherFactory.getDecryptor(AesMode.GCM, columnKey);
        }
    }
}