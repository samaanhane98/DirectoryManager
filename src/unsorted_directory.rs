use std::{
    error::Error,
    fs::{create_dir, rename},
    path::PathBuf,
    rc::Rc,
};
use toml::{map::Map, Value};

type ExtensionMap = Map<String, Value>;

#[derive(Debug, Clone)]
pub struct UnsortedDirectory {
    root_path: PathBuf,
    extension_map: ExtensionMap,
    files: Rc<[PathBuf]>,
}

impl UnsortedDirectory {
    pub fn build(path: impl Into<PathBuf>, map: ExtensionMap) -> Result<Self, Box<dyn Error>> {
        // Determine root path
        let buffer: PathBuf = path.into();
        let root_path = buffer
            .parent()
            .map(|p| p.to_path_buf())
            .ok_or("Error extracting root path")?;

        // Extract extensions
        let extension_map = map
            .get("extensions")
            .and_then(|ext| ext.as_table())
            .ok_or("Config does not include extensions")?
            .to_owned();

        // Extract files
        let entries = root_path.read_dir()?;
        let files: Rc<[PathBuf]> = entries
            .into_iter()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_file() && !path.ends_with("config.toml"))
            .collect();

        Ok(UnsortedDirectory {
            root_path,
            extension_map,
            files,
        })
    }

    pub fn order(&self) -> Result<(), Box<dyn Error>> {
        for file in self.files.iter() {
            let file_extension = file
                .as_path()
                .extension()
                .ok_or("Error occurred determining path extension")?
                .to_str()
                .ok_or("Error occurred converting OS String")?;

            if let Some(folder) = self
                .extension_map
                .get(file_extension)
                .and_then(|f| f.as_str())
            {
                let destination = self.root_path.join(folder);
                move_file(file, destination)?;
            }
        }
        Ok(())
    }
}

fn move_file(src: &PathBuf, dst: PathBuf) -> Result<(), String> {
    if !src.is_file() {
        return Err("File to move is not a file".to_owned());
    }

    if !dst.is_dir() {
        create_dir(&dst).map_err(|_| "Failed to create directory")?;
    }

    if let Some(file_name) = src.file_name() {
        rename(src, dst.join(file_name)).map_err(|_| "Failed to move file")?;
    }
    Ok(())
}
