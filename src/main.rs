use cargo_fluent::{arg_validators::{file_exists}, commands::{cmd_scan}};
use clap::{clap_app, crate_authors, crate_version, crate_description};
use std::{ffi::CString, io::Result, process::exit};

fn work() -> Result<()> {
    let args = clap_app!(CargoFluent =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@setting SubcommandRequiredElseHelp)
        (@arg quiet: -q --quiet !required +global "Quiet mode, don't show any output, only errors")
        (@subcommand scan =>
            (version: crate_version!())
            (author: crate_authors!())
            (about: "")
            (after_help: "Help")
            (@arg PATH: * ... {file_exists} "")
            (@arg dest: -d --dest <path> "")
        )
    ).get_matches();

    if args.is_present("quiet") {
        let mode = CString::new("w")?;
        let dev_null = CString::new(if cfg!(windows) { "NUL" } else { "/dev/null" })?;

        unsafe {
            libc::freopen(dev_null.as_ptr(), mode.as_ptr(), libc::fdopen(1, mode.as_ptr()));
        }
    }

    if let Some(cmd) = args.subcommand_matches("scan") { cmd_scan(cmd)? }

    Ok(())
}

fn main() {
    if let Err(err) = work() {
        eprintln!("{}", err);
        exit(1);
    }
}
