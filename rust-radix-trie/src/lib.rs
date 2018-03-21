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
        /// # Arguments:
        ///
        /// `characters` - the characters to store into the created node
        ///
        /// # Returns:
        ///
        /// new radix trie node
        pub fn new(characters: &str) -> Node {
            Node {
                characters: characters.to_string(),
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
               and continuously compare it with the word characters;
               should not directly leave if there are different characters */

            if word[..self.characters.len()] == self.characters &&
                self.children.is_empty() {
                self.characters = word.to_string();

                return;
            }

            /* FIXME: this should only happens when the current character
               is not the first character of any current node child */

            self.children.push(Node::new(&self.characters));
            self.children.push(Node::new(word));

            self.characters = "".to_string();
        }

        /// Indicates if a word exists into the radix trie
        ///
        /// # Arguments:
        ///
        /// `word` - the word to search for
        ///
        /// # Returns:
        ///
        /// True if the word exists, False if the word does not exist
        pub fn exists(&self, word: &str) -> bool {

            /* FIXME: should search for the word into children */

            if self.characters == word {
                return true;
            }

            false
        }

        /// Getter of the characters stored into the node.
        ///
        /// # Returns:
        ///
        /// the characters into the node
        pub fn get_characters(&self) -> &str {
            &self.characters
        }

        /// Getter of the children of the node.
        ///
        /// # Returns:
        ///
        /// list of children
        pub fn get_children(&self) -> &Vec<Node> {
            &self.children
        }
    }
}

#[cfg(test)]
mod tests {

    use rt::Node;

    #[test]
    fn test_node_creation_and_insertion() {

        let mut node = Node::new("hello");

        const INSERTED_CHARACTERS: &str = "helloworld";
        node.insert(INSERTED_CHARACTERS);
        assert_eq!(node.get_characters(), INSERTED_CHARACTERS);
    }

    #[test]
    fn test_two_children_from_root() {

        const FIRST_CHARACTERS: &str = "hello";
        const SECOND_CHARACTERS: &str = "bonjour";

        let mut node = Node::new(FIRST_CHARACTERS);

        const INSERTED_CHARACTERS: &str = SECOND_CHARACTERS;
        node.insert(INSERTED_CHARACTERS);

        const ROOT_NODE_EXPECTED_CHARACTERS: &str = "";
        assert_eq!(node.get_characters(), ROOT_NODE_EXPECTED_CHARACTERS);

        /* FIXME: order should not matter when getting the children */

        let children = node.get_children();
        assert_eq!(children[0].get_characters(), FIRST_CHARACTERS);
        assert_eq!(children[1].get_characters(), SECOND_CHARACTERS);

        assert_eq!(node.get_characters(), "");
    }

    #[test]
    fn test_get_children_from_root_are_empty_by_default() {

        let node = Node::new("hello");
        assert_eq!(node.get_children().is_empty(), true);
    }

    #[test]
    fn test_characters_exist_into_root_node() {

        let node = Node::new("hello");

        assert_eq!(node.exists("hello"), true);
        assert_eq!(node.exists("bonjour"), false);
    }
}
