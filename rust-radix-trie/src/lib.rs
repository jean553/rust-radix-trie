mod rt {

    use std::ptr;

    struct RadixTrieNodeChild {
        next: *const RadixTrieNodeChild,
        node: *const RadixTrieNode,
    }

    pub struct RadixTrieNode {
        key: String,
        children: *const RadixTrieNodeChild,
    }

    impl RadixTrieNode {

        pub fn new(key: &str) -> RadixTrieNode {
            RadixTrieNode {
                key: key.to_string(),
                children: ptr::null(),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use rt;

    #[test]
    fn test_create() {

        let node = rt::RadixTrieNode::new("");
    }
}
