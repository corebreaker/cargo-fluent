use crate::error::mk_error_with_msg_from_error;
use wax::{Glob, GlobError};
use std::{path::PathBuf, io::Result};

#[cfg(windows)]
#[inline]
fn format_pattern_to_expression(pattern: &str) -> String {
    pattern.replace("\\", "/")
}

#[cfg(not(windows))]
#[inline]
fn format_pattern_to_expression(pattern: &str) -> String {
    pattern.to_string()
}

#[cfg(windows)]
#[inline]
fn format_expression_to_path(expression: &str) -> String {
    expression.replace("/", "\\")
}

#[cfg(not(windows))]
#[inline]
fn format_expression_to_path(expression: &str) -> String {
    expression.to_string()
}

pub(super) fn parse_path_pattern(pattern: &str) -> Result<Vec<PathBuf>> {
    let mut expression = format_pattern_to_expression(pattern);

    if expression.starts_with("./") {
        expression.replace_range(0..2, "");
    }

    let glob = Glob::new(&expression).map_err(mk_error_with_msg_from_error)?;

    if glob.is_invariant() {
        let path = PathBuf::from(expression);

        if !path.is_dir() {
            return Ok(vec![]);
        }

        let expression = format!("{}/**", format_expression_to_path(&path.to_string_lossy()));

        return parse_path_pattern(&expression);
    }

    let mut res = vec![];

    for path in glob.walk(".", usize::MAX) {
        let entry = path.map_err(mk_error_with_msg_from_error)?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        if let Some(ext) = path.extension() {
            if ext == "po" {
                res.push(path.to_path_buf());
            }
        }
    }

    Ok(res)
}
