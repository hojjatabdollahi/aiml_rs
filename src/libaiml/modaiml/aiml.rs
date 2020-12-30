use indextree;
use minidom::Element;
use std::collections::HashMap;

#[derive(Debug)]
pub struct AIML {
    //pub nodes: Vec<Node>,
    pub arena: indextree::Arena<Node>,
    current_node: usize,
}

/// This struct is going to map exactly the content of the aiml _category_ node.
#[derive(Debug, Clone)]
pub struct Node {
    path: String,
    pub pattern: String,
    pub that: Option<String>,
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
        Node {
            path: input_that_topic(&pattern, that.as_deref(), topic.as_deref()),
            pattern,
            that,
            topic,
            template,
        }
    }

    pub fn is_match(&self, input: &str) -> bool {
        trace!("comparing {:?} to {:?}", self.path, input);
        self.path == input
    }
}

impl AIML {
    pub fn new() -> Self {
        Self {
            arena: indextree::Arena::new(),
            current_node: 0,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &indextree::Node<Node>> {
        self.arena.iter()
    }
}

/// Append the <input>, <that> and <topic> for the matching
/// # Examples
///
/// ```
/// # use libaiml::modaiml::aiml;
/// assert_eq!(
///     aiml::input_that_topic("a", Some("b"), Some("c")),
///     "a <that> b <topic> c"
/// );
/// assert_eq!(aiml::input_that_topic("a", None, None),"a <that> * <topic> *");
/// ```
pub fn input_that_topic(input: &str, that: Option<&str>, topic: Option<&str>) -> String {
    let mut result = String::new();
    result.push_str(input.trim());
    result.push_str(" <that> ");
    match that {
        Some(txt) => result.push_str(txt.trim()),
        None => result.push_str("*"),
    }
    result.push_str(" <topic> ");
    match topic {
        Some(txt) => result.push_str(txt.trim()),
        None => result.push_str("*"),
    }
    result.to_lowercase()
}

pub struct Userdata {
    vars: HashMap<String, String>,
}

impl Userdata {
    pub fn new() -> Self {
        Userdata {
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.vars.get(key).cloned()
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.vars.insert(key.to_string(), value.to_string());
    }

    pub fn remove(&mut self, key: &str) {
        self.vars.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_userdata() {
        let mut userdata = Userdata::new();
        userdata.set("test", "testval");
        assert_eq!(userdata.get("test"), Some("testval".to_string()));
        assert!(userdata.get("test2").is_none());
    }
}
