use serde::Deserialize;
use std::{error::Error, fs::File};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub base_lang: String,
    pub target_langs: Vec<String>,
    pub codegen: String,
    pub source_dir: String,
    pub target_dir: String,
}

impl Config {
    pub fn form_json_file(file: File) -> Result<Self, Box<dyn Error>> {
        let config: Config = serde_json::from_reader(file)?;
        Ok(config)
    }

    pub fn base_lang_filepath(&self) -> (String, String) {
        (
            self.base_lang.clone(),
            format!("{}/{}.json", self.source_dir, self.base_lang),
        )
    }

    pub fn target_lang_filepaths(&self) -> Vec<(String, String)> {
        self.target_langs
            .iter()
            .map(|target_lang| {
                (
                    target_lang.clone(),
                    format!("{}/{}.json", self.source_dir, target_lang),
                )
            })
            .collect()
    }
}
