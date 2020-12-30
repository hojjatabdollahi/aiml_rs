use super::aiml::{Userdata, AIML};
use super::resolver::resolve;
pub struct Bot {
    userdata: Userdata,
}

impl Bot {
    pub fn new() -> Self {
        Bot {
            userdata: Userdata::new(),
        }
    }

    pub fn query(&mut self, input: &str, root: &AIML) -> String {
        resolve(root, input, &mut self.userdata)
    }
}
