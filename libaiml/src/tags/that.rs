use std::cmp::{Ord, Ordering};
#[derive(Debug, Clone)]
pub struct That {
    pub value: String,
}

impl That {
    pub fn new(str: &str) -> Self {
        Self {
            value: str.to_string(),
        }
    }
}

impl Ord for That {
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

impl PartialOrd for That {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for That {
    fn eq(&self, other: &Self) -> bool {
        (self.value == "*" && other.value == "*") || (self.value != "*" && other.value != "*")
    }
}

impl Eq for That {}

#[cfg(test)]
mod tests {
    use crate::tags::that::That;
    #[test]
    fn test_that() {
        assert_eq!(That::new("*"), That::new("*"));
        assert!(That::new("Hi") > That::new("*"));
        assert!(That::new("*") < That::new("Hi"));
    }
}
