use prettytable::{format, row, Cell, Row, Table};
use std::{env, error::Error, fs::File};
use trustlate::{codegen::CodeGenerator, config::Config, parser::Parser, translations_tree::TreeComparisonDifference};
mod trustlate;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config_file = File::open(&args[1])?;
    let conf = Config::form_json_file(config_file)?;
    let parser = Parser::from_config(&conf);
    let code_generator = CodeGenerator::from_config(&conf);
    let translations_trees = parser.parse_translation_files()?; 

    for target_lang in parser.target_languages() {
        let target_lang_tree = translations_trees.get(target_lang).unwrap();
        let differences = translations_trees
            .get(parser.base_lang())
            .unwrap()
            .compare(target_lang_tree);

        println!("{}.json", target_lang);
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row!["Path", "Error"]);
        for diff in differences {
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
            }
        }
        table.printstd()
    }

    code_generator.generate(translations_trees.get("es").unwrap())?;

    Ok(())
}
