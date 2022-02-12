use std::io::Write;

use parquet_format_async_temp::FileMetaData;

use parquet_format_async_temp::thrift::protocol::TCompactOutputProtocol;
use parquet_format_async_temp::thrift::protocol::TOutputProtocol;
use parquet_format_async_temp::RowGroup;

use crate::crypto::aad::AAD;
use crate::crypto::block_encryptor::BlockEncryptor;
pub use crate::metadata::KeyValue;
use crate::write::FileEncryptor;
use crate::{
    error::{ParquetError, Result},
    metadata::SchemaDescriptor,
    FOOTER_SIZE, PARQUET_MAGIC, PARQUET_MAGIC_EF,
};

use super::{row_group::write_row_group, RowGroupIter, WriteOptions};

pub(super) fn end_file<W: Write>(mut writer: &mut W, metadata: FileMetaData) -> Result<u64> {
    // Write metadata
    let mut protocol = TCompactOutputProtocol::new(&mut writer);
    let metadata_len = metadata.write_to_out_protocol(&mut protocol)? as i32;
    protocol.flush()?;

    // Write footer
    let metadata_bytes = metadata_len.to_le_bytes();
    let mut footer_buffer = [0u8; FOOTER_SIZE as usize];
    (0..4).for_each(|i| {
        footer_buffer[i] = metadata_bytes[i];
    });

    (&mut footer_buffer[4..]).write_all(&PARQUET_MAGIC)?;
    writer.write_all(&footer_buffer)?;
    Ok(metadata_len as u64 + FOOTER_SIZE)
}

pub(super) fn end_file_e<W: Write>(
    mut writer: &mut W,
    metadata: FileMetaData,
    encryptor: BlockEncryptor,
    aad: AAD,
) -> Result<u64> {
    // write FileCryptoMetaData
    let file_crypto_metadata = encryptor.get_file_crypto_metadata();
    let mut protocol = TCompactOutputProtocol::new(&mut writer);
    let crypto_metadata_len = file_crypto_metadata.write_to_out_protocol(&mut protocol)?;
    protocol.flush()?;

    // Write metadata
    let metadata_encoded = encode_thrift(&metadata);
    let metadata_len = encryptor.write(&mut writer, &metadata_encoded, &aad.footer_add());
    let footer_len = metadata_len + crypto_metadata_len;

    // Write footer
    let footer_bytes = footer_len.to_le_bytes();
    let mut footer_buffer = [0u8; FOOTER_SIZE as usize];
    (0..4).for_each(|i| {
        footer_buffer[i] = footer_bytes[i];
    });

    (&mut footer_buffer[4..]).write_all(&PARQUET_MAGIC_EF)?;
    writer.write_all(&footer_buffer)?;
    Ok(footer_len as u64 + FOOTER_SIZE)
}

/// An interface to write a parquet file.
/// Use `start` to write the header, `write` to write a row group,
/// and `end` to write the footer.
pub struct FileWriter<W: Write> {
    writer: W,
    schema: SchemaDescriptor,
    options: WriteOptions,
    created_by: Option<String>,
    encryptor: Option<FileEncryptor>,
    offset: u64,
    row_groups: Vec<RowGroup>,
}

// Accessors
impl<W: Write> FileWriter<W> {
    /// The options assigned to the file
    pub fn options(&self) -> &WriteOptions {
        &self.options
    }

    /// The [`SchemaDescriptor`] assigned to this file
    pub fn schema(&self) -> &SchemaDescriptor {
        &self.schema
    }
}

impl<W: Write> FileWriter<W> {
    /// Returns a new [`FileWriter`].
    pub fn new(
        writer: W,
        schema: SchemaDescriptor,
        options: WriteOptions,
        created_by: Option<String>,
    ) -> Self {
        let encryptor = Some(FileEncryptor::new());
        // todo: check options.encryptor with schema
        Self {
            writer,
            schema,
            options,
            encryptor,
            created_by,
            offset: 0,
            row_groups: vec![],
        }
    }

    /// Writes the header of the file
    pub fn start(&mut self) -> Result<()> {
        let magic = if self.encryptor.is_none() {
            &PARQUET_MAGIC
        } else {
            &PARQUET_MAGIC_EF
        };
        self.writer.write_all(magic)?;
        self.offset = magic.len() as u64;
        Ok(())
    }

    /// Writes a row group to the file.
    ///
    /// This call is IO-bounded
    pub fn write<E>(&mut self, row_group: RowGroupIter<'_, E>, num_rows: usize) -> Result<()>
    where
        ParquetError: From<E>,
        E: std::error::Error,
    {
        if self.offset == 0 {
            return Err(ParquetError::General(
                "You must call `start` before writing the first row group".to_string(),
            ));
        }
        let (group, size) = write_row_group(
            &mut self.writer,
            self.offset,
            self.schema.columns(),
            self.options.compression,
            row_group,
            num_rows,
            self.encryptor.clone(),
            self.row_groups.len() as i16,
        )?;
        self.offset += size;
        self.row_groups.push(group);
        Ok(())
    }

    /// Writes the footer of the parquet file. Returns the total size of the file and the
    /// underlying writer.
    pub fn end(mut self, key_value_metadata: Option<Vec<KeyValue>>) -> Result<(u64, W)> {
        let footer_encryptor = BlockEncryptor::new();
        // compute file stats
        let num_rows = self.row_groups.iter().map(|group| group.num_rows).sum();

        let metadata = FileMetaData::new(
            self.options.version.into(),
            self.schema.into_thrift()?,
            num_rows,
            self.row_groups,
            key_value_metadata,
            self.created_by,
            None,
            None,
            None,
        );

        let len = if let Some(e) = self.encryptor {
            end_file_e(
                &mut self.writer,
                metadata,
                e.get_block_encryptor_file(),
                e.file_aad(),
            )?;
        } else {
            end_file(&mut self.writer, metadata)?;
        };

        Ok((self.offset + len, self.writer))
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Cursor};

    use super::*;

    use crate::error::Result;
    use crate::read::read_metadata;
    use crate::tests::get_path;

    #[test]
    fn empty_file() -> Result<()> {
        let mut testdata = get_path();
        testdata.push("alltypes_plain.parquet");
        let mut file = File::open(testdata).unwrap();

        let mut metadata = read_metadata(&mut file)?;

        // take away all groups and rows
        metadata.row_groups = vec![];
        metadata.num_rows = 0;

        let mut writer = Cursor::new(vec![]);

        // write the file
        start_file(&mut writer)?;
        end_file(&mut writer, metadata.into_thrift()?)?;

        let a = writer.into_inner();

        // read it again:
        let result = read_metadata(&mut Cursor::new(a));
        assert!(result.is_ok());
        Ok(())
    }
}

// todo: use macro
fn encode_thrift(file_metadata: &FileMetaData) -> Vec<u8> {
    let mut buf = Vec::with_capacity(100); // todo!
    let mut protocol = TCompactOutputProtocol::new(&buf);
    file_metadata.write_to_out_protocol(&mut protocol)?;
    buf
}
