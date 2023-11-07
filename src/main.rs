use std::error::Error;

mod config;
use config::{load_base_config, load_config};

mod unsorted_directory;
use unsorted_directory::UnsortedDirectory;

fn main() -> Result<(), Box<dyn Error>> {
    let base_config = load_base_config()?;

    let directories = base_config
        .get("directories")
        .and_then(|conf| conf.as_table())
        .unwrap();

    for (_, map) in directories.into_iter() {
        let Some(config_path) = map.get("config").and_then(|val| val.as_str()) else {
            println!("Directory does not specify a config path");
            continue;
        };

        let Ok(config) = load_config(config_path) else {
            println!("Specified config file does not exist");
            continue;
        };

        UnsortedDirectory::build(config_path, config)?.order()?;
    }

    Ok(())
}
