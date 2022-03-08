use crate::config::Config;
use clap::ArgMatches;
use std::io::Result;

pub fn command(_cmd: &ArgMatches, config: &Config) -> Result<()> {
    Ok(())
}