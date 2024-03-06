use std::error::Error;
use super::{config::Config, translations_tree::TranslationsTree};

pub struct Parser {
    config: Config
}

impl Parser {
    pub fn from_config(config: Config) -> Self {
        Self {
            config
        }
    }

    pub fn parse(&self) -> Result<TranslationsTree, Box<dyn Error>> {
        let base_input_filepaths = self.config.base_lang_filepath();
        let target_input_filepaths = self.config.target_lang_filepaths();
        dbg!(base_input_filepaths, target_input_filepaths);

        todo!()
    }
}
