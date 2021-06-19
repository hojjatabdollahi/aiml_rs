// Important: _.children()_ ignores the texts. But _.nodes()_ returns those too. Here is an example:
// <root>hello<child1 />this<child2 />is<child3 />ignored</root> If you want the text, you should use nodes!
// If you only want the texts, and you want to ignore the elements, you can use: _.texts()_

use minidom::Element;

use crate::tags::aiml::AIML;
use crate::tags::node;

// Just to save a little bit of memory
// Also AIML files do not use namespaces
const NS: minidom::NSChoice = minidom::NSChoice::Any;

/// This parses the aiml text inside an aiml file.
/// This is called by the loader.rs and it receives the content of a file
/// It will add the Nodes to the AIML root
pub fn parse(xmldata: &str, root: &mut AIML) {
    let xmldata: Element = xmldata.parse().unwrap();

    for child in xmldata.children() {
        parse_element(child, root, None);
    }
}

fn parse_element(node: &Element, root: &mut AIML, topic: Option<&str>) {
    let topic = topic.map(String::from);
    match node.name() {
        "category" => {
            let pattern = node
                .get_child("pattern", NS)
                .unwrap()
                .text()
                .trim()
                .to_owned();
            //let that = node.get_child("that", NS).cloned();
            // let that = match node.get_child("that", NS) {
            //     Some(e) => Some(e.text().trim().to_owned()),
            //     None => None,
            // };
            let that = node
                .get_child("that", NS)
                .map(|e| e.text().trim().to_owned());
            let template = node.get_child("template", NS).cloned();
            //TODO: I should insert the nodes in order, so
            // When it is being inserted it should keep the order
            // with the siblings
            root.insert(node::Node::new(pattern, that, topic, template));
        }
        //TODO: I'm going to create a node in the arena and pass that as
        // the root for all the stuff in that topic.
        // The thing is, I should first check that that topic
        // doesn't already exist in the arena
        "topic" => {
            for child in node.children() {
                parse_element(child, root, node.attr("name"));
            }
        }
        _ => {
            error!("Unknown tag: {}", node.name());
            todo!(); // Right now the topic is here we have to call a function to handle the topic stuff
        }
    }
}
