use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Edge {
    x: String,
    y: String,
    attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(x: &str, y: &str) -> Self {
        Self {
            x: x.to_string(),
            y: y.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        for (k, v) in attrs.into_iter() {
            self.attrs.insert(k.to_string(), v.to_string());
        }

        self
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        for (k, v) in attrs.into_iter() {
            self.attrs.insert(k.to_string(), v.to_string());
        }

        self
    }

    pub fn get_attr(&self, name: &str) -> Option<&str> {
        self.attrs.get(name).map(|s| s.as_str())
    }
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
        self.nodes = nodes.to_vec();
        self
    }

    pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
        self.edges = edges.to_vec();
        self
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        for (k, v) in attrs.into_iter() {
            self.attrs.insert(k.to_string(), v.to_string());
        }

        self
    }

    pub fn get_node(self, name: &str) -> Option<Node> {
        self.nodes.into_iter().find(|n| n.name == name)
    }
}

pub mod graph {
    pub use super::Graph;
    pub mod graph_items {
        pub mod edge {
            pub use super::super::super::Edge;
        }
        pub mod node {
            pub use super::super::super::Node;
        }
    }
}
