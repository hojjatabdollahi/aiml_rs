use libaiml::logic::bot::Bot;
use libaiml::storage::loader;
use test_env_log::test;

use std::path::PathBuf;

//
//
//
// To run the tests
// cargo test --package=libaiml  -- --nocapture
//
//
//
//

#[test]
fn integration_test() {
    let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_dir.push("../data/test/test.aiml");
    let aiml = loader::load_aiml_file(test_dir.to_str().unwrap());
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
