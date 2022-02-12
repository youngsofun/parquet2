#[derive(Default)]
pub struct KeyMaterial {
    is_footer_key: bool,
    master_key_id: String,
    encoded_wrapped_dek: String,

    kms_instance_url: String,
    kms_instance_id: String,

    is_double_wrapped: bool,
    kek_id: String,
    encoded_wrapped_kek: String,
}

impl KeyMaterial {
    fn parse(key_material_bytes :&[u8]) -> KeyMaterial {
        KeyMaterial::default()
    }
}