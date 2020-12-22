#[macro_use]
extern crate log;
extern crate env_logger;

mod libaiml;


fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "trace");
    }
    env_logger::init();

    // let aiml = libaiml::loader::load_aiml_set("data");
    let aiml = libaiml::loader::load_aiml_file("data/root.aiml");
    info!("{:?}", aiml);
    for node in aiml {
        info!("Node: {:?}", node);
    }
}
