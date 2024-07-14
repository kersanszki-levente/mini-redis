use std::io::{Cursor, Read};

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
    pub fn check(buffer: &mut Cursor<&[u8]>) -> Result<(), Error> {
        let mut data = String::new();
        let n = match buffer.to_owned().read_to_string(&mut data) {
            Ok(n) => n,
            Err(err) => {
                return Result::Err(Error::Other(err.to_string()))
            }
        };
        if n == 0 {
            return Err(Error::Incomplete)
        }
        if !data.contains("\r\n") {
            return Err(Error::Incomplete)
        }
        Ok(())
    }
    pub fn parse(_buffer: &mut Cursor<&[u8]>) -> crate::Result<Frame> {
        Ok(Frame::Null)
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Incomplete,
    Other(String),
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn frame_check() {

        let err_test_case: [u8; 4] = [255, 255, 255, 255];
        let mut func_input = Cursor::new(&err_test_case[..]);
        let output = Frame::check(&mut func_input);
        assert_eq!(output, Err(Error::Other("stream did not contain valid UTF-8".into())));

        let incomplete_test_cases = vec![
            ("", Error::Incomplete),
            ("Hello World", Error::Incomplete),
        ];

        for (input_str, expected_output) in incomplete_test_cases {
            let mut func_input = Cursor::new(input_str.as_bytes());
            let output = Frame::check(&mut func_input);
            let Err(err_type) = output else {
                panic!("Frame::check returned Ok")
            };
            assert_eq!(err_type, expected_output);
        }
    }
}
