
trait KmsClient {
    fn wrapKey(key: &[u8], key_id: String) -> String;
    fn unwrapKey(wrapped_key: &[u8], key_id: String) -> String;
}

