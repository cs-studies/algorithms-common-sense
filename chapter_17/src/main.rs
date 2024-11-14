#![allow(unused_variables, dead_code, unused_imports)]

use std::collections::HashMap;

fn main() {
    println!("\n*** Chapter 17 ***\n");

    let trie = Trie::<&str>::new();
    dbg!(&trie);
}

#[derive(Debug)]
struct Trie<T> {
    root: TrieNode<T>
}

#[derive(Debug)]
struct TrieNode<T> {
    children: HashMap<T, TrieNode<T>>,
}

impl<T> Trie<T> {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }
}

impl<T> TrieNode<T> {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
        }
    }
}
