pub fn build(data: &[char]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut word;
    for c in data {
        for d in data {
            if c != d {
                word = String::from(*c);
                word.push(*d);
                result.push(word);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_words() {
        assert_eq!(Vec::<String>::new(), build(&[]));
        assert_eq!(
            vec!["ab", "ac", "ba", "bc", "ca", "cb"],
            build(&['a', 'b', 'c'])
        );
    }
}
