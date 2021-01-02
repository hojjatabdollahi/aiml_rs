use crate::modaiml::node::Node;
use indextree;

#[derive(Debug)]
pub struct AIML {
    //pub nodes: Vec<Node>,
    arena: indextree::Arena<Node>,
    root_id: indextree::NodeId,
}

impl AIML {
    pub fn new() -> Self {
        let mut arena = indextree::Arena::new();
        let root_id = arena.new_node(Node::new("root".to_string(), None, None, None));
        Self { arena, root_id }
    }

    pub fn iter(&self) -> impl Iterator<Item = &indextree::Node<Node>> {
        self.arena.iter()
    }

    pub fn insert(&mut self, new_node: Node) {
        // This is literally the first node we are adding
        if self.root_id.children(&self.arena).count() == 0 {
            debug!("Adding the first child");

            let new_node_id = self.arena.new_node(new_node);
            self.root_id.append(new_node_id, &mut self.arena);
        } else {
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
                    if new_node.that > self.arena.get(child).unwrap().get().that {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modaiml::that::That;
    use crate::modaiml::userdata::Userdata;
    #[test]
    fn test_userdata() {
        let mut userdata = Userdata::new();
        userdata.set("test", "testval");
        assert_eq!(userdata.get("test"), Some("testval".to_string()));
        assert!(userdata.get("test2").is_none());
    }

    #[test]
    fn test_aiml() {
        let mut aiml = AIML::new();
        assert_eq!(aiml.iter().count(), 1);
        aiml.insert(Node::new("test".to_string(), None, None, None));
        aiml.insert(Node::new("test".to_string(), None, None, None));
        aiml.insert(Node::new("test".to_string(), None, None, None));
        assert_eq!(aiml.iter().count(), 4);
    }

    #[test]
    fn test_that() {
        assert_eq!(That::new("*"), That::new("*"));
        assert!(That::new("Hi") > That::new("*"));
        assert!(That::new("*") < That::new("Hi"));
    }
}
