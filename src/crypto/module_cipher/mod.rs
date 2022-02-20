use crate::error::Result;
use crate::thrift_io_wrapper::{
    read_from_thrift, read_from_thrift_async, write_to_thrift, write_to_thrift_async, ThriftType,
};
use byteorder::{LittleEndian, ReadBytesExt};
use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use rand::{rngs::ThreadRng, thread_rng, RngCore};
use std::io::{Cursor, Read, Write};
use std::sync::Arc;

pub trait BlockCipher {
    fn encrypt_in_place(&self, buf: &mut Vec<u8>, nonce: [u8; 12], aad: &[u8]) -> Result<()>;
    fn decrypt_in_place(&self, buf: &mut Vec<u8>, nonce: [u8; 12], aad: &[u8]) -> Result<()>;
}

pub(crate) struct ModuleCipher {
    cipher: Arc<dyn BlockCipher>,
    rng: ThreadRng,
}

pub(crate) enum ModuleCipherMode {
    GCM,
    CTR,
}

use aes_ctr_cipher::AesCtrCipherFactory;
use aes_gcm_cipher::AesGcmCipherFactory;

mod aes_ctr_cipher;
mod aes_gcm_cipher;

impl ModuleCipher {
    pub fn new(mode: ModuleCipherMode, key: &[u8]) -> Result<Self> {
        let cipher = match mode {
            ModuleCipherMode::CTR => AesCtrCipherFactory::new(key)?,
            ModuleCipherMode::GCM => AesGcmCipherFactory::new(key)?,
        };
        Ok(ModuleCipher {
            cipher,
            rng: thread_rng(),
        })
    }

    pub fn read_from<R: Read>(&mut self, mut reader: R, aad: &[u8]) -> Result<Vec<u8>> {
        let len = reader.read_u32::<LittleEndian>()? as usize;
        let mut nonce = [0u8; 12];
        let mut buf = Vec::with_capacity(len - 12);
        reader.read_exact(&mut nonce)?;
        reader.read_exact(&mut buf)?;
        self.cipher.decrypt_in_place(&mut buf, nonce, aad)?;
        Ok(buf)
    }

    pub async fn read_from_async<R: AsyncRead + Unpin + Send>(
        &mut self,
        mut reader: R,
        aad: &[u8],
    ) -> Result<Vec<u8>> {
        let mut len_buf = [0u8; 4];
        reader.read_exact(&mut len_buf[..]).await?;
        let mut len_buf = Cursor::new(len_buf);
        let len = len_buf.read_u32::<LittleEndian>()? as usize;
        let mut nonce = [0u8; 12];
        let mut buf = Vec::with_capacity(len - 12);
        reader.read_exact(&mut nonce).await?;
        reader.read_exact(&mut buf).await?;
        self.cipher.decrypt_in_place(&mut buf, nonce, aad)?;
        Ok(buf)
    }

    pub fn write_to<W: Write>(
        &mut self,
        buf: &mut Vec<u8>,
        writer: &mut W,
        aad: &[u8],
    ) -> Result<usize> {
        let mut nonce = [0u8; 12];
        self.rng.fill_bytes(&mut nonce[..]);
        self.cipher.encrypt_in_place(buf, nonce, aad)?;
        let size_buf = (buf.len() as u32).to_le_bytes();
        assert_eq!(size_buf.len(), 4);
        writer.write_all(&size_buf)?;
        writer.write_all(&nonce)?;
        writer.write_all(&buf)?;
        Ok(16 + buf.len())
    }

    pub fn write_to_thrift<T: ThriftType, W: Write>(
        &mut self,
        v: &T,
        writer: &mut W,
        aad: &[u8],
    ) -> Result<usize> {
        let mut buf = Vec::with_capacity(128);
        write_to_thrift(v, &mut buf)?;
        self.write_to(&mut buf, writer, aad)
    }

    pub async fn write_to_async<W: AsyncWrite + Unpin + Send>(
        &mut self,
        buf: &mut Vec<u8>,
        writer: &mut W,
        aad: &[u8],
    ) -> Result<usize> {
        let mut nonce = [0u8; 12];
        self.rng.fill_bytes(&mut nonce[..]);
        self.cipher.encrypt_in_place(buf, nonce, aad)?;
        let size_buf = (buf.len() as u32).to_le_bytes();
        assert_eq!(size_buf.len(), 4);
        writer.write_all(&size_buf).await?;
        writer.write_all(&nonce).await?;
        writer.write_all(&buf).await?;
        Ok(16 + buf.len())
    }

    pub async fn write_to_thrift_async<T: ThriftType, W: AsyncWrite + Unpin + Send>(
        &mut self,
        v: &T,
        writer: &mut W,
        aad: &[u8],
    ) -> Result<usize> {
        let mut buf = Vec::with_capacity(128);
        write_to_thrift_async(v, &mut buf).await?;
        self.write_to_async(&mut buf, writer, aad).await
    }
}
