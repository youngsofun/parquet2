use super::key_material::KeyMaterial;

#[derive(Default)]
pub struct KeyMetadata {
    isInternalStorage: bool,
    keyReference: bool,
    keyMaterial :KeyMaterial
}


impl KeyMetadata {
    pub fn parse(key_metadata: &[u8]) -> KeyMetadata {
        KeyMetadata::default()
    }
}
