use std::collections::HashMap;

#[derive(Debug)]
pub struct TranslationsTree {
    children: HashMap<String, Box<TranslationTreeNode>>
}

impl TranslationsTree {
}

#[derive(Debug)]
pub enum TranslationTreeNode {
    NonLeaf{ name: String, children: HashMap<String, Box<TranslationTreeNode>>},
    Leaf(String)
}