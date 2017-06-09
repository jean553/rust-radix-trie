mod rt {

    use std::ptr;

    struct RadixTrieNodeChild {
        next: Option<Box<RadixTrieNodeChild>>,
        node: Box<RadixTrieNode>,
    }

    pub struct RadixTrieNode {
        key: String,
        children: Option<Box<RadixTrieNodeChild>>,
    }

    impl RadixTrieNode {

        /// Creates a new radix trie node
        pub fn new(key: &str) -> RadixTrieNode {

            RadixTrieNode {
                key: key.to_string(),
                children: None,
            }
        }

        /// Inserts a new key into the radix trie
        ///
        /// NOTE: partially implemented
        pub fn insert(&mut self, key: &str) {

            /* insert a new node child for the current node,
               the new child next node is the last inserted
               child inserted before */
            let new_node = Some(Box::new(RadixTrieNodeChild {
                next: self.children.take(),
                node: Box::new(RadixTrieNode {
                    key: key.to_string(),
                    children: None,
                })
            }));

            /* replace the previous last node child by the new one */
            self.children = new_node;
        }

        /// Checks if a key exists inside the radix trie
        ///
        /// NOTE: This function is partialy implemented;
        /// by now, it simply looks at the first child of the node
        pub fn key_exists(&self, key: &str) -> bool {

            let mut child = &self.children;

            loop {
                match child {
                    &Some(ref v) => {
                        if v.node.key == key {
                            return true;
                        }

                        child = &v.next;
                    },
                    &None => {
                        break;
                    }
                }
            }

            false
        }
    }
}

#[cfg(test)]

mod tests {

    use rt;

    #[test]
    fn test_create() {

        let mut node = rt::RadixTrieNode::new("");

        assert_eq!(
            node.key_exists("first"),
            false,
            "The first key should not exist !",
        );

        node.insert("second");

        assert_eq!(
            node.key_exists("second"),
            true,
            "The second key should exist !",
        );

        node.insert("third");

        assert_eq!(
            node.key_exists("third"),
            true,
            "The third key should exist !",
        );
        assert_eq!(
            node.key_exists("second"),
            true,
            "The second key should exist !",
        );

        assert_eq!(
            node.key_exists("unknown"),
            false,
            "The unknown key should not exist !",
        );
    }
}
