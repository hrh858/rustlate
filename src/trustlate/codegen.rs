use std::{collections::HashMap, fs::File, io::Write};

use super::{
    config::{self, Config},
    errors::TrustlateError,
    translations_tree::{TranslationTreeNode, TranslationsTree},
};

pub fn generate(
    config: &Config,
    tree: &HashMap<String, TranslationsTree>,
) -> Result<(), TrustlateError> {
    for (key, value) in tree {
        let (code, extension) = match config.codegen {
            config::CodegenTarget::Typescript => (
                genererate_typescript(value)
                    .map_err(|_| TrustlateError::GenerateCannotGenerateCode)?,
                "ts",
            ),
        };

        let mut f = File::create(config.target_dir.join(format!("{}.{}", key, extension)))
            .map_err(|_| TrustlateError::GenerateCannotCreateOutputFile)?;
        write!(f, "{}", code).map_err(|_| TrustlateError::GenerateCannotWriteToOutputFile)?;
    }

    Ok(())
}

fn genererate_typescript(tree: &TranslationsTree) -> Result<String, TrustlateError> {
    let suffix = "export const trustlate=";
    let prefix = "as const;";
    let top_level_nodes: Vec<String> = tree
        .children
        .iter()
        .map(|(k, v)| generate_typescript_rec(k, v))
        .collect();
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
            let children_code: Vec<String> = children
                .iter()
                .map(|(k, v)| generate_typescript_rec(k, v))
                .collect();
            let mut aux = "".to_string();
            for (i, child_code) in children_code.iter().enumerate() {
                aux = format!("{}{}", aux, child_code);
                if i != children_code.len() - 1 {
                    aux = format!("{},", aux);
                }
            }
            format!("{}:{{{}}}", key, aux)
        }
    }
}
