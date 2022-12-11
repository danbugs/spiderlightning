use anyhow::Result;
use spiderlightning::core::{secret::create_secret, slightfile::TomlFile};
use std::{fs::OpenOptions, path::Path};

pub fn handle_secret(key: &str, value: &str, toml_file_path: impl AsRef<Path>) -> Result<()> {
    let mut toml_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&toml_file_path)?;
    let toml_file_contents = std::fs::read_to_string(&toml_file_path)?;
    let mut toml = toml::from_str::<TomlFile>(&toml_file_contents)?;

    // if specversion is 0.1 -- set secret_store to configs.usersecrets
    if toml.specversion.eq("0.1") {
        toml.secret_store = Some("configs.usersecrets".to_string());
    }

    // removed global secret_store
    create_secret(key, value, &mut toml, &mut toml_file)
}
