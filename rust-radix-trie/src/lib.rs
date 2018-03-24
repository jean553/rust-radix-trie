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

            let mut separator_index: Option<usize> = None;

            /* check if the word is present into
               the first part of the node characters */

            for (index, character) in self.characters.chars().enumerate() {

                if character != (word.as_bytes()[index] as char) {
                    separator_index = Some(index);
                    break;
                }
            }

            /* if the first part of the node characters is exactly
               the same as the word, then just replace it by the node
               (if there is no child) */

            if separator_index.is_none() && self.children.is_empty() {
                self.characters = word.to_string();
                return;
            }

            /* in any other case, keep only the common part and set it
               as the current node characters; the current node second
               part is moved into a new child; the inserted word second
               part is also moved into a new child */

            let word = word.to_string();

            if separator_index.is_none() {
                separator_index = Some(self.characters.len());
            }

            let separator_index = separator_index.unwrap();

            let (_, word_second) = word.split_at(separator_index);

            if self.children.is_empty() {

                let characters = self.characters.clone();
                let (first, second) = characters.split_at(separator_index);

                self.characters = first.to_string();
                self.children.push(Node::new(second));
                self.children.push(Node::new(word_second));

                return;
            }

            for child in self.children.iter_mut() {

                let child_characters = child.get_characters().to_string();

                if child_characters.len() > word_second.len() {
                    continue;
                }

                let (inserable, _) = word_second.split_at(
                    child_characters.len()
                );

                if child.children.is_empty() && child_characters == inserable {
                    child.set_characters(word_second);
                    return;
                }
            }

            self.children.push(Node::new(word_second));
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

            for (index, character) in self.characters.chars().enumerate() {

                if index == word.len() {
                    return true;
                }

                if character != (word.as_bytes()[index] as char) {
                    return false;
                }
            }

            if self.characters.len() == word.len() {
                return true;
            }

            let (_, second) = word.split_at(self.characters.len());
            let mut exists_into_child = false;

            for child in self.children.iter() {

                exists_into_child = child.exists(second);

                if exists_into_child {
                    break;
                }
            }

            exists_into_child
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

        /// Setter of the node characters.
        ///
        /// # Args:
        ///
        /// `characters` - the characters to use
        pub fn set_characters(&mut self, characters: &str) {
            self.characters = characters.to_string();
        }
    }
}

#[cfg(test)]
mod tests;
