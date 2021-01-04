use crate::tags::node::Node;
use indextree;

#[derive(Debug)]
pub struct AIML {
    arena: indextree::Arena<Node>,
    root_id: indextree::NodeId,
    topics_root_id: indextree::NodeId,
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
    /// all the patterns
    /// It will change indentation when going deeper in a branch
    pub fn tree(&self, verbose: bool) {
        let root_count = self.root_id.children(&self.arena).count();
        println!("No topic (count: {})", root_count);
        if verbose {
            for _ in self.root_id.children(&self.arena) {
                println!("|- c");
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
                for _ in topic.children(&self.arena) {
                    println!("   |- c");
                }
            }
        }
    }

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
                        self.topics_root_id.append(topic_id, &mut self.arena);
                        let new_node_id = self.arena.new_node(new_node);
                        topic_id.append(new_node_id, &mut self.arena);
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
            // The node doesn't have a topic
            debug!("The node doesn't have a topic");
            if self.root_id.children(&self.arena).count() == 0 {
                debug!("Adding the first node without a topic");
                let new_node_id = self.arena.new_node(new_node);
                self.root_id.append(new_node_id, &mut self.arena);
            } else {
                debug!("adding not-the-first node without a topic");
                let first_child = self.arena.get(self.root_id).unwrap().first_child().unwrap();
                let mut best_child = self.root_id;
                //TODO: Currently we only check the <that>
                // Next step is to check the pattern for order
                // The step before that is to insert in the topic
                // So, the order is: topic, then that, and then pattern
                // ex: if there is a topic, go find it and insert into children
                // Now, when inserting, check the that, if that is bigger, then insert (probably
                // becasue there is nothing else with a that
                // but if that is equal, then compare pattern
                // if pattern is bigger insert, if smaller then move forward.
                // the reason that we didn't test "smaller" for that is either, we have a that or we
                // don't. I don't think there is a third option at this point.
                for child in first_child.following_siblings(&self.arena) {
                    if !self.arena.get(child).unwrap().get().is_topic {
                        // TODO: I change > to >= because it was not
                        // inserting a second catergory in the root
                        // I had two categories in the example file, but
                        // the `tree` function was printing only one category
                        // I didn't look any deeper but it looks like that there
                        // was an issue here. Since I'm changing the comparison function
                        // and moving it to a dedicated function, I don't care.
                        if new_node.that >= self.arena.get(child).unwrap().get().that {
                            best_child = child;
                            break;
                        }
                    }
                }
                let new_node_id = self.arena.new_node(new_node);
                best_child.insert_after(new_node_id, &mut self.arena);
            }
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
