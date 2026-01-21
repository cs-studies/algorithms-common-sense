use std::collections::HashMap;

pub fn are_anagrams_1(s1: &str, s2: &str) -> bool {
    let mut vec2: Vec<char> = s2.chars().collect();
    for c1 in s1.chars() {
        if vec2.is_empty() {
            return false;
        }
        for j in 0..vec2.len() {
            if c1 == vec2[j] {
                vec2.remove(j);
                break;
            }
        }
    }
    vec2.is_empty()
}

pub fn are_anagrams_2(s1: &str, s2: &str) -> bool {
    let mut hm1 = HashMap::<char, usize>::new();
    let mut hm2 = HashMap::<char, usize>::new();

    for c1 in s1.chars() {
        hm1.entry(c1).and_modify(|c| *c += 1).or_insert(1);
    }
    for c2 in s2.chars() {
        hm2.entry(c2).and_modify(|c| *c += 1).or_insert(1);
    }
    // println!("hm1: {:?}", hm1);
    // println!("hm2: {:?}", hm2);
    hm1 == hm2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_anagrams_1() {
        assert!(are_anagrams_1("", ""));
        assert!(!are_anagrams_1("a", ""));
        assert!(!are_anagrams_1("", "a"));
        assert!(are_anagrams_1("a", "a"));
        assert!(are_anagrams_1("ab", "ab"));
        assert!(are_anagrams_1("ab", "ba"));
        assert!(are_anagrams_1("abc", "cab"));
        assert!(are_anagrams_1("abc", "cba"));
        assert!(!are_anagrams_1("abz", "cab"));
    }

    #[test]
    fn test_are_anagrams_2() {
        assert!(are_anagrams_2("", ""));
        assert!(!are_anagrams_2("a", ""));
        assert!(!are_anagrams_2("", "a"));
        assert!(are_anagrams_2("a", "a"));
        assert!(are_anagrams_2("ab", "ab"));
        assert!(are_anagrams_2("ab", "ba"));
        assert!(are_anagrams_2("abc", "cab"));
        assert!(are_anagrams_2("abc", "cba"));
        assert!(!are_anagrams_2("abz", "cab"));
    }
}
