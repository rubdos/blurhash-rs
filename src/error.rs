use core::fmt;

#[derive(Debug)]
pub enum Error {
    HashTooShort,
    LengthMismatch { expected: usize, actual: usize },
    InvalidAscii,
    InvalidBase83(u8),
    ComponentsOutOfRange,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::HashTooShort => write!(f, "blurhash must be at least 6 characters long"),
            Error::LengthMismatch { expected, actual } => write!(
                f,
                "blurhash length mismatch: length is {} but it should be {}",
                actual, expected
            ),
            Error::InvalidBase83(byte) => {
                write!(f, "Invalid base83 character: {:?}", *byte as char)
            }
            Error::InvalidAscii => write!(f, "blurhash must be valid ASCII"),
            Error::ComponentsOutOfRange => {
                write!(f, "blurhash must have between 1 and 9 components")
            }
        }
    }
}

impl core::error::Error for Error {}
