use std::{fs::File, io::Write};

use super::{config::Config, translations_tree::{TranslationTreeNode, TranslationsTree}};

pub struct CodeGenerator<'a> {
    conf: &'a Config,
}

impl<'a> CodeGenerator<'a> {
    pub fn from_config(conf: &'a Config) -> Self { 

        Self {
            conf,
        }
    }

    pub fn generate(&self, tree: &TranslationsTree) -> Result<(), Box<dyn std::error::Error>> {
        let (code, extension) = match self.conf.codegen.as_str() {
            "typescript" => (genererate_typescript(tree)?, "ts"),
            _ => panic!("The programming language {} is not supported", self.conf.codegen.as_str())
        };

        let mut f = File::create(format!("{}/trustlate.{}", self.conf.target_dir, extension))?;
        write!(f, "{}", code)?;

        Ok(())
    }
}

fn genererate_typescript(tree: &TranslationsTree) -> Result<String, Box<dyn std::error::Error>> {
    let suffix = "export const trustlate=";
    let prefix = "as const;";
    let top_level_nodes: Vec<String> = tree.children.iter().map(|(k, v)| generate_typescript_rec(k, v)).collect();
    let mut aux = "".to_string();
    for (i, child_code) in top_level_nodes.iter().enumerate() {
        aux = format!("{}{}", aux, child_code);
        if i != top_level_nodes.len() - 1 {
            aux = format!("{},", aux);
        }
    }
    aux = format!("{{{}}}", aux);
    Ok(format!("{}{}{}", suffix, aux, prefix))
}

fn generate_typescript_rec(key: &String, curr_node: &Box<TranslationTreeNode>) -> String {
    match &**curr_node {
        TranslationTreeNode::Leaf(value) => format!("{}:\"{}\"", key, value),
        TranslationTreeNode::NonLeaf(children) => {
            let children_code: Vec<String> = children.iter().map(|(k, v)| generate_typescript_rec(k, v)).collect();
            let mut aux = "".to_string();
            for (i, child_code) in children_code.iter().enumerate() {
                aux = format!("{}{}", aux, child_code);
                if i != children_code.len() - 1 {
                    aux = format!("{},", aux);
                }
            }
            format!("{}:{{{}}}", key, aux)
        } ,
    }
}
