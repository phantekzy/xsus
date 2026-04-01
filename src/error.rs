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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VivecError::Io(e) => write!(f, "IO Error : {}", e),
            VivecError::Parse(s) => write!(f, "Parsing Error : {}", s),
            VivecError::Network(s) => writeln!(f, "Network Error : {}", s),
            VivecError::InvalidUrl(u) => writeln!(f, "Invalid URL : {}", u),
            VivecError::Timeout => writeln!(f, "Request timed out"),
        }
    }
}

impl From<std::io::Error> for VivecError {
    fn from(err: std::io::Error) -> Self {
        VivecError::Io(err)
    }
}
