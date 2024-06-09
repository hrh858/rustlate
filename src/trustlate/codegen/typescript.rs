use crate::trustlate::{
    errors::TrustlateError,
    translations_tree::{TranslationTreeNode, TranslationsTree},
};

pub fn genererate_typescript(tree: &TranslationsTree) -> Result<String, TrustlateError> {
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

pub fn generate_typescript_index(langs: &[&str]) -> String {
    let imports = langs.iter().fold("".to_string(), |prev, curr| {
        format!(
            "{}import {{ trustlate as Translations{} }} from \"./{}\";\n",
            prev,
            curr.to_uppercase(),
            curr
        )
    });

    let locales = format!(
        "export const locales = [{}] as const;",
        langs
            .iter()
            .fold("".to_string(), |prev, curr| format!("{}'{}', ", prev, curr))
    );

    let translations = format!(
        "const translations = {{\n{}}} as const;",
        langs.iter().fold("".to_string(), |prev, curr| format!(
            "{}\t\"{}\": Translations{},\n",
            prev,
            curr,
            curr.to_uppercase()
        ))
    );

    format!("{}\n{}\nexport type Locale = typeof locales[number];\n\n{}\n\nexport function trustlate(lang: keyof typeof translations) {{ return translations[lang] }}", imports,locales,translations)
}

fn generate_typescript_rec(key: &String, curr_node: &Box<TranslationTreeNode>) -> String {
    match &**curr_node {
        TranslationTreeNode::Leaf(value) => format!("{}:{}", key, value),
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
