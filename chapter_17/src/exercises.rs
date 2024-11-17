use crate::{Trie, TrieNode};

impl Trie {
    pub fn traverse(&self) {
        self.root.traverse();
    }
}

impl TrieNode {
    fn traverse(&self) {
        for (key, child) in self.children.iter() {
            println!("{key}");
            if *key != '*' {
                child.traverse();
            }
        }
    }
}
