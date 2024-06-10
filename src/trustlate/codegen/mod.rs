pub mod go;
pub mod typescript;

use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

use go::generate_golang;
use typescript::{generate_typescript_index, genererate_typescript};

use super::{
    config::{CodegenTarget, Config},
    errors::TrustlateError,
    translations_tree::TranslationsTree,
};

pub fn generate(
    config: &Config,
    tree: &HashMap<String, TranslationsTree>,
) -> Result<(), TrustlateError> {
    match config.codegen {
        CodegenTarget::Typescript => {
            for (lang, translations) in tree {
                let (code, extension) = (
                    genererate_typescript(translations)
                        .map_err(|_| TrustlateError::GenerateCannotGenerateCode)?,
                    "ts",
                );
                save_translation_file(config, lang, extension, &code)?;
            }

            let mut f =
                File::create(config.target_dir.join(format!("index.ts"))).map_err(|err| {
                    println!("Path: {:?}", config.target_dir.join("index.ts"));
                    eprint!("Error when creating output file: {}", err);
                    TrustlateError::GenerateCannotCreateOutputFile
                })?;
            let mut langs: Vec<&str> = config.target_langs.iter().map(|l| l.as_str()).collect();
            langs.push(&config.base_lang);
            write!(f, "{}", generate_typescript_index(&langs))
                .map_err(|_| TrustlateError::GenerateCannotWriteToOutputFile)?;
        }
        CodegenTarget::Go => {
            let mut content = String::from("package trustlate\n\nimport \"fmt\"\n\n");
            let mut langs: Vec<&String> = vec![];
            for (lang, translations) in tree {
                langs.push(lang);
                let is_main_lang = config.base_lang == *lang;
                let generations = generate_golang(translations.clone(), lang)
                    .map_err(|_| TrustlateError::GenerateCannotGenerateCode)?;

                if is_main_lang {
                    content += "type Trustlate interface {\n";
                    for gen in &generations {
                        content += &("    ".to_string() + &gen.interface_entry_form() + "\n");
                    }
                    content += "}\n\n";
                }

                content += format!("type Trustlate{} struct{{}}\n\n", lang.to_uppercase()).as_str();
                for gen in &generations {
                    content += format!("{}\n\n", gen.function_form()).as_str();
                }
            }
            for lang in &langs {
                content += format!(
                    "var trustlate{} = Trustlate{}{{}}\n",
                    lang.to_uppercase(),
                    lang.to_uppercase()
                )
                .as_str();
            }
            content += "\n";
            content += format!("func GetTrustlate(lang string) Trustlate {{\n    switch lang {{\n")
                .as_str();
            for lang in &langs {
                content += format!(
                    "    case \"{lang}\":\n      return &trustlate{}\n",
                    lang.to_uppercase()
                )
                .as_str();
            }
            content += format!(
                "    default:\n     return &trustlate{}\n   }}\n}}",
                langs.first().unwrap().to_uppercase()
            )
            .as_str();

            save_translation_file(config, "trustlate", "go", &content)?;
        }
    }
    Ok(())
}

fn save_translation_file(
    config: &Config,
    filename_stem: &str,
    extension: &str,
    code: &str,
) -> Result<(), TrustlateError> {
    let filepath = config
        .target_dir
        .join(format!("{}.{}", filename_stem, extension));
    fs::create_dir_all(&filepath.as_path().parent().unwrap())
        .map_err(|_| TrustlateError::GenerateCannotCreateOutputFolders)?;
    let mut f = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&filepath)
        .map_err(|err| {
            println!("Path: {:?}", filepath);
            eprint!("Error when creating output file: {}", err);
            TrustlateError::GenerateCannotCreateOutputFile
        })?;
    write!(f, "{}", code).map_err(|_| TrustlateError::GenerateCannotWriteToOutputFile)?;
    Ok(())
}
