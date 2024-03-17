mod trustlate;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use trustlate::config::Config;

#[derive(Parser)]
#[command(about, long_about = None)]
#[command(name = "trustlate")]
#[command(about = "A helpful app for safely managing ans generating translation clients for your app", long_about = None)]
struct Cli {
    /// path to the .json file containing the configuration that trustlate should use
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// initializes default directories and configuration for trustlate to work (and for easy
    /// customizaition)
    Init,
    /// checks if the target languages translation files conform to the base language translation file
    /// structure
    Check {
        /// set this to show a table with the differences in case a target language file has
        /// differences with respect to the base language translation file
        #[clap(long, short, action)]
        show_diffs: bool,
    },
    /// makes the target languages translation files conform to the base language translation file
    /// structure while filling the missing translations
    Fix {
        #[clap(long, short, action)]
        filling: Option<String>
    },
    /// generates the translation client code for the specified language
    Generate {
        language: String, // language: Option<trustlate::config::CodegenTarget>
    },
}

fn main() -> Result<(), trustlate::errors::TrustlateError> {
    let cli = Cli::parse();
    let config = if let Some(config_path) = cli.config {
        Config::from_file(&config_path)?
    } else {
        Config::default()
    };

    match &cli.command {
        Commands::Init => config.initialize()?,
        _ => {
            let mut translations_trees = trustlate::generate_trees(&config)?;
            match &cli.command {
                Commands::Check { show_diffs } => {
                    trustlate::check_trees(&config, &translations_trees, *show_diffs)
                }
                Commands::Fix { filling } => trustlate::harmonize_files(
                    &config,
                    &mut translations_trees,
                    filling.as_deref().unwrap_or("[FILLING]"),
                )?,
                Commands::Generate { language } => todo!(),
                _ => unreachable!(),
            }
        }
    }

    Ok(())
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let args: Vec<String> = env::args().collect();
//     let config_file = File::open(&args[1])?;
//
//     let conf = Config::form_json_file(config_file)?;
//     let parser = Parser::from_config(&conf);
//     let code_generator = CodeGenerator::from_config(&conf);
//     let translations_trees = parser.parse_translation_files()?;
//
//     let base_lang_tree = translations_trees
//             .get(parser.base_lang())
//             .unwrap();
//
//     // ⬇︎ Print differences ⬇︎
//     for target_lang in parser.target_languages() {
//         let target_lang_tree = translations_trees.get(target_lang).unwrap();
//         let differences = base_lang_tree.compare(target_lang_tree);
//
//         // let mut table = Table::new();
//         // table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
//         // table.set_titles(row!["Path", "Error"]);
//         // for diff in &differences {
//         //     match diff {
//         //         TreeComparisonDifference::DifferentNodeType(path) => {
//         //             table.add_row(Row::new(vec![
//         //                 Cell::new(&format!("{}", path)),
//         //                 Cell::new("Different value"),
//         //             ]));
//         //         }
//         //         TreeComparisonDifference::MissingNode(path) => {
//         //             table.add_row(Row::new(vec![
//         //                 Cell::new(&format!("{}", path)),
//         //                 Cell::new("Missing values"),
//         //             ]));
//         //         }
//         //     }
//         // }
//         // table.printstd();
//
//         // ⬇︎ Harmonize target_lang tree ⬇︎
//         let mut harmonized_target_lang_tree = target_lang_tree.clone();
//         harmonized_target_lang_tree.harmonize(base_lang_tree, &differences);
//         code_generator.generate(&harmonized_target_lang_tree)?;
//
//         println!("{}", serde_json::to_string_pretty(&harmonized_target_lang_tree).unwrap());
//     }
//
//
//
//     // ⬇︎ Code generation ⬇︎
//     // code_generator.generate(translations_trees.get("es").unwrap())?;
//     Ok(())
// }
