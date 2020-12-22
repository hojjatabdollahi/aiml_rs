#[macro_use]
extern crate log;
extern crate env_logger;

mod libaiml;

/// This is the struct that will contain all the data
#[derive(Debug)]
struct Aiml {
    pattern: String,
    that: String,
}
// impl Aiml {
//     fn new(pattern: String, that: String) -> Self {
//         //! This is a function that creates a new AIML tag
//         Self { pattern, that }
//     }
// }

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "trace");
    }
    env_logger::init();

    let aiml = libaiml::loader::load_aiml_set("data");
    info!("{:?}", aiml);
    for node in aiml {
        info!("Node: {:?}", node);
    }

    // let mut root = trees::fr::<Aiml>();
    // root.push_back(trees::Tree::new(Aiml::new(
    //     String::from("Hello"),
    //     String::new(),
    // )));
    // root.push_back(trees::Tree::new(Aiml::new(
    //     String::from("Hello"),
    //     String::new(),
    // )));
    // info!("{:?}", root);
}
