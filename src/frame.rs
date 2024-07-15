use std::io::{Cursor, Read};

use bytes::Bytes;

static VALID_TYPE_IDS: [char; 3] = ['+', '-', ':'];

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
                return Err(Error::Other(format!("ERR {err}").to_string()))
            }
        };
        if n == 0 {
            return Err(Error::Incomplete)
        }
        // Check for data type first, because the following branch may take long to finish
        if data.find(&VALID_TYPE_IDS).is_none() {
            return Err(Error::Other("ERR stream did not contain a type identifier".to_string()))
        }
        if !data.contains("\r\n") {
            return Err(Error::Incomplete)
        }
        match data.chars().into_iter().nth(0).unwrap() {
            '+' => Ok(()),
            '-' => Ok(()),
            ':' => Ok(()),
            _ => unimplemented!()
        }
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
        assert_eq!(output, Err(Error::Other("ERR stream did not contain valid UTF-8".into())));

        let incomplete_test_cases = vec![
            ("+", Error::Incomplete),
            ("+Hello World", Error::Incomplete),
        ];

        for (input_str, expected_output) in incomplete_test_cases {
            let mut func_input = Cursor::new(input_str.as_bytes());
            let output = Frame::check(&mut func_input);
            let Err(err_type) = output else {
                panic!("Frame::check returned Ok")
            };
            assert_eq!(err_type, expected_output);
        }

        let complete_test_cases = vec![
            "+Hello\r\n",
            "+Hello World\r\n",
            "-This is an error\r\n",
            ":1234\r\n",
        ];

        for input_str in complete_test_cases {
            let mut func_input = Cursor::new(input_str.as_bytes());
            let output = Frame::check(&mut func_input);
            if output.is_err() {
                panic!("Test case validation failed")
            }
        }
    }
}
