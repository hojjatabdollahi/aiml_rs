use minidom::Element;


#[derive(Debug)]
pub struct AIML {
    pub nodes: Vec<Node>,
    current_node: usize,
}

/// This struct is going to map exactly the content of the aiml _category_ node.
#[derive(Debug, Clone)]
pub struct Node {
    pub pattern: String,
    pub template: Option<Element>,
    pub topic: Option<String>,
}

impl AIML {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            current_node: 0,
        }
    }
}

impl Iterator for AIML {
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        self.current_node += 1;
        self.nodes.get(self.current_node - 1).cloned()
    }
}
