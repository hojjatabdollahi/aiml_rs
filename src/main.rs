#[macro_use]
extern crate log;
extern crate env_logger;
use libaiml::modaiml::{bot::Bot, loader};

// TODO: Use the wildcard functions in the code
// How about <sets>? Well, I think the wildcard function
// can improve, instead of `word == word` I can use a function
// that searches the <sets>
//
// TODO: implement the topics as a branch of the tree for eficiency

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "trace");
    }
    env_logger::init();

    ////let aiml = loader::load_aiml_set("data");
    let aiml = loader::load_aiml_file("data/test/test.aiml");
    let mut bot = Bot::new();
    info!("Q: Hello, A: {}", bot.query("HELLO", &aiml));
    info!("Q: Doing well, A: {}", bot.query("DOING WELL", &aiml));
    //info!("Q: Doing well, A: {}", bot.query("DOING WELL", &aiml));
    //info!("Q: What what?, A: {}", bot.query("WHAT WHAT", &aiml));
    //info!("q: how are you, a: {}", bot.query("HOW ARE YOU", &aiml));
    //info!(
    //"q: Let's do an adventure, a: {}",
    //bot.query("LETS DO AN ADVENTURE", &aiml)
    //);
    //info!("q: no, a: {}", bot.query("NO", &aiml));
    //info!("q: yes, a: {}", bot.query("YES", &aiml));
}
