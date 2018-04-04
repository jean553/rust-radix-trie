[![Build Status](https://travis-ci.org/jean553/rust-radix-trie.svg?branch=master)](https://travis-ci.org/jean553/rust-radix-trie)

# rust-radix-trie

## Usage example

```rust
let mut trie = RadixTrie::new("salt");
trie.insert("same");

trie.exists("sam"); // true
trie.exists("salted"); // false
```

## Implemented features

 * Insertion with node key modification
 * Insertion with new nodes creation
 * Insertion with children nodes move if necessary
 * Recursive browsing until the appropriate insertion node is found
 * Check if a key exists

## Development

### Create the container

```sh
vagrant up
```

### Connect to the container

```sh
vagrant ssh
```

### Unit tests

```sh
cargo test
```
