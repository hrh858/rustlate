use prettytable::Table;
use serde::Deserialize;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Deserialize)]
pub struct TranslationsTree {
    #[serde(flatten)]
    pub children: HashMap<String, Box<TranslationTreeNode>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TranslationTreeNode {
    NonLeaf(HashMap<String, Box<TranslationTreeNode>>),
    Leaf(String),
}

impl TranslationsTree {
    pub fn compare(&self, other: &TranslationsTree) -> Vec<TreeComparisonDifference> {
        let mut differences: Vec<TreeComparisonDifference> = Vec::new();
        let path = TreePath::new();

        for or_key in self.children.keys() {
            let path = path.walk(or_key);
            let other_node = other.children.get(or_key);
            if other_node.is_none() {
                differences.push(TreeComparisonDifference::MissingNode(path));
            } else {
                self.compare_rec(
                    self.children.get(or_key).unwrap(),
                    other.children.get(or_key).unwrap(),
                    &path,
                    &mut differences,
                );
            }
        }

        differences
    }

    fn compare_rec(
        &self,
        original: &TranslationTreeNode,
        other: &TranslationTreeNode,
        path: &TreePath,
        differences: &mut Vec<TreeComparisonDifference>,
    ) {
        match (original, other) {
            (
                TranslationTreeNode::NonLeaf(or_children),
                TranslationTreeNode::NonLeaf(ot_children),
            ) => {
                for or_key in or_children.keys() {
                    let path = path.walk(or_key);

                    if !ot_children.contains_key(or_key) {
                        differences.push(TreeComparisonDifference::MissingNode(path));
                    } else {
                        self.compare_rec(
                            or_children.get(or_key).unwrap(),
                            ot_children.get(or_key).unwrap(),
                            &path,
                            differences,
                        );
                    }
                }
            }
            (TranslationTreeNode::NonLeaf(_), TranslationTreeNode::Leaf(_)) => {
                differences.push(TreeComparisonDifference::DifferentNodeType(path.clone()))
            }
            (TranslationTreeNode::Leaf(_), TranslationTreeNode::NonLeaf(_)) => {
                differences.push(TreeComparisonDifference::DifferentNodeType(path.clone()))
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct TreePath(Vec<String>);

impl TreePath {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn walk(&self, next: &String) -> Self {
        let mut current = self.0.clone();
        current.push(next.clone());
        Self(current)
    }
}

impl Display for TreePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (step_idx, step) in self.0.iter().enumerate() {
            if step_idx == self.0.len() - 1 {
                write!(f, "{}", step);
            } else {
                write!(f, "{} -> ", step);
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum TreeComparisonDifference {
    MissingNode(TreePath),
    DifferentNodeType(TreePath),
}
