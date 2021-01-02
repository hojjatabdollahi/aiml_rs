use std::collections::HashMap;

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
    use crate::logic::userdata::Userdata;
    #[test]
    fn test_userdata() {
        let mut userdata = Userdata::new();
        userdata.set("test", "testval");
        assert_eq!(userdata.get("test"), Some("testval".to_string()));
        assert!(userdata.get("test2").is_none());
    }
}
