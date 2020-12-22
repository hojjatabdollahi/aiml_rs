use std::cmp::Ordering;

use minidom::{Element, Node, element::Nodes};

use super::aiml::AIML;

/// This parses the aiml text inside an aiml file.
/// This is called by the loader.rs and it receives the content of a file
/// It will add the Nodes to the AIML root
pub fn parse(xmldata: &str, root: &mut AIML) {
    let root2: Element = xmldata.parse().unwrap();

    // Important: _.children()_ ignores the texts. But _.nodes()_ returns those too. Here is an example:
    // <root>hello<child1 />this<child2 />is<child3 />ignored</root> If you want the text, you should use nodes!
    // If you only want the texts, and you want to ignore the elements, you can use: _.texts()_

    for child in root2.children() {
        parse_element(child, &mut root, None);
        // if child.is("category", NS) {
        //     let pattern = child.get_child("pattern", NS).unwrap().text();
        //     let template = child.get_child("template",  NS).unwrap().text();
        //     categories.push(Node {
        //         pattern,
        //         template,
        //     });
        // }
    }

    println!("{:?}", categories);
    // for each category
}

fn parse_element(node: &Element, categories: &mut Vec<Node>, topic: Option<&str>) {
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
           
            
            categories.push(Node { pattern, template, topic });
        }
        "topic" => {
            for child in node.children() {
                parse_element(child, categories, node.attr("name"));
            }
        }
        _ => {
            println!("Unknown tag: {}", node.name());
            todo!(); // Right now the topic is here we have to call a function to handle the topic stuff
        }
    }
}

fn resolve_element(elem: &Element) -> String {
    let result = String::new();
    let mut iter = elem.nodes();
    for node in iter{
        result.push_str(&resolve_node(node));
    }
    result
}

// recursively replace all the values && return the string result
fn resolve_node(node: &Node) -> String {
    return "nope".to_string();
}
