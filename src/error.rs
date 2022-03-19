use simple_error::SimpleError;
use std::io::{Error, ErrorKind};

#[inline]
pub(crate) fn mk_error_with_msg(msg: impl Into<String>) -> Error {
    Error::new(ErrorKind::Other, SimpleError::new(msg))
}

#[inline]
pub(crate) fn mk_error_with_err(err: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> Error {
    Error::new(ErrorKind::Other, err)
}

#[inline]
pub(crate) fn mk_error_with_msg_from_error(err: impl std::error::Error) -> Error {
    Error::new(ErrorKind::Other, SimpleError::new(err.to_string()))
}
