use crate::config::Config;
use clap::ArgMatches;
use std::{io::Result, path::PathBuf};

pub fn command(_cmd: &ArgMatches, config: &Config) -> Result<()> {
    Ok(())
}