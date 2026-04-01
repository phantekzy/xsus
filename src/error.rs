#[derive(Debug)]
pub enum Name {
    Io(std::io::Error),
    Parse(String),
    Network(String),
    InvalidUrl(String),
    Timeout,
}
