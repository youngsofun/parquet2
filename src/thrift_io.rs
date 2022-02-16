use parquet_format_async_temp::thrift::protocol::{TCompactInputProtocol, TCompactOutputProtocol};
use crate::page::ParquetPageHeader;

trait ThriftT {
    fn to_vec(&self) -> Vec<u8>;
    fn into_buf(&self, buf: &mut [u8]) -> Vec<u8>;
}

impl ThriftT for ParquetPageHeader {
    fn to_vec(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(100); // todo!
        let mut protocol = TCompactOutputProtocol::new(&buf);
        header.write_to_out_protocol(&mut protocol)?;
        buf
    }

    fn into_buf(&self, buf: &mut [u8]) -> Vec<u8> {
        todo!()
    }
}

/// Reads Page header from Thrift.
fn read_page_header(&mut self) -> ParquetPageHeader {
    let mut prot = TCompactInputProtocol::new(&mut self.reader);
    let page_header = ParquetPageHeader::read_from_in_protocol(&mut prot)?;
    Ok(page_header)
}