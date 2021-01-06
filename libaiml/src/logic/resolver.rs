use crate::logic::userdata::Userdata;
use crate::tags::aiml::AIML;
use crate::utils::functions::input_that_topic;
use minidom::Element;

/// This function finds the answer and also makes changes to the userdata
/// userdata is used to store user's information that might need to be
/// used in the answer, or set for the next question
pub fn resolve(root: &AIML, input: &str, userdata: &mut Userdata) -> String {
    let mut output = String::new();
    match find_with_userdata(root, input, userdata) {
        Some(res) => {
            debug!("I found this answer: {:?}", res);
            //TODO: I don't need this. I should alaways set something as then
            // that tag, if there is no category with that question, then
            // It will default to *
            let mut set_that = false;
            for child in res.nodes() {
                //trace!("{:?}", child);
                let txt = child.as_text();
                let elem = child.as_element();
                if txt.is_some() {
                    output.push_str(txt.unwrap().trim());
                    output.push_str(" ");
                }
                if elem.is_some() {
                    let elem = elem.unwrap();
                    let tag = elem.name();
                    let txt = elem.text();
                    if tag == "set" {
                        for attr in elem.attrs() {
                            if attr.0 == "name" {
                                userdata.set(attr.1, &txt);
                                if attr.1 == "most recent dialogue question" {
                                    set_that = true;
                                }
                                debug!("Just set {:?} to {:?}", attr.1, txt);
                            }
                        }
                        // I add the text if it is <set>
                        // But if it is <think><set> then I shouldn't
                        output.push_str(&elem.text().trim());
                        output.push_str(" ");
                    }

                    if tag == "think" {
                        for grandchildren in elem.nodes() {
                            let subelem = grandchildren.as_element();
                            if subelem.is_some() {
                                let subelem = subelem.unwrap();
                                let subtag = subelem.name();
                                let subtxt = subelem.text();
                                if subtag == "set" {
                                    for attr in subelem.attrs() {
                                        if attr.0 == "name" {
                                            userdata.set(attr.1, &subtxt);
                                            if attr.1 == "most recent dialogue question" {
                                                set_that = true;
                                            }
                                            debug!("Just set {:?} to {:?}", attr.1, subtxt);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !set_that {
                userdata.remove("most recent dialogue question");
            }
            debug!("output: {}", output);
        }
        None => {
            output.push_str("I don't have an answer for that");
        }
    }
    output.trim().to_string()
}

fn find_with_userdata(root: &AIML, input: &str, userdata: &Userdata) -> Option<Element> {
    find(
        root,
        input,
        userdata.get("most recent dialogue question"),
        userdata.get("topic"),
    )
}

/// This function finds the node that matches the input
/// We need the input, plus "that" and "topic".
/// As far as I know, we set the <that> using <set> so,
/// We can have a variable that keeps the lates <that> for
/// the current user. Also it can contain the <topic> as well.
/// We then pass those variables to this find() function.
/// It returns the <template>
///
/// Check the find() test function in this file for an example.
fn find(root: &AIML, input: &str, that: Option<String>, topic: Option<String>) -> Option<Element> {
    let mut id = root.root_id;
    let mut res: Option<Element> = None;
    match topic {
        Some(t) => {
            id = root.find_topic(&t).unwrap();
        }
        None => {}
    };

    for child in id.children(&root.arena) {
        let node = root.arena.get(child).unwrap().get();
        let mut ok = false;
        match &that {
            Some(t) => {
                match &node.that {
                    Some(T) => {
                        debug!("<That> comparing {}, {}", t, T.value);
                        if t == &T.value {
                            ok = true;
                        }
                    }
                    None => {}
                };
            }
            None => match &node.that {
                Some(_) => {}
                None => {
                    ok = true;
                }
            },
        }
        if ok {
            if node.is_match(input) {
                res = node.template.clone();
                break;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::logic::userdata::Userdata;
    use crate::tags::aiml::AIML;
    use crate::tags::node::Node;
    use minidom::Element;

    // This creates an AIML object so that we search in it
    fn setup() -> (AIML, Userdata) {
        let mut aiml = AIML::new();
        aiml.insert(Node::new(
            "Hi".to_string(),
            Some("hello".to_string()),
            None,
            Some(Element::builder("hello").build()),
        ));
        let mut userdata = Userdata::new();
        userdata.set("most recent dialogue question", "hello");
        (aiml, userdata)
    }

    #[test]
    fn test_find() {
        assert_eq!(
            super::find(&setup().0, "hi", Some("hello".to_string()), None)
                .unwrap()
                .name(),
            "hello"
        );
    }

    #[test]
    fn test_find_with_userdata() {
        let (aiml, userdata) = setup();
        assert_eq!(
            super::find_with_userdata(&aiml, "hi", &userdata)
                .unwrap()
                .name(),
            "hello"
        );
    }
}
