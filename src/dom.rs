use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Node {
  children: Vec<Node>,
  node_type: NodeType,
}

#[derive(Debug, PartialEq)]
enum NodeType {
  Text(String),
  Element(ElementData),
}

#[derive(Debug, PartialEq)]
struct ElementData {
  tag_name: String,
  attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

fn text(data: String) -> Node {
  Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
  Node {
    children: children,
    node_type: NodeType::Element(ElementData {
      tag_name: name,
      attributes: attrs,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_text() {
    let node = text(String::from("TeatData"));
    assert_eq!(node, Node {
      children: Vec::new(),
      node_type: NodeType::Text(String::from("TeatData")),
    });
  }

  #[test]
  fn test_elem() {
    let elem_node = elem(
      String::from("TeatData"),
      HashMap::new(),
      Vec::new());
    assert_eq!(elem_node, Node {
      children: Vec::new(),
      node_type: NodeType::Element(ElementData {
        tag_name: String::from("TeatData"),
        attributes: HashMap::new(),
      })
    });
  }
}
