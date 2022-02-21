use std::io::Write;

use parquet_format_async_temp::FileMetaData;

use crate::thrift_io_wrapper::write_to_thrift;
use parquet_format_async_temp::RowGroup;

pub use crate::metadata::KeyValue;
use crate::{
    error::{ParquetError, Result},
    metadata::SchemaDescriptor,
    FOOTER_SIZE, PARQUET_MAGIC, PARQUET_MAGIC_EF,
};

use super::{row_group::write_row_group, RowGroupIter, WriteOptions};

pub(super) fn start_file<W: Write>(writer: &mut W) -> Result<u64> {
    writer.write_all(&PARQUET_MAGIC)?;
    Ok(PARQUET_MAGIC.len() as u64)
}

use crate::crypto::{Encryptor, ModuleCipher, AAD};

pub(super) fn end_file<W: Write>(writer: &mut W, metadata: FileMetaData) -> Result<u64> {
    // Write metadata
    let metadata_len = write_to_thrift(&metadata, writer)?;

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

/// An interface to write a parquet file.
/// Use `start` to write the header, `write` to write a row group,
/// and `end` to write the footer.
pub struct FileWriter<W: Write> {
    writer: W,
    schema: SchemaDescriptor,
    options: WriteOptions,
    created_by: Option<String>,

    offset: u64,
    row_groups: Vec<RowGroup>,
    encryptor: Option<Encryptor>,
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
        Self {
            writer,
            schema,
            options,
            created_by,
            offset: 0,
            row_groups: vec![],
            encryptor: Some(Encryptor::new()),
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
        )?;
        self.offset += size;
        self.row_groups.push(group);
        Ok(())
    }

    /// Writes the footer of the parquet file. Returns the total size of the file and the
    /// underlying writer.
    pub fn end(mut self, key_value_metadata: Option<Vec<KeyValue>>) -> Result<(u64, W)> {
        // compute file stats
        let num_rows = self.row_groups.iter().map(|group| group.num_rows).sum();

        let encryption_algorithm = self
            .encryptor
            .as_ref()
            .map(|ref e| e.get_encryption_algorithm());
        let footer_signing_key_metadata = self
            .encryptor
            .as_ref()
            .and_then(|ref e| e.get_footer_signing_key_metadata());
        let metadata = FileMetaData::new(
            self.options.version.into(),
            self.schema.into_thrift()?,
            num_rows,
            self.row_groups,
            key_value_metadata,
            self.created_by,
            None,
            encryption_algorithm,
            footer_signing_key_metadata,
        );

        let (footer_len, magic) = if let Some(e) = self.encryptor {
            if e.encrypted_footer() {
                let mut cipher = e.get_footer_cipher();
                let crypto_metadata_len =
                    write_to_thrift(&e.get_file_crypto_metadata(), &mut self.writer)?;
                let metadata_len = cipher.write_to_thrift(
                    &metadata,
                    &mut self.writer,
                    &e.file_aad().footer_add(),
                )?;
                (metadata_len + crypto_metadata_len, PARQUET_MAGIC_EF)
            } else {
                let mut buf = vec![];
                write_to_thrift(&metadata, &mut buf)?;
                let signature = e.sign(&buf);
                self.writer.write_all(&signature);
                assert_eq!(signature.len(), 28);
                (buf.len() + signature.len(), PARQUET_MAGIC)
            }
        } else {
            (write_to_thrift(&metadata, &mut self.writer)?, PARQUET_MAGIC)
        };

        let mut footer_buffer = [0u8; FOOTER_SIZE as usize];
        (&mut footer_buffer[..4]).write_all(&(footer_len as u32).to_le_bytes())?;
        (&mut footer_buffer[4..]).write_all(&magic)?;
        self.writer.write_all(&footer_buffer)?;
        let file_len = self.offset + footer_len as u64 + FOOTER_SIZE;
        Ok((file_len, self.writer))
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
