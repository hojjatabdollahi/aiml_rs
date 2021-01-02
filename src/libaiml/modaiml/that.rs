use std::cmp::{Ord, Ordering};
#[derive(Debug)]
pub struct That {
    that: String,
}

impl That {
    pub fn new(str: &str) -> Self {
        Self {
            that: str.to_string(),
        }
    }
}

impl Ord for That {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.that == other.that {
            Ordering::Equal
        } else if self.that == "*" {
            if other.that == "*" {
                // self == *, other == *
                Ordering::Equal
            } else {
                // self == *, other == word
                Ordering::Less
            }
        } else {
            // self == word
            if other.that == "*" {
                // self == word, other == *
                Ordering::Greater
            } else {
                // self == word, other == word
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for That {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for That {
    fn eq(&self, other: &Self) -> bool {
        (self.that == "*" && other.that == "*") || (self.that != "*" && other.that != "*")
    }
}

impl Eq for That {}
