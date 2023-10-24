use super::{Element, Node};

pub fn parse (elems: Vec<Element>) -> Vec<Node> {
    elems
        .into_iter()
        .map(|element| Node { element, children: vec![] })
        .collect()
}
