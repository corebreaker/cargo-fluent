use crate::{config::Config, cli::ConvertArgs as Args};
use std::io::Result;

pub fn command(args: Args, config: Config) -> Result<()> {
    println!("Args:   {:?}, {:?}", args.domain, args.po_files());
    println!("Config: {:?}", config);

    Ok(())
}