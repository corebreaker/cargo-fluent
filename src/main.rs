use cargo_fluent::{commands::{cmd_scan, cmd_convert, cmd_edit}, config::Config};
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
            (about: "scan source files (rust files only) and create or update the FLT files")
            (after_help: "Help")
        )
        (@subcommand convert =>
            (version: crate_version!())
            (author: crate_authors!())
            (about: "scan po files and create or update the FLT files")
            (after_help: "Help")
        )
        (@subcommand edit =>
            (version: crate_version!())
            (author: crate_authors!())
            (about: "start a GUI for editing the FLT files")
            (after_help: "Help")
        )
    ).get_matches();

    if args.is_present("quiet") {
        let mode = CString::new("w")?;
        let dev_null = CString::new(if cfg!(windows) { "NUL" } else { "/dev/null" })?;

        unsafe {
            libc::freopen(dev_null.as_ptr(), mode.as_ptr(), libc::fdopen(1, mode.as_ptr()));
        }
    }

    let config = Config::read()?;

    if let Some(cmd) = args.subcommand_matches("scan") { cmd_scan(cmd, &config)? }
    if let Some(cmd) = args.subcommand_matches("convert") { cmd_convert(cmd, &config)? }
    if let Some(cmd) = args.subcommand_matches("edit") { cmd_edit(cmd, &config)? }

    Ok(())
}

fn main() {
    if let Err(err) = work() {
        eprintln!("{}", err);
        exit(1);
    }
}
