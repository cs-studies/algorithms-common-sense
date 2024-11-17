use std::collections::HashMap;

fn main() {
    println!("\n*** Chapter 17 ***\n");

    let mut trie = Trie::new();

    trie.insert("ace");
    trie.insert("act");
    trie.insert("bad");
    trie.insert("bake");
    trie.insert("bat");
    trie.insert("batter");
    trie.insert("cab");
    trie.insert("cat");
    trie.insert("catnap");
    trie.insert("catnip");

    dbg!(&trie);

    dbg!(trie.search("bat"));
    dbg!(trie.search("batman"));

    dbg!(trie.collect_words());

    dbg!(trie.autocomplete("cat"));
}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
}

#[derive(Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;

        for ch in word.chars() {
            current = current.children.entry(ch).or_insert(TrieNode::new());
        }
        current.children.insert('*', TrieNode::new());
    }

    fn search(&self, word: &str) -> Option<&TrieNode> {
        let mut current = &self.root;

        for ch in word.chars() {
            if let Some(child) = current.children.get(&ch) {
                current = child;
            } else {
                return None;
            }
        }

        Some(current)
    }

    fn collect_words(&self) -> Vec<String> {
        let mut words = Vec::new();
        self.root.collect_words("", &mut words);
        words
    }

    fn autocomplete(&self, prefix: &str) -> Vec<String> {
        let mut words = Vec::new();
        if let Some(node) = self.search(prefix) {
            node.collect_words("", &mut words);
        }
        words
    }
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
        }
    }

    fn collect_words(&self, word: &str, words: &mut Vec<String>) {
        for (key, child) in self.children.iter() {
            if *key == '*' {
                words.push(word.to_string());
            } else {
                let mut new_word = word.to_string();
                new_word.push(*key);
                child.collect_words(&new_word, words);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut trie = Trie::new();
        assert!(trie.root.children.is_empty());

        trie.insert("bat");
        assert_eq!(trie.root.children.len(), 1);

        trie.insert("batter");
        assert_eq!(trie.root.children.len(), 1);

        trie.insert("ace");
        assert_eq!(trie.root.children.len(), 2);
    }

    #[test]
    fn test_search() {
        let mut trie = Trie::new();

        assert!(trie.search("cat").is_none());

        trie.insert("cat");
        assert!(trie.search("cat").is_some());
        assert!(trie.search("bat").is_none());

        trie.insert("batter");
        assert!(trie.search("bat").is_some());
        assert!(trie.search("batter").is_some());
    }

    #[test]
    fn test_collect_words() {
        let mut trie = Trie::new();
        let words = ["bake", "bat", "batter"];
        for word in words.iter() {
            trie.insert(word);
        }
        let collected = trie.collect_words();
        assert_eq!(collected.len(), words.len());
        for word in words.iter() {
            assert!(collected.contains(&word.to_string()));
        }
    }

    #[test]
    fn test_autocomplete() {
        let mut trie = Trie::new();
        let words = ["cat", "cater", "bake", "bat", "batter"];
        for word in words.iter() {
            trie.insert(word);
        }
        let completions = trie.autocomplete("bat");
        let expected = vec!["", "ter"];
        assert_eq!(completions.len(), expected.len());
        for word in expected.iter() {
            assert!(completions.contains(&word.to_string()));
        }
    }
}
