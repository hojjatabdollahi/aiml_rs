#[macro_use]
extern crate log;
extern crate env_logger;
pub mod logic;
pub mod storage;
mod tags;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::logic::bot::Bot;
    use crate::storage::loader;
    #[test]
    fn integration_test() {
        let aiml = loader::load_aiml_file("data/test/test.aiml");
        let mut bot = Bot::new();
        assert_eq!(bot.query("HELLO", &aiml), "Well, hello! How are you doing?");
        assert!(bot.query("DOING WELL", &aiml) == "To be honest, I don't care how you are doing.");
        assert!(bot.query("DOING WELL", &aiml) == "What?");
        assert!(bot.query("WHAT WHAT", &aiml) == "Wow, that got confusing, fast.");
        assert!(bot.query("HOW ARE YOU", &aiml) == "I'm a bot, silly!");
        assert!(
        bot.query("LETS DO AN ADVENTURE", &aiml) == "There's nothing like a good adventure to liven things up! Have you ever read The Adventures of Tom Sawyer?");
        assert!(bot.query("NO", &aiml) == "I don't have an answer for that");
        assert!(bot.query("YES", &aiml) == "What did you think of all the mischief Tom got into?");
    }
}
