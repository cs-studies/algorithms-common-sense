use std::collections::HashMap;

pub fn group_array(data: &[char]) -> Vec<char> {
    let mut hm = HashMap::<char, usize>::new();

    for &c in data {
        hm.entry(c).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut result = Vec::with_capacity(data.len());
    for (&c, &count) in hm.iter() {
        for _ in 0..count {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_array() {
        assert_eq!(group_array(&[]), vec![]);
        assert_eq!(group_array(&['a']), vec!['a']);
        assert_eq!(group_array(&['a'; 5]), vec!['a'; 5]);
    }
}
