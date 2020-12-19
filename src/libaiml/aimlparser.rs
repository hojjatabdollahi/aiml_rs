use minidom::Element;

use super::aiml::AIML;
use super::aiml::Node;
// use quick_xml::events::Event;
// use quick_xml::Reader;

/// This parses the aiml text inside an aiml file.
pub fn parse(xmldata: &str, root: &mut AIML) {
    // let tokenizer = xmlparser::Tokenizer::from(xmldata); // for token in tokenizer { //     println!("{:?}", token);
    // }
    let NS = minidom::NSChoice::Any;
    let root2: Element = xmldata.parse().unwrap();
    let mut categories: Vec<Node> = Vec::new();
    // Important: _.children()_ ignores the texts. But _.nodes()_ returns those too. Here is an example:
    // <root>hello<child1 />this<child2 />is<child3 />ignored</root> If you want the text, you should use nodes!
    // If you only want the texts, and you want to ignore the elements, you can use: _.texts()_
    for child in root2.children() {
        if child.is("category", NS) {
            let pattern = child.get_child("pattern", NS).unwrap().text();
            let template = child.get_child("template",  NS).unwrap().text();
            categories.push(Node {
                pattern,
                template,
            });
        }
    }
    println!("{:?}", categories);
    // let mut reader = Reader::from_str(xmldata);
    // reader.trim_text(true);
    // If the closing tag has a trailing whitespace, we would've gotten an error
    // reader.trim_markup_names_in_closing_tags(true);

    // let mut count = 0;

    // let mut txt = Vec::new();
    // let mut buf = Vec::new();

    // // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    // let mut pattern_node = false;
    // loop {
    //     match reader.read_event(&mut buf) {
    //         Ok(Event::Start(ref e)) => match e.name() {
    //             b"category" => {
    //                 println!("Found a new category!");
    //             }
    //             b"pattern" => {
    //                 println!(
    //                     "attributes values: {:?}",
    //                     e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
    //                 );
    //                 pattern_node = true;
    //             }
    //             b"template" => count += 1,
    //             _ => (),
    //         },
    //         Ok(Event::Text(e)) => {
    //             txt.push(e.unescape_and_decode(&reader).unwrap());
    //             if (pattern_node) {
    //                 root.append(e.unescape_and_decode(&reader).unwrap());
    //                 pattern_node = false;
    //             }
    //         }
    //         Ok(Event::Eof) => break, // exits the loop when reaching end of file
    //         Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
    //         _ => (), // There are several other `Event`s we do not consider here
    //     }
    // }
    // // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
    // buf.clear();

    // for each category
    parse_category();
}

// parses a category and creates a correspondant element
fn parse_category() {
    // resolve the pattern
    resolve_node();
}

// recursively replace all the values && return the string result
fn resolve_node() {}
