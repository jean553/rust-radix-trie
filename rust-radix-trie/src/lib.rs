#[allow(dead_code)]

mod rt {

    /// A radix trie node with a string (array of characters) and children to other nodes.
    #[derive(Clone)]
    pub struct RadixTrie {
        characters: String,
        children: Vec<RadixTrie>,
    }

    /// RadixTrie creation factory,
    ///
    /// # Args:
    ///
    /// `characters` - the characters to store into the created node
    ///
    /// # Returns:
    ///
    /// new node
    fn create_node(characters: &str) -> RadixTrie {

        RadixTrie {
            characters: characters.to_string(),
            children: Vec::new(),
        }
    }

    impl RadixTrie {

        /// Creates a new radix trie, with an empty array of characters and an empty list of children nodes.
        ///
        /// # Arguments:
        ///
        /// `characters` - the characters to store into the first node of the trie (after the root node)
        ///
        /// # Returns:
        ///
        /// new radix trie
        pub fn new(characters: &str) -> RadixTrie {

            RadixTrie {
                characters: String::new(),
                children: vec![create_node(characters)],
            }
        }

        /// Inserts a new word into the radix trie (may create new nodes).
        ///
        /// # Arguments:
        ///
        /// `word` - the new word to store
        pub fn insert(&mut self, word: &str) {

            /* check if a new child of the current node has to be inserted */

            let mut child_index: Option<usize> = None;

            for (index, child) in self.children.iter().enumerate() {

                let separator = child.contains_word(word);
                if separator.is_none() || separator.unwrap() != 0 {
                    child_index = Some(index);
                    break;
                }
            }

            if child_index.is_none() {
                self.children.push(create_node(word));
                return;
            }

            /* iterate to one selected child node
               if insertion of the text can be done there */

            self.children[child_index.unwrap()].insert_node(word);
        }

        /// Recursively browse the radix trie in order to insert the word (may create new nodes).
        ///
        /// # Arguments:
        ///
        /// `word` - the new word to store
        fn insert_node(&mut self, word: &str) {

            let mut index = self.contains_word(word);

            let word = word.to_string();

            if index.is_none() {

                if self.children.is_empty() {
                    self.characters = word;
                    return;
                }

                index = Some(self.characters.len());
            }

            let index = index.unwrap();

            let (_, word) = word.split_at(index);

            if self.children.is_empty() {
                self.create_children(index, word);
                return;
            }

            let mut selected_child: Option<usize> = None;

            for (index, child) in self.children.iter().enumerate() {

                let child_characters = child.get_characters().to_string();

                let (inserable, _) = if word.len() >= child_characters.len() {
                    word.split_at(child_characters.len())
                } else {
                    (word, "")
                };

                if child.get_children().is_empty() &&
                    child_characters == inserable {
                    selected_child = Some(index);
                    break;
                }

                if child_characters.as_bytes()[0] == word.as_bytes()[0] {
                    selected_child = Some(index);
                }
            }

            if selected_child.is_some() {
                self.children[selected_child.unwrap()].insert_node(word);
                return;
            }

            if index != self.characters.len() {

                /* in that case, modification of the current node characters
                   is required; it is also required to move the current node
                   children as sub-children of a new child */

                let characters = self.characters.clone();

                let (
                    saved_characters,
                    moved_characters
                ) = characters.split_at(index);

                self.characters = saved_characters.to_string();

                let mut last_child = create_node(moved_characters);

                last_child.children = self.children
                    .iter()
                    .map(|child| { (*child).clone() })
                    .collect();

                self.children.clear();

                self.children.push(last_child);
                self.children.push(create_node(word));

                return;
            }

            self.children.push(create_node(word));
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

        /// Indicates if the node contains the given word. That means if the word is the beginning of the node contained characters, or if the word is exactly the node characters.
        ///
        /// # Args:
        ///
        /// `word` - the word to find
        ///
        /// # Returns:
        ///
        /// The index of the first different character between the two words or none if no
        /// difference is found after browsing the node characters and comparing with the word
        fn contains_word(&self, word: &str) -> Option<usize> {

            for (index, character) in self.characters.chars().enumerate() {
                if character != (word.as_bytes()[index] as char) {
                    return Some(index);
                }
            }

            None
        }

        /// Takes the node characters and extract the part from the given separator index in order to create a first child. The second child is simply created with the given word.
        ///
        /// # Args:
        ///
        /// `separator` - the index of the separator where the node word has to be divided
        /// `word` - the word to insert into the second new created child
        fn create_children(&mut self, separator: usize, word: &str) {

            let characters = self.characters.clone();
            let (first, second) = characters.split_at(separator);

            self.characters = first.to_string();
            self.children.push(create_node(second));
            self.children.push(create_node(word));
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
        pub fn get_children(&self) -> &Vec<RadixTrie> {
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
