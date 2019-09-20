pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node { 
                        name: name.to_owned(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (a, b) in attrs {
                        self.attrs.insert(a.to_string(), b.to_string());
                    }
                    self
                }
                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    Some(self.attrs.get(name).unwrap())
                }
            }
        }
        pub mod edge {
            use std::collections::HashMap;
            
            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge { 
                        from: from.to_owned(),
                        to: to.to_owned(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (a, b) in attrs {
                        self.attrs.insert(a.to_string(), b.to_string());
                    }
                    self
                }
            }
        }
    }
    use graph_items::edge::Edge;
    use graph_items::node::Node;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes = nodes.clone();
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges = edges.clone();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (a, b) in attrs {
                self.attrs.insert(a.to_string(), b.to_string());
            }
            self
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|&x| x.name == name)
        }
    }
}
