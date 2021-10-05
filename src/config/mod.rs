mod files;
mod read_toml;
mod create_config;

use std::{path::PathBuf, collections::HashMap, io::Result, env::current_dir};

pub(super) struct Config {
    crates: HashMap<String, create_config::CrateConfig>
}

impl Config {
    pub(super) fn read() -> Result<Config> {
        let cwd = current_dir()?;
        let mut crates = HashMap::new();

        for (name, path) in files::get_crates(&cwd)? {
            create_config::CrateConfig::import_config(name, path, &mut crates)?
        }

        if let Some(conf) = files::read_i18n(&cwd)? {
            create_config::CrateConfig::import_config(String::from("."), cwd, &mut crates)?
        }

        Ok(Config { crates })
    }
}