use std::cmp::{Ord, Ordering};
#[derive(Debug, Clone)]
pub struct Pattern {
    value: String,
}

impl Pattern {
    pub fn new(str: &str) -> Self {
        Self {
            value: str.to_string(),
        }
    }
}

impl Ord for Pattern {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value == other.value {
            Ordering::Equal
        } else if self.value == "*" {
            if other.value == "*" {
                // self == *, other == *
                Ordering::Equal
            } else {
                // self == *, other == word
                Ordering::Less
            }
        } else {
            // self == word
            if other.value == "*" {
                // self == word, other == *
                Ordering::Greater
            } else {
                // self == word, other == word
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Pattern {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        (self.value == "*" && other.value == "*") || (self.value != "*" && other.value != "*")
    }
}

impl Eq for Pattern {}

#[cfg(test)]
mod tests {
    use crate::tags::pattern::Pattern;
    #[test]
    fn test_that() {
        assert_eq!(Pattern::new("*"), Pattern::new("*"));
        assert!(Pattern::new("Hi") > Pattern::new("*"));
        assert!(Pattern::new("*") < Pattern::new("Hi"));
    }
}
