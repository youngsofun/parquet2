use crate::error;
use futures::{AsyncWrite, AsyncWriteExt};
use parquet_format_async_temp::{
    AesGcmV1, ColumnCryptoMetaData, EncryptionAlgorithm, FileCryptoMetaData,
};
use std::io::Write;

#[derive(Clone)]
pub struct BlockEncryptor {}

impl BlockEncryptor {
    pub fn new() -> Self {
        BlockEncryptor {}
    }
}

impl BlockEncryptor {
    pub fn encrypt(&self, plaintext: &[u8], aad: &[u8]) -> Vec<u8> {
        vec![b'1']
    }
    pub fn write<W: Write>(
        &self,
        mut writer: &mut W,
        plaintext: &[u8],
        aad: &[u8],
    ) -> error::Result<usize> {
        let buf = self.encrypt(plaintext, aad);
        writer.write_all(&buf)?;
        Ok(buf.len())
    }
    pub async fn write_async<W: AsyncWrite + Unpin + Send>(
        &self,
        writer: &mut W,
        plaintext: &[u8],
        aad: &[u8],
    ) -> error::Result<usize> {
        let buf = self.encrypt(plaintext, aad);
        writer.write_all(&buf).await?;
        Ok(buf.len())
    }

    pub fn alg(&self) -> EncryptionAlgorithm {
        EncryptionAlgorithm::AESGCMV1(AesGcmV1 {
            aad_prefix: None,
            aad_file_unique: None,
            supply_aad_prefix: None,
        })
    }

    fn key_meta(&self) -> Vec<u8> {
        vec![]
    }

    pub fn get_column_crypto_metadata(&self) -> ColumnCryptoMetaData {
        todo!()
    }

    pub fn get_file_crypto_metadata(&self) -> FileCryptoMetaData {
        todo!()
    }
}
