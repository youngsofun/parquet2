#[derive(Clone)]
pub struct AAD {
    file_aad: Vec<u8>,
    row_group_ordinal: i16,
    column_ordinal: i16,
    page_ordinal: i16,
}

#[derive(Copy, Clone)]
pub enum ModuleType {
    Footer = 0,
    ColumnMetaData = 1,
    DataPage = 2,
    DictionaryPage = 3,
    DataPageHeader = 4,
    DictionaryPageHeader = 5,
    ColumnIndex = 6,
    OffsetIndex = 7,
    BloomFilterHeader = 8,
    BloomFilterBitset = 9,
}

impl ModuleType {
    fn to_byte(&self) -> u8 {
        *self as u8
    }
}

impl AAD {
    pub fn new(file_aad: &[u8]) -> Self {
        AAD {
            file_aad: Vec::from(file_aad),
            row_group_ordinal: 0,
            column_ordinal: 0,
            page_ordinal: 0,
        }
    }

    pub(crate) fn with_row_group_ordinal(&self, ordinal: i16) -> Self {
        AAD {
            row_group_ordinal: ordinal,
            ..self.clone()
        }
    }

    pub(crate) fn with_column_ordinal(&self, ordinal: i16) -> Self {
        AAD {
            column_ordinal: ordinal,
            ..*self.clone()
        }
    }

    pub(crate) fn with_page_ordinal(&self, ordinal: i16) -> Self {
        AAD {
            page_ordinal: ordinal,
            ..*self.clone()
        }
    }

    pub fn footer_add(&self) -> Vec<u8> {
        let mut v = Vec::from(self.file_aad.clone());
        v.push(ModuleType::Footer.to_byte());
        v
    }

    pub fn column_chunk_aad(self, t: ModuleType) -> Vec<u8> {
        let mut v = Vec::with_capacity(self.file_aad.len() + 5);
        v.extend_from_slice(&self.fileADD);
        v.push(t.to_byte());
        v.extend_from_slice(&self.row_group_ordinal.to_le_bytes());
        v.extend_from_slice(&self.column_ordinal.to_le_bytes());
        v
    }

    // only for DataPage and DataPageHeader
    pub fn page_aad(self, t: ModuleType) -> Vec<u8> {
        let mut v = Vec::with_capacity(self.file_aad.len() + 7);
        v.ends_with(&self.fileADD);
        v.push(t.to_byte());
        v.extend_from_slice(&self.row_group_ordinal.to_le_bytes());
        v.extend_from_slice(&self.column_ordinal.to_le_bytes());
        v.extend_from_slice(&self.page_ordinal.to_le_bytes());
        v
    }
}
