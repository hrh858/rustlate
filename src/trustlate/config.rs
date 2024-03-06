use std::{error::Error, fs::File, path::{Path, PathBuf}};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    base_lang: String,
    target_langs: Vec<String>,
    source_dir: String,
    target_dir: String,
}

impl Config {
    pub fn form_json_file(file: File) -> Result<Self, Box<dyn Error>> {
        let config: Config = serde_json::from_reader(file)?;
        Ok(config)
    }

    pub fn base_lang_filepath(&self) -> String {
        format!("{}/{}.json", self.source_dir, self.base_lang)
    }

    pub fn target_lang_filepaths(&self) -> Vec<String> {
        self.target_langs.iter().map(|target_lang| format!("{}/{}.json", self.source_dir, target_lang)).collect()
    }
}