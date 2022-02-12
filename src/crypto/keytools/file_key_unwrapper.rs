use std::collections::HashMap;
use super::key_metadata::KeyMetadata;
use crate::crypto::decryptor::DecryptionKeyRetriever;

struct FileKeyUnwrapper {
    //A map of KEK_ID -> KEK bytes, for the current token
    kekPerKekID: HashMap<String, Vec<u8>>,
}

impl DecryptionKeyRetriever for FileKeyUnwrapper {
    fn get_key(key_metadata: &[u8]) -> Vec<u8> {
        let key_metadata = KeyMetadata::parse(key_metadata);
        let key_
    }
}