use crate::tags::{pattern::Pattern, that::That};
use crate::utils::functions::{input_that_topic, is_match};
use minidom::Element;
/// This struct is going to map exactly the content of the aiml _category_ node.
#[derive(Debug, Clone)]
pub struct Node {
    path: String,
    pub pattern: Pattern,
    pub that: Option<That>,
    pub topic: Option<String>,
    pub template: Option<Element>,
}

impl Node {
    pub fn new(
        pattern: String,
        that: Option<String>,
        topic: Option<String>,
        template: Option<Element>,
    ) -> Self {
        let t = match &that {
            Some(s) => Some(That::new(&s)),
            None => None,
        };
        Node {
            path: input_that_topic(&pattern, that.as_deref(), topic.as_deref()),
            pattern: Pattern::new(&pattern),
            that: t,
            topic,
            template,
        }
    }

    pub fn is_match(&self, input: &str) -> bool {
        trace!("comparing {:?} to {:?}", self.path, input);
        is_match(input, &self.path)
    }
}
