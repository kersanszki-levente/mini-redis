use bytes::Bytes;

use crate::Result;
use crate::frame::Frame;

#[derive(Debug)]
pub enum Command<'a> {
    Get(Getter<'a>),
    Set(Setter<'a>),
}

impl<'a> Command<'a> {
    pub fn from_frame(_frame: Frame) -> Result<Command<'a>> {
        Ok(Command::Get(Getter { key: "Hello" }))
    }
}

#[derive(Debug)]
pub struct Setter<'a> {
    key: &'a str,
    value: &'a Bytes,
}

impl<'a> Setter<'a> {
    pub fn key(&self) -> &'a str {
        self.key
    }
    pub fn value(&self) -> &'a Bytes {
        self.value
    }
}

#[derive(Debug)]
pub struct Getter<'a> {
    key: &'a str
}

impl<'a> Getter<'a> {
    pub fn key(&self) -> &'a str {
        self.key
    }
}
