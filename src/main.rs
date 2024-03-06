use std::{env, error::Error, fs::File};
mod trustlate;
use trustlate::{config::Config, parser::Parser};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config_file = File::open(&args[1])?;
    let conf = Config::form_json_file(config_file)?;
    let parser = Parser::from_config(conf);
    let translations_trees = parser.parse()?;

    Ok(())
}
