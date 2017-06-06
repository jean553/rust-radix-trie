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

        pub fn new(key: &str) -> RadixTrieNode {

            RadixTrieNode {
                key: key.to_string(),
                children: None,
            }
        }

        pub fn insert(&mut self, key: &str) {

            self.children = Some(Box::new(RadixTrieNodeChild {
                next: None,
                node: Box::new(RadixTrieNode {
                    key: key.to_string(),
                    children: None,
                })
            }));
        }

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
