use std::collections::{HashMap, HashSet};

pub fn intersect(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut hs = HashSet::new();
    for i in a {
        hs.insert(i);
    }
    for j in b {
        if hs.contains(j) {
            result.push(*j);
        }
    }
    result
}

pub fn find_duplicate(data: &[&str]) -> Option<String> {
    if data.len() < 2 {
        return None;
    }
    let mut hs = HashSet::new();
    for s in data {
        if !hs.insert(s) {
            return Some(s.to_string());
        }
    }
    None
}

pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn find_missing(line: &str) -> Option<char> {
    if line.len() < 25 {
        return None;
    }
    let mut hs = HashSet::new();
    for c in line.chars() {
        hs.insert(c);
    }
    #[allow(clippy::manual_find)]
    for c in ALPHABET.chars() {
        if !hs.contains(&c) {
            return Some(c);
        }
    }
    None
}

pub fn first_non_dup(line: &str) -> Option<char> {
    let mut hm = HashMap::new();
    for c in line.chars() {
        hm.entry(c).and_modify(|count| *count += 1).or_insert(1);
    }
    for c in line.chars() {
        match hm.get(&c) {
            Some(v) if *v == 1 => return Some(c),
            _ => (),
        };
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        let v_empty = Vec::<i32>::new();
        assert_eq!(intersect(&[], &[]), v_empty);
        assert_eq!(intersect(&[1], &[]), v_empty);
        assert_eq!(intersect(&[], &[1]), v_empty);
        let x = intersect(&[3, 1, 4, 2], &[4, 5, 3, 6]);
        assert!(x.eq(&vec![3, 4]) || x.eq(&vec![4, 3]));
    }

    #[test]
    fn test_find_duplicate() {
        assert_eq!(find_duplicate(&[]), None);
        assert_eq!(find_duplicate(&["cat"]), None);
        assert_eq!(find_duplicate(&["cat", "dog"]), None);
        assert_eq!(
            find_duplicate(&["cat", "dog", "cat"]),
            Some("cat".to_string())
        );
        assert_eq!(
            find_duplicate(&["cat", "dog", "fish", "dog"]),
            Some("dog".to_string())
        );
    }

    #[test]
    fn test_find_missing() {
        assert_eq!(find_missing(""), None);
        assert_eq!(find_missing(&ALPHABET[0..(ALPHABET.len() - 1)]), Some('z'));
        assert_eq!(
            find_missing("the quick brown box jumps over a lazy dog"),
            Some('f')
        );
    }

    #[test]
    fn test_first_non_dup() {
        assert_eq!(first_non_dup(""), None);
        assert_eq!(first_non_dup("mama"), None);
        assert_eq!(first_non_dup("minimum"), Some('n'));
    }
}
