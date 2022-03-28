use super::po_file::InputPoFile;
use crate::{config::Config, cli::ConvertArgs as Args, error::{mk_error, mk_error_with_msg_from_error}};
use wax::Glob;
use std::{io::{Result, ErrorKind}, env::current_dir};

#[inline]
fn find_po_files(args: &Args, config: &Config) -> Result<Vec<InputPoFile>> {
    let mut res = vec![];

    if let Some(files) = args.po_files() {
        for path in files {
            let file = InputPoFile::read(path)?;

            if let Some(domain) = &args.domain {
                if &file.domain != domain {
                    continue;
                }
            }

            res.push(file);
        }

        if !res.is_empty() {
            return Ok(res);
        }
    }

    let cwd = current_dir()?;
    let po_dir = match config.po_dir() {
        None => cwd.as_path(),
        Some(v) => v,
    };

    let expression = if let Some(domain) = &args.domain {
        format!("**/{}.po", domain)
    } else {
        String::from("**/*.po")
    };

    let glob = Glob::new(&expression).map_err(mk_error_with_msg_from_error)?;

    for entry in glob.walk(po_dir, usize::MAX) {
        let entry = entry.map_err(mk_error_with_msg_from_error)?;

        res.push(InputPoFile::read(entry.path())?);
    }

    Ok(res)
}

pub(super) fn collect_po_files(args: &Args, config: &Config) -> Result<Vec<InputPoFile>> {
    find_po_files(args, config).and_then(|r| if r.is_empty() {
        Err(mk_error(ErrorKind::NotFound, "No PO file found"))
    } else {
        Ok(r)
    })
}
