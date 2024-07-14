use std::io::Cursor;

use bytes::{Buf, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

use crate::frame::Frame;

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    fn new(stream: TcpStream) -> Self {
        Self {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(4096),
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>, crate::Error> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame))
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None)
                } else {
                    return Err("Connection reset by peer".into())
                }
            }
        }
    }

    fn parse_frame(&mut self) -> crate::Result<Option<Frame>> {
        use crate::frame::Error::{Incomplete, Other};

        let mut buffer = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut buffer) {
            Ok(_) => {
                let len = buffer.position() as usize;
                buffer.set_position(0);
                let frame = Frame::parse(&mut buffer)?;
                self.buffer.advance(len);
                Ok(Some(frame))
            },
            Err(Incomplete) => Ok(None),
            Err(Other(e)) => Err(e.into()),
        }
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> crate::Result<()> {
        match frame {
            Frame::Simple(value) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(value.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            },
            Frame::Error(value) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(value.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            },
            Frame::Integer(value) => {
                self.stream.write_u8(b':').await?;
                self.stream.write_u64(*value).await?;
                self.stream.write_all(b"\r\n").await?;
            },
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            },
            Frame::Bulk(value) => {
                let len = value.len() as u64;

                self.stream.write_u8(b'$').await?;
                self.stream.write_u64(len).await?;
                self.stream.write_all(value).await?;
                self.stream.write_all(b"\r\n").await?;
            },
            Frame::Array(_value) => {
                // self.stream.write_u8(b'*').await?;
                unimplemented!()
            }
        };

        self.stream.flush().await?;

        Ok(())
    }
}
