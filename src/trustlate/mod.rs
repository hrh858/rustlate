use std::collections::HashMap;

use crate::trustlate::translations_tree::TreeComparisonDifference;

use self::{config::Config, errors::TrustlateError, translations_tree::TranslationsTree};

pub mod config;
// pub mod parser;
pub mod codegen;
pub mod errors;
pub mod translations_tree;

pub fn generate_trees(
    config: &Config,
) -> Result<HashMap<String, TranslationsTree>, errors::TrustlateError> {
    let mut trees = HashMap::new();

    for lang in std::iter::once(&config.base_lang).chain(config.target_langs.iter()) {
        let f = std::fs::File::open(config.source_dir.join(format!("{}.json", lang)))
            .map_err(|_| TrustlateError::ParseTranslationFileCannotOpen)?;
        if let Some(_) = trees.insert(lang.to_string(), TranslationsTree::from_file(&f)?) {
            // If this is reached it means that one of the target languages is repeated.
            // Let's throw an error in such case.
            return Err(TrustlateError::ParseTranslationFileRepeatedLanguageKey);
        }
    }

    Ok(trees)
}

pub fn check_trees(
    config: &Config,
    trees: &HashMap<String, TranslationsTree>,
    show_differences: bool,
) -> bool {
    let mut ok = true;
    let base_lang_tree = trees.get(&config.base_lang).unwrap();

    for target_lang in &config.target_langs {
        use colored::*;

        let target_tree = trees.get(target_lang).unwrap();
        let differences = base_lang_tree.compare(target_tree);
        println!(
            "Translations for: {} -> {}",
            target_lang.to_uppercase().bold().underline().blue(),
            if differences.len() == 0 {
                "OK".bold().green()
            } else {
                "NOT OK".bold().red()
            },
        );

        if differences.len() != 0 && show_differences {
            use prettytable::*;

            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
            table.set_titles(row!["Path", "Error"]);
            for diff in &differences {
                match diff {
                    TreeComparisonDifference::DifferentNodeType(path) => {
                        table.add_row(Row::new(vec![
                            Cell::new(&format!("{}", path)),
                            Cell::new("Different value"),
                        ]));
                    }
                    TreeComparisonDifference::MissingNode(path) => {
                        table.add_row(Row::new(vec![
                            Cell::new(&format!("{}", path)),
                            Cell::new("Missing values"),
                        ]));
                    }
                    TreeComparisonDifference::DifferentParameters(path) => {
                        table.add_row(Row::new(vec![
                            Cell::new(&format!("{}", path)),
                            Cell::new("Incompatible parameters"),
                        ]));
                    }
                }
            }
            table.printstd();
            println!();
        ok = false;
        }
    }
    ok
}

pub fn harmonize_files(
    config: &Config,
    trees: &mut HashMap<String, TranslationsTree>,
    filling_string: &str,
) -> Result<(), TrustlateError> {
    let base_lang_tree = trees.get(&config.base_lang).unwrap();

    for target_lang in &config.target_langs {
        let mut target_lang_tree = trees.get(target_lang).unwrap().clone();
        let diffs = base_lang_tree.compare(&target_lang_tree);
        if diffs.len() != 0 {
            use colored::*;

            target_lang_tree.harmonize(base_lang_tree, &diffs, filling_string);
            let f = std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(config.source_dir.join(format!("{}.json", target_lang)))
                .map_err(|_| TrustlateError::FixTreeCannotOpenSourceFile)?;
            serde_json::to_writer_pretty(f, &target_lang_tree)
                .map_err(|_| TrustlateError::FixTreeCannotWriteToSourceFile)?;
            println!(
                "Fixed {} differences -> {}",
                diffs.len().to_string().bold().green(),
                target_lang.to_uppercase().bold().underline().blue(),
            );
        }
    }

    Ok(())
}

pub fn generate_code(
    config: &Config,
    trees: &HashMap<String, TranslationsTree>,
) -> Result<(), TrustlateError> {
    codegen::generate(config, trees)
}
