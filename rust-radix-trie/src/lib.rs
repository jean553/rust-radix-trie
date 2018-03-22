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

            /* check if the word is present into
               the first part of the node characters */

            for (index, character) in self.characters.chars().enumerate() {

                if character != (word.as_bytes()[index] as char) {
                    contained_word = false;
                    different_character_index = index;
                    break;
                }
            }

            /* if the first part of the node characters is exactly
               the same as the word, then just replace it by the node
               (if there is no child) */

            if contained_word && self.children.is_empty() {
                self.characters = word.to_string();
                return;
            }

            /* in any other case, keep only the common part and set it
               as the current node characters; the current node second
               part is moved into a new child; the inserted word second
               part is also moved into a new child */

            if self.children.is_empty() {

                let characters = self.characters.clone();
                let (first, second) = characters.split_at(
                    different_character_index as usize
                );

                self.characters = first.to_string();
                self.children.push(Node::new(second));
            }

            if contained_word {
                different_character_index = self.characters.len();
            }

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
