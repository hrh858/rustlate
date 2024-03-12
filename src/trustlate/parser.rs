use super::{config::Config, translations_tree::TranslationsTree};
use std::{collections::HashMap, error::Error, fs::File};

pub struct Parser<'a> {
    config: &'a Config,
}

impl<'a> Parser<'a> {
    pub fn from_config(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn parse_translation_files(
        &self,
    ) -> Result<HashMap<String, TranslationsTree>, Box<dyn Error>> {
        let mut trees_map: HashMap<String, TranslationsTree> = HashMap::new();

        let base = self.config.base_lang_filepath();
        let targets = self.config.target_lang_filepaths();

        for (lang, filepath) in std::iter::once(base).chain(targets) {
            let base_file = File::open(filepath)?;
            trees_map.insert(lang, serde_json::from_reader(base_file)?);
        }

        Ok(trees_map)
    }

    pub fn base_lang(&self) -> &str {
        &self.config.base_lang
    }

    pub fn target_languages(&self) -> &Vec<String> {
        &self.config.target_langs
    }
}
