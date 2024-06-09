use regex::Regex;

use crate::trustlate::translations_tree::TranslationTreeNode;
use crate::trustlate::{errors::TrustlateError, translations_tree::LeafType, TranslationsTree};

#[derive(Debug)]
pub struct GolangGenerationFunc {
    lang: String,
    params: Option<Vec<String>>,
    path: Vec<String>,
    translation: String,
}

impl GolangGenerationFunc {
    pub fn function_form(&self) -> String {
        let receiver_name = make_receiver_name(&self.lang);
        let fn_name = make_function_name(&self.path);

        match &self.params {
            Some(params) => {
                let fn_params = make_function_params(params);
                let sprintf_params = make_sprintf_params(params);
                let translation = make_sprintf_translation(&self.translation);
                format!("func (trl *{receiver_name}) {fn_name}({fn_params}) string {{\n   return fmt.Sprintf(\"{translation}\", {sprintf_params})\n}}")
            }
            None => {
                let translation = &self.translation;
                format!(
                    "func (trl *{receiver_name}) {fn_name}() string {{\n   return \"{translation}\"\n}}"
                )
            }
        }
    }

    pub fn interface_entry_form(&self) -> String {
        let fn_name = make_function_name(&self.path);

        match &self.params {
            Some(params) => {
                let fn_params = make_function_params(params);
                format!("{fn_name}({fn_params}) string")
            }
            None => {
                format!("{fn_name}() string")
            }
        }
    }
}

pub fn generate_golang(
    tree: TranslationsTree,
    lang: &String,
) -> Result<Vec<GolangGenerationFunc>, TrustlateError> {
    let mut gen: Vec<GolangGenerationFunc> = Vec::new();
    let node = TranslationTreeNode::NonLeaf(tree.children);
    collect_fns_rec(&mut gen, node, vec![], lang);
    Ok(gen)
}

fn collect_fns_rec(
    acc: &mut Vec<GolangGenerationFunc>,
    curr: TranslationTreeNode,
    path: Vec<String>,
    lang: &str,
) {
    match curr {
        TranslationTreeNode::Leaf(curr) => match curr {
            LeafType::LiteralLeaf(translation) => {
                acc.push(GolangGenerationFunc {
                    params: None,
                    path,
                    translation,
                    lang: lang.to_string(),
                });
            }
            LeafType::ParametrizedLeaf { parameters, raw } => {
                acc.push(GolangGenerationFunc {
                    params: Some(parameters),
                    path,
                    translation: raw,
                    lang: lang.to_string(),
                });
            }
        },
        TranslationTreeNode::NonLeaf(children) => {
            for (child_name, child_node) in children.into_iter() {
                let mut path = path.to_vec();
                path.push(child_name);
                collect_fns_rec(acc, *child_node, path, lang)
            }
        }
    }
}

fn make_function_name(path: &Vec<String>) -> String {
    let func_name: String = capitalize_first_letter(path.first().unwrap())
        + &path
            .iter()
            .skip(1)
            .map(|p| format!("_{}", capitalize_first_letter(p)))
            .collect::<String>();
    func_name
}

fn make_receiver_name(lang: &String) -> String {
    format!("Trustlate{}", lang.to_uppercase())
}

fn make_sprintf_params(params: &Vec<String>) -> String {
    let last_param = params.last().unwrap();
    let mut aux: String = params[..params.len() - 1]
        .iter()
        .map(|param| format!("{param}, "))
        .collect();
    aux += format!("{last_param}").as_str();
    aux
}

fn make_sprintf_translation(translation: &str) -> String {
    let re = Regex::new(r"\{\{.*?\}\}").unwrap();
    let result = re.replace_all(translation, "%s");
    result.into()
}

fn make_function_params(params: &Vec<String>) -> String {
    let last_param = params.last().unwrap();
    let mut aux: String = params[..params.len() - 1]
        .iter()
        .map(|param| format!("{param} string, "))
        .collect();
    aux += format!("{last_param} string").as_str();
    aux
}

fn is_leaf(node: &TranslationTreeNode) -> bool {
    match node {
        TranslationTreeNode::Leaf(_) => true,
        _ => false,
    }
}

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn build_type_name(path: &[&str]) -> String {
    let mut type_str = path.iter().skip(1).fold(path[0].to_string(), |prev, curr| {
        format!("{}_{}", prev, curr)
    });
    type_str.push_str("_T");
    type_str
}
