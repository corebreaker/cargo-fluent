use crate::path_utils::{format_expression_to_path, format_pattern_to_expression, path_to_string, trim_last_separator};
use crate::error::{mk_error_with_msg_from_error, mk_error_with_msg_from_glob_error};
use wax::{Glob, Pattern};
use simple_error::SimpleError;
use std::{path::{PathBuf, Component}, io::{Result, Error, ErrorKind}};

fn check_expression(expression: &String) -> Result<()> {
    let path = PathBuf::from(expression);

    if path.is_absolute() || path.has_root() {
        return Err(Error::new(ErrorKind::Unsupported, SimpleError::new("Absolute paths are not supported")));
    }

    for part in path.components() {
        match part {
            Component::Normal(_) => { break; }
            Component::ParentDir => {
                let msg = "Relative paths prefixed with parent directory are not supported";

                return Err(Error::new(ErrorKind::Unsupported, SimpleError::new(msg)));
            }
            _ => { continue; }
        }
    }

    Ok(())
}

fn scan_directory(path: PathBuf) -> Result<Vec<PathBuf>> {
    if !path.exists() {
        let msg = format!("Path not found: `{}`", path.display());

        return Err(Error::new(ErrorKind::NotFound, SimpleError::new(msg)));
    }

    let real_path = format_expression_to_path(&path_to_string(&path));
    let pattern = format!("{}/**/*.po", trim_last_separator(&real_path));

    parse_path_pattern_from_base(&pattern, Some(path))
}

fn parse_path_pattern_from_base(pattern: &str, base_path: Option<PathBuf>) -> Result<Vec<PathBuf>> {
    let mut expression = format_pattern_to_expression(pattern);

    while expression.starts_with("./") {
        expression.replace_range(0..2, "");
    }

    check_expression(&expression)?;

    let glob = Glob::new(&expression).map_err(|err| mk_error_with_msg_from_glob_error(err.into_owned()))?;
    let mut res = vec![];

    for path in glob.walk(".") {
        let entry = path.map_err(mk_error_with_msg_from_error)?;
        let entry_path = entry.path();

        if !entry_path.exists() {
            println!("Warning: The path `{}` was not found", entry_path.display());
            continue;
        }

        if base_path.as_ref().map_or(false, |p| p == entry_path) {
            continue;
        }

        if entry_path.is_dir() {
            res.extend(scan_directory(entry_path.to_path_buf())?);
            continue;
        }

        if let Some(ext) = entry_path.extension() {
            if ext == "po" {
                res.push(entry_path.to_path_buf());
            }
        }
    }

    let x = res.iter().map(|v| v.display().to_string()).collect::<Vec<_>>();

    Ok(res)
}

pub(super) fn parse_path_pattern(pattern: &str) -> Result<Vec<PathBuf>> {
    let mut expression = format_pattern_to_expression(pattern);

    if expression.starts_with("./") {
        expression.replace_range(0..2, "");
    }

    let glob = Glob::new(&expression).map_err(|err| mk_error_with_msg_from_glob_error(err.into_owned()))?;

    if glob.variance().is_invariant() {
        let path = PathBuf::from(expression);

        if path.is_dir() {
            scan_directory(path)
        } else {
            if let Some(ext) = path.extension() {
                if ext == "po" {
                    return Ok(vec![path]);
                }
            }

            Ok(vec![])
        }
    } else {
        parse_path_pattern_from_base(&expression, None)
    }
}