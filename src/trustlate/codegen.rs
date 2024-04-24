use std::{collections::HashMap, fs::File, io::Write};

use super::{
    config::{self, Config},
    errors::TrustlateError,
    translations_tree::{TranslationTreeNode, TranslationsTree},
};

// pub fn generate(
//     config: &Config,
//     tree: &HashMap<String, TranslationsTree>,
// ) -> Result<(), TrustlateError> {
//     for (key, value) in tree {
//         let (code, extension) = match config.codegen {
//             config::CodegenTarget::Typescript => (
//                 genererate_typescript(value)
//                     .map_err(|_| TrustlateError::GenerateCannotGenerateCode)?,
//                 "ts",
//             ),
//         };
//
//         let mut f = File::create(config.target_dir.join(format!("{}.{}", key, extension)))
//             .map_err(|err| {
//                 println!(
//                     "Path: {:?}",
//                     config.target_dir.join(format!("{}.{}", key, extension))
//                 );
//                 eprint!("Error when creating output file: {}", err);
//                 TrustlateError::GenerateCannotCreateOutputFile
//             })?;
//         write!(f, "{}", code).map_err(|_| TrustlateError::GenerateCannotWriteToOutputFile)?;
//     }
//
//     Ok(())
// }

pub fn generate(
    config: &Config,
    tree: &HashMap<String, TranslationsTree>,
) -> Result<(), TrustlateError> {
    match config.codegen {
        config::CodegenTarget::Typescript => {
            for (key, value) in tree {
                let (code, extension) = (
                    genererate_typescript(value)
                        .map_err(|_| TrustlateError::GenerateCannotGenerateCode)?,
                    "ts",
                );
                let mut f = File::create(config.target_dir.join(format!("{}.{}", key, extension)))
                    .map_err(|err| {
                        println!(
                            "Path: {:?}",
                            config.target_dir.join(format!("{}.{}", key, extension))
                        );
                        eprint!("Error when creating output file: {}", err);
                        TrustlateError::GenerateCannotCreateOutputFile
                    })?;
                write!(f, "{}", code)
                    .map_err(|_| TrustlateError::GenerateCannotWriteToOutputFile)?;
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

fn generate_typescript_index(langs: &[&str]) -> String {
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

    //     format!(
    //         r#"import {{ trustlate as TranslationsKr }} from "./kr";
    // import {{ trustlate as TranslationsEn }} from "./en";
    // import {{ trustlate as TranslationsEs }} from "./es";
    //
    // export const locales = ['es', 'en', 'cat'] as const;
    // export type Locale = typeof locales[number];
    //
    // const translations = {{
    //   "kr": TranslationsKr,
    //   "es": TranslationsEs,
    //   "en": TranslationsEn
    // }} as const;
    //
    // export function trustlate(lang: keyof typeof translations) {{
    //   return translations[lang]
    // }}"#
    //     )
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
