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

            let mut contained_word = true;
            let mut different_character_index = 0;

            for (index, character) in self.characters.chars().enumerate() {

                if character != (word.as_bytes()[index] as char) {

                    contained_word = false;
                    different_character_index = index;
                    break;
                }
            }

            if contained_word && self.children.is_empty() {
                self.characters = word.to_string();
                return;
            }

            let characters = self.characters.clone();
            let (first, second) = characters.split_at(
                different_character_index as usize
            );

            self.characters = first.to_string();
            self.children.push(Node::new(second));

            let word = word.to_string();
            let (_, second) = word.split_at(
                different_character_index as usize
            );

            self.children.push(Node::new(second));
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
mod tests;
