use std::collections::HashMap;
use std::fmt;
use std::fmt::{Formatter, Error};

#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,

    node_type: NodeType,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(&*format!("{}:{}", "child", self.children.len()))?;
        f.write_str(",NodeType:")?;
        Ok(())
    }
}

#[derive(Debug)]
enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}