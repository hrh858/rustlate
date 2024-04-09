use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::Display};

use super::errors::{self, TrustlateError};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TranslationsTree {
    #[serde(flatten)]
    pub children: HashMap<String, Box<TranslationTreeNode>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum TranslationTreeNode {
    NonLeaf(HashMap<String, Box<TranslationTreeNode>>),
    Leaf(LeafType),
}

#[derive(Debug, Serialize, Clone)]
pub enum LeafType {
    LiteralLeaf(String),
    ParametrizedLeaf {
        parameters: Vec<String>,
        raw: String,
    },
}

impl<'de> Deserialize<'de> for LeafType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        match value {
            Value::String(value) => {
                let param_re = Regex::new(r"\{\{(.+?)\}\}").unwrap();
                if param_re.is_match(&value) {
                    let params = param_re
                        .captures_iter(&value)
                        .filter_map(|caps| caps.get(1))
                        .map(|mat| mat.as_str().to_string())
                        .collect();
                    Ok(LeafType::ParametrizedLeaf {
                        parameters: params,
                        raw: value,
                    })
                } else {
                    Ok(LeafType::LiteralLeaf(value))
                }
            }
            _ => Err(serde::de::Error::custom("Only string values are valid")),
        }
    }
}

impl Display for LeafType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeafType::LiteralLeaf(val) => write!(f, "\"{}\"", val),
            LeafType::ParametrizedLeaf { parameters, raw } => {
                let args = parameters
                    .iter()
                    .fold("".to_string(), |acc, el| format!("{}{}:string,", acc, el));
                // remove the final ","
                let args = args.strip_suffix(",").unwrap();
                let mut body = raw.clone();
                for parameter in parameters {
                    body = body.replace(&format!("{{{{{}}}}}", parameter), &format!("${{{}}}", parameter));
                }
                write!(f, "({})=>`{}`", args, body)
            }
        }
    }
}

impl TranslationTreeNode {
    fn blank_values(&mut self, blank_val: String) {
        match self {
            TranslationTreeNode::Leaf(val) => *val = LeafType::LiteralLeaf(blank_val),
            TranslationTreeNode::NonLeaf(children) => {
                for (_, v) in children {
                    v.blank_values(blank_val.clone());
                }
            }
        }
    }
}

impl TranslationsTree {
    pub fn from_file(f: &std::fs::File) -> Result<TranslationsTree, errors::TrustlateError> {
        let tree = serde_json::from_reader(f)
            .map_err(|_| TrustlateError::ParseTraslationFileInvalidJson)?;
        Ok(tree)
    }

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

    pub fn harmonize(
        &mut self,
        reference: &TranslationsTree,
        differences: &Vec<TreeComparisonDifference>,
        filling_str: &str,
    ) {
        for diff in differences {
            match diff {
                TreeComparisonDifference::DifferentNodeType(path) => {
                    let ref_node = reference.get_node_at(path);
                    match &**ref_node {
                        TranslationTreeNode::Leaf(_) => {
                            let mut new_node = *ref_node.clone();
                            new_node.blank_values(filling_str.to_string());
                            self.replace_node_at(new_node, path);
                        }
                        TranslationTreeNode::NonLeaf { .. } => {
                            let mut new_node = *ref_node.clone();
                            new_node.blank_values(filling_str.to_string());
                            self.replace_node_at(new_node, path);
                        }
                    }
                }
                TreeComparisonDifference::MissingNode(path) => {
                    let ref_node = reference.get_node_at(path);
                    let mut new_node = ref_node.clone();
                    new_node.blank_values(filling_str.to_string());
                    self.insert_node_at(new_node, path);
                }
            }
        }
    }

    fn get_node_at(&self, path: &TreePath) -> &Box<TranslationTreeNode> {
        let mut n = self
            .children
            .get(path.0.first().expect("a non empty path"))
            .expect("an existing node");
        for link in path.0.iter().skip(1) {
            match &**n {
                TranslationTreeNode::NonLeaf(children) => {
                    n = children.get(link).expect("the children to exist");
                }
                TranslationTreeNode::Leaf(_) => panic!(""),
            }
        }
        n
    }

    fn get_node_at_mut(&mut self, path: &TreePath) -> &mut TranslationTreeNode {
        let mut n = self
            .children
            .get_mut(path.0.first().expect("a non empty path"))
            .expect("an existing node");
        for link in path.0.iter().skip(1) {
            n = match **n {
                TranslationTreeNode::NonLeaf(ref mut children) => {
                    children.get_mut(link).expect("the children to exist")
                }
                TranslationTreeNode::Leaf(_) => panic!(""),
            }
        }
        n
    }

    fn insert_node_at(&mut self, node: Box<TranslationTreeNode>, path: &TreePath) {
        let mut p = path.clone();
        p.0 = path.0[..path.0.len() - 1].to_vec();
        let n = if p.0.len() == 0 {
            self.children.insert(path.0.first().unwrap().clone(), node);
            return;
        } else {
            self.get_node_at_mut(&p)
        };

        match *n {
            TranslationTreeNode::NonLeaf(ref mut children) => {
                children.insert(path.0.last().unwrap().clone(), node);
            }
            TranslationTreeNode::Leaf(_) => {}
        }
    }

    fn replace_node_at(&mut self, node: TranslationTreeNode, path: &TreePath) {
        let n = self.get_node_at_mut(path);
        *n = node;
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
