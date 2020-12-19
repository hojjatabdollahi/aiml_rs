use std::fmt::write;

#[derive(Debug)]
pub struct AIML {
    nodes: Vec<String>,
    current_node: usize,
}

/// This struct is going to map exactly the content of the aiml _category_ node.
#[derive(Debug)]
pub struct Node {
    pub pattern: String,
    pub template: String,
}

impl AIML {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            current_node: 0,
        }
    }

    pub fn append(&mut self, pattern: String) {
        self.nodes.push(pattern);
    }
}

impl Iterator for AIML {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.current_node += 1;
        Some(self.nodes.get(self.current_node - 1)?.to_owned())
    }
}
