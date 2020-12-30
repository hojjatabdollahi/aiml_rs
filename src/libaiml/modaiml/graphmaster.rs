//#[derive(Debug)]
//pub struct Graphmaster {
//root: Nodemapper,
//}

//#[derive(Debug)]
//pub struct Nodemapper {
//category: Category,
//}

//#[derive(Debug)]
//pub struct Category {
//input: String,
//that: String,
//topic: String,
//}

//impl Graphmaster {
//pub fn category_exists(&self, c: &Category) -> bool {
//match self.find_node(&c.input, &c.that, &c.topic) {
//Some(_) => true,
//None => false,
//}
//}

//pub fn find_node(&self, input: &str, that: &str, topic: &str) -> Option<Nodemapper> {
//self.find_node_in_tree(
//&self.root,
//Self::input_that_topic(input, that, topic)
//.split(' ')
//.collect(),
//)
//}

//pub fn find_node_in_tree(&self, _node: &Nodemapper, _path: Vec<&str>) -> Option<Nodemapper> {
//None
//}
//}

//#[cfg(test)]
//mod tests {
//use super::Category;
//use super::Graphmaster;
//use super::Nodemapper;

//fn make_root() -> Graphmaster {
//Graphmaster {
//root: Nodemapper {
//category: Category {
//input: "a".to_string(),
//that: "b".to_string(),
//topic: "c".to_string(),
//},
//},
//}
//}

//#[test]
//fn works_input_that_topic() {
//assert_eq!(
//Graphmaster::input_that_topic("a", "b", "c"),
//"a <that> b <topic> c"
//);
//}

//#[test]
//fn find_node() {
//assert!(make_root().find_node("a", "b", "c").is_none());
//}
//}
