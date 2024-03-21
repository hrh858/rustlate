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
    Generate, 
    // {
        // /// programming language in which to generate the code
        // #[clap(value_enum, default_value_t)]
        // language: trustlate::config::CodegenTarget,
    // },
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
                Commands::Generate => trustlate::generate_code(&config, &translations_trees)?,
                _ => unreachable!(),
            }
        }
    }

    Ok(())
}
