use std::io::Cursor;

use bytes::Bytes;

pub enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<Frame>)
}

impl Frame {
    pub fn check(_buffer: &mut Cursor<&[u8]>) -> Result<(), Error> {
        Err(Error::Incomplete)
    }
    pub fn parse(_buffer: &mut Cursor<&[u8]>) -> crate::Result<Frame> {
        Ok(Frame::Null)
    }
}

pub enum Error {
    Incomplete,
    Other(crate::Error),
}
