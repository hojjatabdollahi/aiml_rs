use crate::modaiml::utils::{input_that_topic, is_match};
use minidom::Element;
/// This struct is going to map exactly the content of the aiml _category_ node.
#[derive(Debug, Clone)]
pub struct Node {
    path: String,
    pub pattern: String,
    pub that: Option<String>,
    pub topic: Option<String>,
    pub template: Option<Element>,
    pub is_topic: bool,
}

impl Node {
    pub fn new(
        pattern: String,
        that: Option<String>,
        topic: Option<String>,
        template: Option<Element>,
    ) -> Self {
        Node {
            path: input_that_topic(&pattern, that.as_deref(), topic.as_deref()),
            pattern,
            that,
            topic,
            template,
            is_topic: false,
        }
    }

    pub fn is_match(&self, input: &str) -> bool {
        trace!("comparing {:?} to {:?}", self.path, input);
        is_match(input, &self.path)
    }
}
