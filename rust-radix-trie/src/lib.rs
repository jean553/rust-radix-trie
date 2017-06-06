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
        /// NOTE: This fonction is partialy implemented;
        /// by now, it simply gives an unique child to the node
        pub fn insert(&mut self, key: &str) {

            self.children = Some(Box::new(RadixTrieNodeChild {
                next: None,
                node: Box::new(RadixTrieNode {
                    key: key.to_string(),
                    children: None,
                })
            }));
        }

        /// Checks if a key exists inside the radix trie
        ///
        /// NOTE: This function is partialy implemented;
        /// by now, it simply looks at the first child of the node
        pub fn key_exists(&self, key: &str) -> bool {

            if self.children.is_none() {
                return false;
            }

            let current = &self.children;

            loop {

                match current {
                    &Some(ref child) => {

                        if &child.node.key == key {
                            return true;
                        }
                    }
                    &None => {
                        return false;
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
            "The first key should exist !",
        );

        node.insert("second");

        assert_eq!(
            node.key_exists("second"),
            true,
            "The second key should exist !",
        );
    }
}
