use wax::GlobError;
use simple_error::SimpleError;
use std::io::{Error, ErrorKind};

#[inline]
pub(crate) fn mk_error(kind: ErrorKind, msg: impl Into<String>) -> Error {
    Error::new(kind, SimpleError::new(msg))
}

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
    let src = match err.source() {
        Some(src) => format!("\n  Source: {}", src),
        None => String::new(),
    };

    let msg = format!("Error: {}{}", err, src);

    Error::new(ErrorKind::Other, SimpleError::new(msg))
}

#[inline]
fn error_to_string(err: &impl std::error::Error) -> String {
    err.to_string()
}

#[inline]
fn get_expression_from_glob_error(err: &GlobError) -> String {
    match err {
        GlobError::Parse(err) => err.expression().to_string(),
        GlobError::Rule(err) => err.expression().to_string(),
        GlobError::Walk(err) => format!("{:?}", err.path()),
        _ => String::from("<No value>"),
    }
}

#[inline]
pub(crate) fn mk_error_with_msg_from_glob_error(err: GlobError) -> Error {
    let msg = format!("Error on expression {}: {}", get_expression_from_glob_error(&err), error_to_string(&err));

    Error::new(ErrorKind::Other, SimpleError::new(msg))
}
