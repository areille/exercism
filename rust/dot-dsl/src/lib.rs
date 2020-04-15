pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;
    pub struct Graph {
        pub attrs: HashMap<String, String>,
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::<Node>::new(),
                edges: Vec::<Edge>::new(),
                attrs: HashMap::<String, String>::new(),
            }
        }
        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Graph {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Graph {
            self.edges = edges.to_vec();
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Graph {
            let mut _attrs = HashMap::<String, String>::new();
            for attr in attrs.iter() {
                _attrs.insert(attr.0.to_string(), attr.1.to_string());
            }
            self.attrs = _attrs;
            self
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct Edge {
                start: String,
                end: String,
                attrs: HashMap<String, String>,
            }
            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Edge {
                        start: a.to_owned(),
                        end: b.to_owned(),
                        attrs: HashMap::<String, String>::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Edge {
                    let mut _attrs = HashMap::<String, String>::new();
                    for attr in attrs.iter() {
                        _attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self.attrs = _attrs;
                    self
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;
            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct Node {
                name: String,
                attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(string: &str) -> Self {
                    Node {
                        name: string.to_owned(),
                        attrs: HashMap::<String, String>::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Node {
                    let mut _attrs = HashMap::<String, String>::new();
                    for attr in attrs.iter() {
                        _attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self.attrs = _attrs;
                    self
                }

                pub fn node(self) -> String {
                    self.name
                }
            }
        }
    }
}
