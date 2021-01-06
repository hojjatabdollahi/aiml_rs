use crate::tags::node::Node;
use indextree;

#[derive(Debug)]
pub struct AIML {
    pub arena: indextree::Arena<Node>,
    pub root_id: indextree::NodeId,
    pub topics_root_id: indextree::NodeId,
}

impl AIML {
    pub fn new() -> Self {
        let mut arena = indextree::Arena::new();
        let root_id = arena.new_node(Node::new("root".to_string(), None, None, None));
        let topics_root_id = arena.new_node(Node::new("topics".to_string(), None, None, None));
        Self {
            arena,
            root_id,
            topics_root_id,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &indextree::Node<Node>> {
        self.arena.iter()
    }

    /// This function will traverse the arena and print out
    /// all the categories and topics in a tree form.
    /// It will change indentation when going deeper in a branch
    pub fn tree(&self, verbose: bool) {
        let root_count = self.root_id.children(&self.arena).count();
        println!("No topic (count: {})", root_count);
        if verbose {
            for child in self.root_id.children(&self.arena) {
                println!("|- {}", self.arena.get(child).unwrap().get().pattern);
            }
        }
        println!("|");
        for topic in self.topics_root_id.children(&self.arena) {
            let topic_count = topic.children(&self.arena).count();
            println!(
                "|- {} (count: {}) ",
                self.arena.get(topic).unwrap().get().topic.clone().unwrap(),
                topic_count
            );
            if verbose {
                for child in topic.children(&self.arena) {
                    println!("   |- {}", self.arena.get(child).unwrap().get().pattern);
                }
            }
        }
    }

    pub fn find_topic(&self, topic: &str) -> Option<indextree::NodeId> {
        self.topics_root_id
            .children(&self.arena)
            .find(|&x| self.arena.get(x).unwrap().get().topic.clone().unwrap() == topic)
    }

    /// This function appends a node to a nodeID and makes sure the order is correct
    fn insert_node(&mut self, new_node: Node, root: indextree::NodeId) {
        // The node doesn't have a topic
        debug!("The node doesn't have a topic");
        if root.children(&self.arena).count() == 0 {
            debug!("Adding the first node in this root (could be a topic too)");
            let new_node_id = self.arena.new_node(new_node);
            root.append(new_node_id, &mut self.arena);
        } else {
            debug!("adding another node to this root (or topic)");
            let first_child = self.arena.get(root).unwrap().first_child().unwrap();
            let mut best_child = root;
            //TODO: Currently we only check the <that>
            // Next step is to check the pattern for order
            // So, the order is: topic, then that, and then pattern
            // the reason that we didn't test "smaller" for that
            // is either, we have a that or we
            // don't. I don't think there is a third option at this point.
            let mut bigger = false;
            for child in first_child.following_siblings(&self.arena) {
                // TODO: I change > to >= because it was not
                // inserting a second catergory in the root
                // I had two categories in the example file, but
                // the `tree` function was printing only one category
                // I didn't look any deeper but it looks like that there
                // was an issue here. Since I'm changing the comparison function
                // and moving it to a dedicated function, I don't care.

                if new_node.that > self.arena.get(child).unwrap().get().that {
                    best_child = child;
                    bigger = true;
                    break;
                } else if new_node.that == self.arena.get(child).unwrap().get().that {
                    //TODO: compare pattern
                    if new_node.pattern > self.arena.get(child).unwrap().get().pattern {
                        best_child = child;
                        bigger = true;
                        break;
                    }
                }
                best_child = child;
            }
            //
            if bigger {
                let new_node_id = self.arena.new_node(new_node);
                best_child.insert_before(new_node_id, &mut self.arena);
            } else {
                let new_node_id = self.arena.new_node(new_node);
                best_child.insert_after(new_node_id, &mut self.arena);
            }
        }
    }
    /// This function inserts a node into the arena
    pub fn insert(&mut self, new_node: Node) {
        // TODO: we are adding nodes with topics now, but
        // we are not inserting them in order
        // I think the best option is to move the insertion into a
        // new function and move the comparison into a new function as well

        // if it has a topic
        if new_node.topic.is_some() {
            debug!("The node has a topic");
            debug!("topic is: {:?}", new_node.topic);
            let t = new_node.topic.clone().unwrap();
            if self.topics_root_id.children(&self.arena).count() == 0 {
                debug!("the first topic added: {}", t);
                // no topics yet
                let new_topic_node =
                    self.arena
                        .new_node(Node::new(t.clone(), None, Some(t.clone()), None));
                self.topics_root_id.append(new_topic_node, &mut self.arena);
                let new_node_id = self.arena.new_node(new_node);
                new_topic_node.append(new_node_id, &mut self.arena);
            } else {
                debug!("Not the first topical node");
                // We already have a some topics we should check if this topic already exists
                // If it doesn't exist then we add it.
                match self.topics_root_id.children(&self.arena).find(|&x| {
                    debug!(
                        "comparing the topic: {} to {}",
                        self.arena.get(x).unwrap().get().topic.clone().unwrap(),
                        t
                    );
                    self.arena.get(x).unwrap().get().topic.clone().unwrap() == t
                }) {
                    // topic already exists
                    Some(topic_id) => {
                        debug!("the topic already exists");
                        //self.topics_root_id.append(topic_id, &mut self.arena);
                        //let new_node_id = self.arena.new_node(new_node);
                        //topic_id.append(new_node_id, &mut self.arena);
                        self.insert_node(new_node, topic_id);
                    }
                    // new topic
                    None => {
                        debug!("new topic added: {}", t);
                        let new_topic_node =
                            self.arena
                                .new_node(Node::new(t.clone(), None, Some(t.clone()), None));
                        self.topics_root_id.append(new_topic_node, &mut self.arena);
                        let new_node_id = self.arena.new_node(new_node);
                        new_topic_node.append(new_node_id, &mut self.arena);
                    }
                };
            }
        } else {
            debug!("Node doesn't have a topic");
            self.insert_node(new_node, self.root_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tags::aiml::AIML;
    use crate::tags::node::Node;

    #[test]
    fn test_aiml() {
        let mut aiml = AIML::new();
        assert_eq!(aiml.iter().count(), 2);
        aiml.insert(Node::new("test".to_string(), None, None, None));
        aiml.insert(Node::new("test".to_string(), None, None, None));
        aiml.insert(Node::new("test".to_string(), None, None, None));
        assert_eq!(aiml.iter().count(), 5);
        aiml.insert(Node::new(
            "test".to_string(),
            Some("Sciene".to_string()),
            None,
            None,
        ));
        assert_eq!(aiml.iter().count(), 6);
    }
}
