use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    /// Reached end of file while reading
    UnexpectedEof { position: usize, needed: usize },
    /// Invalid or unrecognized value for a field
    InvalidValue { field: &'static str, value: i32 },
    /// String encoding error
    EncodingError { position: usize },
    /// Unsupported file format or version
    UnsupportedVersion(String),
    /// Generic I/O error
    IoError(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedEof { position, needed } => {
                write!(f, "unexpected end of file at position {position}, needed {needed} more bytes")
            }
            ParseError::InvalidValue { field, value } => {
                write!(f, "invalid value {value} for field '{field}'")
            }
            ParseError::EncodingError { position } => {
                write!(f, "string encoding error at position {position}")
            }
            ParseError::UnsupportedVersion(v) => {
                write!(f, "unsupported file version: {v}")
            }
            ParseError::IoError(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for ParseError {}
