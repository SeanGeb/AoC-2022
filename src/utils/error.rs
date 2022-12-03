use std::error;
use std::io;

/// Takes any implementation of error::Error and turns it into an InvalidData
/// io::Error. The error encountered should indicate the file format is not as
/// expected (e.g. expecting lines of integers, but failed to parse an int).
pub fn invalid_data_err_from(e: impl error::Error) -> io::Error {
    invalid_data_err(&e.to_string())
}

/// Takes any error message string and turns it into an InvalidData io::Error.
/// The error encountered should indicate the file format is not as
/// expected (e.g. expecting lines of integers, but failed to parse an int).
pub fn invalid_data_err(e: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, e)
}
