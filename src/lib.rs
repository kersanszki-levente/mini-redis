pub mod command;
pub mod connection;
pub mod db;
pub mod frame;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;
