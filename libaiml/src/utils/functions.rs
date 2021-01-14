pub fn is_match(input: &str, pattern: &str) -> bool {
    is_match_vec(normalize_input(input), normalize_pattern(pattern))
}

fn is_match_vec(input: Vec<String>, pattern: Vec<String>) -> bool {
    //TODO: I assume that there are not two stars next to eachother a**b
    // You need to do a clean up, make sure that the case of the letters
    // are the same too

    // I can not use arrays (because we don't know the length)
    // So, I should use a Vec and I will make it look like an array
    //
    //debug!("input: {:?}, pattern: {:?}", input, pattern);

    let pattern_size = pattern.len() + 1;
    let input_size = input.len() + 1;

    // Base 1d array
    let mut t_raw = vec![false; input_size * pattern_size];

    // Vector of 'width' elments slices
    let mut t_base: Vec<_> = t_raw.as_mut_slice().chunks_mut(pattern_size).collect();

    // Findal 2d array `&mut [&mut [..]]`
    let t = t_base.as_mut_slice();

    // alg: we start with true
    t[0][0] = true;

    // alg: if the pattern starts with a star, then the second cell is true too
    if pattern_size > 0 && pattern[0] == "*" {
        t[0][1] = true;
    }

    for i in 1..input_size {
        for j in 1..pattern_size {
            //debug!("pattern: {}, input: {}", pattern[j - 1], input[i - 1]);
            if pattern[j - 1] == input[i - 1] {
                t[i][j] = t[i - 1][j - 1];
            } else if pattern[j - 1] == "*" {
                t[i][j] = t[i - 1][j] || t[i][j - 1];
            }
        }
    }

    t[input_size - 1][pattern_size - 1]
}

fn normalize_input(input: &str) -> Vec<String> {
    input
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn normalize_pattern(pattern: &str) -> Vec<String> {
    let mut res = Vec::new();
    let pat = pattern.trim().to_lowercase();
    let pat = pat.split_whitespace().collect::<Vec<&str>>();
    let mut first = true;
    for word in pat {
        if word == "*" {
            if first {
                first = false;
                res.push(word.to_string());
            }
        } else {
            res.push(word.to_string());
            first = true;
        }
    }
    res
}

/// Append the <input>, <that> and <topic> for the matching
/// # Examples
///
/// ```
/// # use libaiml::utils::functions::input_that_topic;
/// assert_eq!(
///     input_that_topic("a", Some("b"), Some("c")),
///     "<topic> c <that> b <pattern> a"
/// );
/// assert_eq!(input_that_topic("a", None, None),"<topic> * <that> * <pattern> a");
/// ```
pub fn input_that_topic(input: &str, that: Option<&str>, topic: Option<&str>) -> String {
    let mut result = String::new();
    result.push_str("<topic> ");
    match topic {
        Some(txt) => result.push_str(txt.trim()),
        None => result.push_str("*"),
    }
    result.push_str(" <that> ");
    match that {
        Some(txt) => result.push_str(txt.trim()),
        None => result.push_str("*"),
    }
    result.push_str(" <pattern> ");
    result.push_str(input.trim());
    result.to_lowercase()
}

#[cfg(test)]
mod tests {
    use crate::utils::functions::{is_match, normalize_input, normalize_pattern};
    use test_env_log::test;
    #[test]
    fn test_is_match() {
        assert!(is_match("Hello friend", "Hello *"));
        assert!(is_match("Hello friend I love you", "Hello *"));
        assert!(is_match("Hello friend I love you", "Hello * you *"));
        assert_eq!(
            is_match("Hello friend I love you", "Hello * you * too"),
            false
        );
    }

    #[test]
    fn test_normalize_input() {
        assert_eq!(
            normalize_input("Hello \t\u{2009}   darling   \n \r"),
            vec!["hello".to_string(), "darling".to_string()]
        );
    }

    #[test]
    fn test_normalize_pattern() {
        assert_eq!(
            normalize_pattern("* Hello \t\u{2009}   * * * darling *   \n \r"),
            vec![
                "*".to_string(),
                "hello".to_string(),
                "*".to_string(),
                "darling".to_string(),
                "*".to_string()
            ]
        );
    }
}
