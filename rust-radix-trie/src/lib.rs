#[allow(dead_code)]

mod rt {

    /// A radix trie node with a string (array of characters) and children to other nodes.
    pub struct Node {
        characters: String,
        children: Vec<Node>,
    }

    impl Node {

        /// Creates a new radix trie node, with an empty array of characters and an empty list of children nodes.
        ///
        /// # Returns:
        ///
        /// new radix trie node
        pub fn new() -> Node {
            Node {
                characters: String::new(),
                children: Vec::new(),
            }
        }

        /// Inserts a new word into the radix trie (may create new nodes).
        ///
        /// # Arguments:
        ///
        /// `word` - the new word to store
        pub fn insert(&mut self, word: &str) {

            /* FIXME: should browse the characters from the beginning
               and continuously compare it with the word characters */

            self.characters = word.to_string();
        }

        /// Getter of the characters stored into the node.
        ///
        /// # Returns:
        ///
        /// the characters into the node
        pub fn get_characters(&self) -> &str {
            &self.characters
        }
    }
}

#[cfg(test)]
mod tests {

    use rt::Node;

    #[test]
    fn test_node_creation_and_insertion() {

        let mut node = Node::new();
        node.insert("word");
    }
}
