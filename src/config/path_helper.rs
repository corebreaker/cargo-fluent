use crate::error::mk_error_with_msg;
use path_absolutize::Absolutize;
use std::{path::{PathBuf, Path}, io::Result, borrow::Cow};

#[inline]
fn make_path_from_cow_path(p: Cow<Path>) -> PathBuf {
    p.to_path_buf()
}

pub(super) fn path_join(base: &Path, path: &Path) -> Result<PathBuf> {
    base.join(path).absolutize().map(make_path_from_cow_path)
}

pub(super) fn make_dirpath(p: String) -> Result<PathBuf> {
    let res = PathBuf::from(p);

    if res.exists() && !res.is_dir() {
        Err(mk_error_with_msg(format!("If the path {:?} exists, it must be a directory", res)))
    } else {
        res.absolutize().map(make_path_from_cow_path)
    }
}
