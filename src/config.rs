use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml::Table;

const CONFIG_PATH: &str = "/usr/local/bin/Config.toml";

pub fn load_base_config() -> Result<Table, Box<dyn Error>> {
    let mut config_file = File::open(CONFIG_PATH).map_err(|_| "Base Config not found")?;

    let mut content = String::new();
    config_file.read_to_string(&mut content)?;

    let config = content.parse::<Table>()?;
    Ok(config)
}

pub fn load_config(config: impl AsRef<Path>) -> Result<Table, Box<dyn Error>> {
    let mut config_file = File::open(config)?;

    let mut content = String::new();
    config_file.read_to_string(&mut content)?;

    let config = content.parse::<Table>()?;
    Ok(config)
}
