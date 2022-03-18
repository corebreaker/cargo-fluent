use cargo_fluent::{commands::{cmd_scan, cmd_convert, cmd_edit}, config::Config, cli::CliArgs};
use clap::Parser;
use std::{ffi::CString, io::Result, process::exit};
use cargo_fluent::cli::CliCommand;

fn work() -> Result<()> {
    let cli: CliArgs = CliArgs::parse();

    if cli.quiet {
        let mode = CString::new("w")?;
        let dev_null = CString::new(if cfg!(windows) { "NUL" } else { "/dev/null" })?;

        unsafe {
            libc::freopen(dev_null.as_ptr(), mode.as_ptr(), libc::fdopen(1, mode.as_ptr()));
        }
    }

    let config = Config::read(cli.output)?;

    match cli.command {
        CliCommand::Scan(args) => cmd_scan(args, config),
        CliCommand::Convert(args) => cmd_convert(args, config),
        CliCommand::Edit => cmd_edit(config),
    }
}

fn main() {
    if let Err(err) = work() {
        eprintln!("{}", err);
        exit(1);
    }
}
