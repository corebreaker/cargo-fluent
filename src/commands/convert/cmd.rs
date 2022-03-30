use super::{po_list::collect_po_files, registry::FluentFileRegistry};
use crate::{config::Config, cli::ConvertArgs as Args};
use itertools::Itertools;
use std::io::Result;

pub fn command(args: Args, config: Config) -> Result<()> {
    let output_dir = config.output();

    match config.po_dir() {
        None => { println!("Convert from PO files to {:?}", output_dir); }
        Some(input) => { println!("Convert from files {:?} to {:?}", input, output_dir); }
    }

    println!("Collect files:");
    let po_files = collect_po_files(&args, &config)?;
    let domains = po_files.iter().map(|input| input.domain.clone()).collect_vec();
    let domain = if args.merge || (domains.len() == 1) {
        Some(args.domain.unwrap_or_else(|| config.name().to_string()))
    } else {
        None
    };

    let mut reg = FluentFileRegistry::new();

    println!();
    println!("Convert files:");
    for file in po_files {
        print!(" - {}", file.path.to_string_lossy());

        let language = file.language.clone();
        let domain = domain.clone().unwrap_or_else(|| file.domain.clone());
        let into = match reg.fetch(output_dir, language, domain) {
            Ok(v) => v,
            Err(err) => {
                println!(": {}", err);
                continue;
            }
        };

        println!();
        file.convert(into, args.include_fuzzy);
    }

    println!();
    println!("Write files:");
    for file in reg.drain() {
        if let Err(err) = file.write(output_dir) {
            print!(": {}", err);
        }

        println!();
    }

    Ok(())
}