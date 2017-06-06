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
    }
}

#[cfg(test)]
mod tests {

    use rt;

    #[test]
    fn test_create() {

        let mut node = rt::RadixTrieNode::new("");
        node.insert("hello");
    }
}
