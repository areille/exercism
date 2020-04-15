macro_rules! impl_attrs {
    () => {
        pub fn get_attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key).map(|s| s.as_str())
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect();
            self
        }
    };
}

pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;
    #[derive(Default, PartialEq, Eq)]
    pub struct Graph {
        pub attrs: HashMap<String, String>,
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Graph {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Graph {
            self.edges = edges.to_vec();
            self
        }
        pub fn get_node(&self, s: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name() == s)
        }
        impl_attrs!();
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            #[derive(Debug, Clone, Eq, PartialEq, Default)]
            pub struct Edge {
                start: String,
                end: String,
                attrs: HashMap<String, String>,
            }
            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Edge {
                        start: a.to_string(),
                        end: b.to_string(),
                        ..Self::default()
                    }
                }

                impl_attrs!();
            }
        }
        pub mod node {
            use std::collections::HashMap;
            #[derive(Debug, Clone, Eq, PartialEq, Default)]
            pub struct Node {
                name: String,
                attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(string: &str) -> Self {
                    Node {
                        name: string.to_string(),
                        ..Self::default()
                    }
                }

                pub fn name(&self) -> &str {
                    &self.name
                }

                impl_attrs!();
            }
        }
    }
}
