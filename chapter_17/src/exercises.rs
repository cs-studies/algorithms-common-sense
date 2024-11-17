use crate::{Trie, TrieNode};

impl Trie {
    pub fn traverse(&self) {
        self.root.traverse();
    }

    pub fn autocorrect(&self, word: &str) -> Option<String> {
        let mut current = &self.root;
        let mut found = String::new();
        for ch in word.chars() {
            if let Some(child) = current.children.get(&ch) {
                found.push(ch);
                current = child;
            } else {
                let mut completions = Vec::new();
                current.collect_words(found.as_str(), &mut completions);
                if let Some(completion) = completions.first() {
                    return Some(completion.to_string());
                }
            }
        }
        None
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autocorrect() {
        let mut trie = Trie::new();
        trie.insert("cat");
        trie.insert("catnap");
        trie.insert("catnip");

        assert!(trie.autocorrect("cat").is_none());
        assert!(trie.autocorrect("catnap").is_none());

        let correct = trie.autocorrect("catnar").unwrap();
        let expected = String::from("catnap");
        assert_eq!(correct, expected);

        let correct = trie.autocorrect("catnixam").unwrap();
        let expected = String::from("catnip");
        assert_eq!(correct, expected);
    }
}
