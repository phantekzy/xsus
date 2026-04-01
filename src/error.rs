use std::fmt;

#[derive(Debug)]
pub enum VivecError {
    Io(std::io::Error),
    Parse(String),
    Network(String),
    InvalidUrl(String),
    Timeout,
}

impl std::error::Error for VivecError {}

impl fmt::Display for VivecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {}
}
