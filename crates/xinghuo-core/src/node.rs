use rctree::Node;
use std::{cell::RefCell, collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub enum NodeData {
    Elem {
        tag: String,
        attrs: RefCell<HashMap<String, String>>,
    },
    Text(String),
}

impl NodeData {
    pub fn new(tag: &str) -> Self {
        Self::Elem {
            tag: tag.to_owned(),
            attrs: RefCell::new(HashMap::new()),
        }
    }
}

pub struct CachedNode {
    node: Node<NodeData>,
}

impl CachedNode {
    pub fn new(tag: &str) -> Self {
        Self {
            node: Node::new(NodeData::new(tag)),
        }
    }

    pub fn raw_node(&self) -> &Node<NodeData> {
        &self.node
    }

    pub fn set_attribute(&mut self, name: &str, value: &str) {
        let node = self.node.borrow_mut();
        let mut attrs = match &*node {
            NodeData::Elem { ref attrs, .. } => attrs.borrow_mut(),
            text => panic!("expected VirtData::Elem, found {:?}", text),
        };

        if let Some(existing) = attrs.iter_mut().find(|(n, _)| *n == name) {
            *existing.1 = value.to_string();
        } else {
            attrs.insert(name.to_string(), value.to_string());
        }
    }

    fn remove_attribute(&mut self, name: &str) {
        let node = self.node.borrow_mut();
        let mut attrs = match &*node {
            NodeData::Elem { attrs, .. } => attrs.borrow_mut(),
            data => panic!("expected VirtData::Elem, found {:?}", data),
        };
        attrs.retain(|n, _| n != name);
    }

    fn get_attribute(&self, name: &str) -> Option<String> {
        match &*self.node.borrow() {
            NodeData::Text(_) => None,
            NodeData::Elem { tag: _, attrs } => {
                attrs.borrow_mut().iter().find_map(|(attr, value)| {
                    if attr.as_str() == name {
                        Some(value.clone())
                    } else {
                        None
                    }
                })
            }
        }
    }

    pub fn append_child(&mut self, new_child: Node<NodeData>) {
        self.node.append(new_child);
    }

    fn first_child(&self) -> Option<Node<NodeData>> {
        self.node.first_child()
    }
}
