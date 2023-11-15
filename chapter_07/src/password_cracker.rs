// const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const ALPHABET: &str = "abcd";

// This iterative function runs for around 50 seconds with the full alphabet,
// generating 6-letter combinations on Apple M2 Max with 32 Gb of memory.
// 7-letter combinations generation fails due to a lack of memory.
pub fn combinations(len: u8) -> Vec<String> {
    // println!("ALPHABET length is {}", ALPHABET.len());
    let mut result = vec![String::new()];

    for _ in 0..len {
        let mut new_result = vec![];
        for c in ALPHABET.chars() {
            for r in &result {
                new_result.push(format!("{c}{r}"));
            }
        }
        result = new_result;
    }
    result
}

// The recursive function runs for around 280 seconds with the full alphabet,
// generating 6-letter combinations on Apple M2 Max with 32 Gb of memory.
// 7-letter combinations generation fails due to lack of memory.
pub fn combinations_recur(len: u8) -> Vec<String> {
    if len == 0 {
        return vec![String::new()];
    }
    ALPHABET
        .chars()
        .flat_map(|c| {
            combinations_recur(len - 1)
                .into_iter()
                .map(move |r| format!("{c}{r}"))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_combinations() {
        assert_eq!(combinations(0), vec![String::new()]);
        assert_eq!(combinations(1), vec!["a", "b", "c", "d"]);

        #[rustfmt::skip]
        assert_eq!(
            combinations(2),
            vec!["aa", "ab", "ac", "ad",
                 "ba", "bb", "bc", "bd",
                 "ca", "cb", "cc", "cd",
                 "da", "db", "dc", "dd"]
        );
    }

    #[test]
    fn test_combinations_recur() {
        assert_eq!(combinations_recur(0), vec![String::new()]);
        assert_eq!(combinations_recur(1), vec!["a", "b", "c", "d"]);

        #[rustfmt::skip]
        assert_eq!(
            combinations_recur(2),
            vec!["aa", "ab", "ac", "ad",
                 "ba", "bb", "bc", "bd",
                 "ca", "cb", "cc", "cd",
                 "da", "db", "dc", "dd"]
        );
    }
}
