#[macro_use]
extern crate log;
extern crate env_logger;
use libaiml::modaiml::{bot::Bot, loader};

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "trace");
    }
    env_logger::init();

    ////let aiml = loader::load_aiml_set("data");
    //let aiml = loader::load_aiml_file("data/test/test.aiml");
    //let mut bot = Bot::new();
    //info!("Q: Hello, A: {}", bot.query("HELLO", &aiml));
    //info!("Q: Doing well, A: {}", bot.query("DOING WELL", &aiml));
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
