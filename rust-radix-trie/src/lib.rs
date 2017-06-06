struct RadixTrieNodeChild {
    next: *const RadixTrieNodeChild,
    node: *const RadixTrieNode,
}

struct RadixTrieNode {
    key: String,
    children: *const RadixTrieNodeChild,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
