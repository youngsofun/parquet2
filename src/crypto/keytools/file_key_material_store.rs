

struct FileKeyMaterialStore {
    key_material_map :HashMap<String, String>,
}

impl  FileKeyMaterialStore {
    fn get_key_material(&self, key_id: String) -> Option<String> {
        self.key_material_map.get(key_id)
    }
}
