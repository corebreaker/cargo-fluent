use crate::{config::Config, cli::ConvertArgs as Args};
use std::io::Result;

pub fn command(args: Args, config: Config) -> Result<()> {
    println!("Config: {:?}", config);

    Ok(())
}