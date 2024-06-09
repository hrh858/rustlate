pub mod go;
pub mod typescript;

use std::{collections::HashMap, fs::File, io::Write};

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
            for (lang, translations) in tree {
                let is_main_lang = config.base_lang == *lang;
                let generations = generate_golang(translations.clone(), lang)
                    .map_err(|_| TrustlateError::GenerateCannotGenerateCode)?;

                if is_main_lang {
                    println!("Lang is {lang}");
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
                // TODO: Save to a file 'trustlate.go'

                //         .map_err(|_| TrustlateError::GenerateCannotGenerateCode)
                // let (generation, extension) = (
                //     &generate_golang(translations.clone())
                //         .map_err(|_| TrustlateError::GenerateCannotGenerateCode)?,
                //     "go",
                // );
                // save_translation_file(config, lang, extension, &code)?;
            }
            println!("{}", content);
            todo!();
        }
    }
    Ok(())
}

fn save_translation_file(
    config: &Config,
    lang: &str,
    extension: &str,
    code: &str,
) -> Result<(), TrustlateError> {
    let mut f =
        File::create(config.target_dir.join(format!("{}.{}", lang, extension))).map_err(|err| {
            println!(
                "Path: {:?}",
                config.target_dir.join(format!("{}.{}", lang, extension))
            );
            eprint!("Error when creating output file: {}", err);
            TrustlateError::GenerateCannotCreateOutputFile
        })?;
    write!(f, "{}", code).map_err(|_| TrustlateError::GenerateCannotWriteToOutputFile)?;
    Ok(())
}
