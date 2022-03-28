use super::{po_list::collect_po_files, registry::FluentFileRegistry};
use crate::{config::Config, cli::ConvertArgs as Args};
use itertools::Itertools;
use std::io::Result;

pub fn command(args: Args, config: Config) -> Result<()> {
    let po_files = collect_po_files(&args, &config)?;
    let domains = po_files.iter().map(|input| input.domain.clone()).collect_vec();
    let domain = if args.merge || (domains.len() == 1) {
        Some(args.domain.unwrap_or_else(|| config.name().to_string()))
    } else {
        None
    };

    let mut reg = FluentFileRegistry::new();
    let output_dir = config.output();

    for file in po_files {
        let language = file.language.clone();
        let domain = domain.clone().unwrap_or_else(|| file.domain.clone());
        let into = reg.fetch(output_dir, language, domain)?;

        file.convert(into, args.include_fuzzy);
    }

    for file in reg.drain() {
        file.write(output_dir)?;
    }

    Ok(())
}