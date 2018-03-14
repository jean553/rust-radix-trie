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
    }
}

#[cfg(test)]
mod tests {

    use rt::Node;

    #[test]
    fn test_node_creation() {

        let node = Node::new();
    }
}
