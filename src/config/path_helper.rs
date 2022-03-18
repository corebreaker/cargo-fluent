use path_absolutize::Absolutize;
use simple_error::SimpleError;
use std::{path::{PathBuf, Path}, io::{Result, Error, ErrorKind}};

pub(super) fn path_join(base: &Path, path: &Path) -> Result<PathBuf> {
    base.join(path).absolutize().map(|p| p.to_path_buf())
}


pub(super) fn to_dirpath(pathname: String) -> Result<PathBuf> {
    let res = PathBuf::from(pathname);

    if !res.exists() {
        return Err(Error::new(ErrorKind::Other, SimpleError::new(format!("The path {:?} does not exist", res))));
    }

    if !res.is_dir() {
        return Err(Error::new(ErrorKind::Other, SimpleError::new(format!("The path {:?} is not a directory", res))));
    }

    Ok(res)
}