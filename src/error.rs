use wax::BuildError;
use miette::Report;
use simple_error::SimpleError;
use std::io::{Error, ErrorKind};

pub(crate) fn error_to_string(err: &impl std::error::Error) -> String {
    err.to_string()
}

pub(crate) fn mk_error(kind: ErrorKind, msg: impl Into<String>) -> Error {
    Error::new(kind, SimpleError::new(msg))
}

pub(crate) fn mk_error_with_msg(msg: impl Into<String>) -> Error {
    Error::new(ErrorKind::Other, SimpleError::new(msg))
}

pub(crate) fn mk_error_with_err(err: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> Error {
    Error::new(ErrorKind::Other, err)
}

pub(crate) fn mk_error_with_msg_from_error(err: impl std::error::Error) -> Error {
    let src = match err.source() {
        Some(src) => format!("\n  Source: {}", src),
        None => String::new(),
    };

    let msg = format!("Error: {}{}", err, src);

    Error::new(ErrorKind::Other, SimpleError::new(msg))
}

pub(crate) fn mk_error_with_msg_from_glob_error(error: BuildError<'static>) -> Error {
    let report = Report::from(error);
    let msg = format!("Error on expression: {:?}", report);

    Error::new(ErrorKind::Other, SimpleError::new(msg))
}
