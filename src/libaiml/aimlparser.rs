// Important: _.children()_ ignores the texts. But _.nodes()_ returns those too. Here is an example:
// <root>hello<child1 />this<child2 />is<child3 />ignored</root> If you want the text, you should use nodes!
// If you only want the texts, and you want to ignore the elements, you can use: _.texts()_


use minidom::{Element, Node};

use super::aiml;
use aiml::AIML;

/// This parses the aiml text inside an aiml file.
/// This is called by the loader.rs and it receives the content of a file
/// It will add the Nodes to the AIML root
pub fn parse(xmldata: &str, root: &mut AIML) {
    let xmldata: Element = xmldata.parse().unwrap();

    for child in xmldata.children() {
        parse_element(child, root, None);
    }

    println!("{:?}", root);
}

fn parse_element(node: &Element, root: &mut AIML, topic: Option<&str>) {
    let ns = minidom::NSChoice::Any;
    let topic = match topic {
        Some(s) => {
            Some(String::from(s))
        }
        None => {None}
    };
    match node.name() {
        "category" => {
            let pattern = node.get_child("pattern", ns).unwrap().text().trim().to_owned();
            let template = node.get_child("template", ns).cloned();
            root.nodes.push(aiml::Node { pattern, template, topic });
        }
        "topic" => {
            for child in node.children() {
                parse_element(child, root, node.attr("name"));
            }
        }
        _ => {
            println!("Unknown tag: {}", node.name());
            todo!(); // Right now the topic is here we have to call a function to handle the topic stuff
        }
    }
}
