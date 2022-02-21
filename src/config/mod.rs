mod files;
mod read_toml;
mod create_config;

use std::{collections::HashMap, io::Result, env::current_dir};

pub struct Config {
    crates: HashMap<String, create_config::CrateConfig>
}

impl Config {
    pub fn read() -> Result<Config> {
        let cwd = current_dir()?;
        let mut crates = HashMap::new();

        for (name, path) in files::get_crates(&cwd)? {
            create_config::CrateConfig::import_config(name, path, &mut crates)?;
        }

        create_config::CrateConfig::import_config(String::from("."), cwd, &mut crates)?;

        Ok(Config { crates })
    }
}