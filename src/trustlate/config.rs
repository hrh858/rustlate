use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use super::errors::TrustlateError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub base_lang: String,
    pub target_langs: Vec<String>,
    pub codegen: CodegenTarget,
    pub source_dir: PathBuf,
    pub target_dir: PathBuf,
}

impl Config {
    pub fn from_file(filepath: &Path) -> Result<Self, TrustlateError> {
        let f = File::open(filepath).map_err(|_| TrustlateError::OpenConfigFile)?;
        serde_json::from_reader(f).map_err(|_| TrustlateError::ParseConfigFile)
    }

    pub fn initialize(&self) -> Result<(), TrustlateError> {
        let config_file =
            File::create(".trustlaterc.json").map_err(|_| TrustlateError::InitCreateConfigFile)?;
        serde_json::to_writer_pretty(config_file, &self)
            .map_err(|_| TrustlateError::InitWriteConfigFile)?;
        fs::create_dir_all(&self.source_dir).map_err(|_| TrustlateError::InitCreateSourceDir)?;
        fs::create_dir_all(&self.target_dir).map_err(|_| TrustlateError::InitCreateTargetDir)?;

        let base_file = File::create(self.source_dir.join(format!("{}.json", self.base_lang)))
            .map_err(|_| TrustlateError::InitCreateTranslationsFile)?;
        serde_json::to_writer_pretty(base_file, &serde_json::json!({"examples": { "helloWorld": "Hola, Mundo!", "greeting": "Encantado de conocerte {{name}}!" }})).map_err(|_| TrustlateError::InitWriteTranslationsExample)?;

        let base_file = File::create(
            self.source_dir
                .join(format!("{}.json", self.target_langs[0])),
        )
        .map_err(|_| TrustlateError::InitCreateTranslationsFile)?;
        serde_json::to_writer_pretty(base_file, &serde_json::json!({"examples": { "helloWorld": "헬로, 월드!", "greeting": "{{name}} 만나서  방아워요!" }})).map_err(|_| TrustlateError::InitWriteTranslationsExample)?;

        let base_file = File::create(
            self.source_dir
                .join(format!("{}.json", self.target_langs[1])),
        )
        .map_err(|_| TrustlateError::InitCreateTranslationsFile)?;
        serde_json::to_writer_pretty(
            base_file,
            &serde_json::json!({"examples": { "helloWorld": "Hello, World!", "greeting": "Nice to meet you {{name}}" }}),
        )
        .map_err(|_| TrustlateError::InitWriteTranslationsExample)?;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            base_lang: "es".to_string(),
            target_langs: vec!["kr".to_string(), "en".to_string()],
            codegen: CodegenTarget::Typescript,
            source_dir: Path::new("./trustlate/translations/").to_path_buf(),
            target_dir: Path::new("./trustlate/codegens/").to_path_buf(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, clap::ValueEnum, Default, Clone)]
pub enum CodegenTarget {
    /// Generate Typescript code
    #[default]
    #[serde(rename = "ts", alias = "typescript")]
    Typescript,
    #[serde(rename = "go", alias = "golang")]
    Go,
}

// pub struct Config2<'a> {
//     pub base_lang: &'a str,
//     pub target_langs: &'a [&'a str],
//     pub codegen_targets: &'a [codegen::CodegenTarget],
//     pub input_dir: &'a Path,
//     pub output_dir: &'a Path
// }
//
// impl<'a> Default for Config2<'a> {
//     fn default() -> Self {
//         Self {
//             base_lang: "es",
//             target_langs: &["cat", "en"],
//             codegen_targets: &[CodegenTarget::Typescript],
//             input_dir: Path::new("./trustlate/inputs"),
//             output_dir: Path::new("./trustlate/outputs")
//         }
//     }
// }
//
// impl<'a> Config2<'a> {
//     pub fn get_input_filepath(&self) -> &'a Path {
//         self.input_dir.join(Path::new(""))
//     }
// }
