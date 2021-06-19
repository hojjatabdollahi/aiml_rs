use std::cmp::{Ord, Ordering};
#[derive(Debug, Clone)]
pub struct Pattern {
    pub value: String,
}

impl Pattern {
    pub fn new(str: &str) -> Self {
        Self {
            value: str
                .to_owned()
                .clear_punctuation()
                .expand_contractions()
                .correct_spelling_mistakes()
                .ensure_one_space(),
        }
    }
}

impl Ord for Pattern {
    //fn cmp(&self, other: &Self) -> Ordering {
    //if self.value == other.value {
    //Ordering::Equal
    //} else if self.value == "*" {
    //if other.value == "*" {
    //// self == *, other == *
    //Ordering::Equal
    //} else {
    //// self == *, other == word
    //Ordering::Less
    //}
    //} else {
    //// self == word
    //if other.value == "*" {
    //// self == word, other == *
    //Ordering::Greater
    //} else {
    //// self == word, other == word
    //Ordering::Equal
    //}
    //}
    //}

    fn cmp(&self, other: &Self) -> Ordering {
        if self.value == other.value {
            Ordering::Equal
        } else {
            let mut a = self.value.split_whitespace();
            let mut b = other.value.split_whitespace();
            let mut res = std::cmp::Ordering::Equal;
            loop {
                match a.next() {
                    Some(v) => {
                        match b.next() {
                            Some(v2) => {
                                if v == "*" && v2 != "*" {
                                    res = std::cmp::Ordering::Less;
                                    break;
                                } else if v != "*" && v2 == "*" {
                                    res = std::cmp::Ordering::Greater;
                                    break;
                                }
                            }
                            None => {
                                if v == "*" {
                                    res = std::cmp::Ordering::Less;
                                }
                                break;
                            }
                        };
                    }
                    None => {
                        match b.next() {
                            Some(v2) => {
                                if v2 == "*" {
                                    res = std::cmp::Ordering::Greater;
                                }
                                break;
                            }
                            None => {
                                break;
                            }
                        };
                    }
                }
            }
            res
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

trait PatternExt {
    /// Repalce puncutations with white space. You should run `ensure_one_space` after this command
    fn clear_punctuation(&self) -> Self;
    fn expand_contractions(&self) -> Self;
    fn correct_spelling_mistakes(&self) -> Self;
    fn ensure_one_space(&self) -> Self;
}

impl PatternExt for String {
    // TODO: This removes all the non_alaphbet_digit characters including * # $ ^ _ < > which are important to us
    // What to do?
    fn clear_punctuation(&self) -> Self {
        self.chars()
            .map(|c| if c.is_ascii_punctuation() { ' ' } else { c })
            .collect()
    }
    fn expand_contractions(&self) -> Self {
        self.to_owned()
    }
    fn correct_spelling_mistakes(&self) -> Self {
        self.to_owned()
    }
    fn ensure_one_space(&self) -> Self {
        self.split_whitespace().collect::<Vec<&str>>().join(" ")
    }
}
#[test]
fn test_pattern_ext() {
    assert!("hello".to_owned().correct_spelling_mistakes().eq("hello"));
    assert!("hello".to_owned().expand_contractions().eq("hello"));
    assert!("hello".to_owned().clear_punctuation().eq("hello"));
    assert!("hello.".to_owned().clear_punctuation().eq("hello "));
    assert!("hello,".to_owned().clear_punctuation().eq("hello "));
    assert!("hello;".to_owned().clear_punctuation().eq("hello "));
    assert!("hello:".to_owned().clear_punctuation().eq("hello "));
    assert!("hello:world"
        .to_owned()
        .clear_punctuation()
        .eq("hello world"));
    // TODO: change these previous tests to use `assert_eq` to give a better error if they fail
    assert_eq!("hello", "hello".to_owned().ensure_one_space());
    assert_eq!("hello world", "hello world".to_owned().ensure_one_space());
    assert_eq!("hello world", "hello  world".to_owned().ensure_one_space());
    assert_eq!("hello world", "hello  world ".to_owned().ensure_one_space());
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Pattern>: {}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::tags::pattern::Pattern;
    #[test]
    fn test_pattern() {
        assert_eq!(Pattern::new("*"), Pattern::new("*"));
        assert!(Pattern::new("Hi") > Pattern::new("*"));
        assert!(Pattern::new("*") < Pattern::new("Hi"));
        assert!(Pattern::new("hello *") < Pattern::new("hello"));
        assert!(Pattern::new("hello") > Pattern::new("hello *"));
        assert!(Pattern::new("hello") > Pattern::new("* hello *"));
    }
}
