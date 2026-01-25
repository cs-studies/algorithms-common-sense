use std::collections::HashSet;

pub fn longest_sequence_len(numbers: &[i32]) -> usize {
    let mut hs = HashSet::<i32>::new();

    for &n in numbers {
        hs.insert(n);
    }

    let mut len = 0;
    for &n in numbers {
        if !hs.contains(&(n - 1)) {
            let mut count = 1;
            let mut current = n;
            while hs.contains(&(current + 1)) {
                current += 1;
                count += 1;
            }
            if count > len {
                len = count;
            }
        }
    }

    len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_sequence_2() {
        assert_eq!(longest_sequence_len(&[]), 0);
        assert_eq!(longest_sequence_len(&[1]), 1);
        assert_eq!(longest_sequence_len(&[1, 2]), 2);
        assert_eq!(longest_sequence_len(&[2, 1]), 2);
        assert_eq!(longest_sequence_len(&[1, 3]), 1);
        assert_eq!(longest_sequence_len(&[3, 1]), 1);
        assert_eq!(longest_sequence_len(&[3, 1, 2]), 3);
        assert_eq!(longest_sequence_len(&[5, 3, 8, 2, 4, 7]), 4);
        assert_eq!(longest_sequence_len(&[5, 3, 8, 2, 4, 6, 7]), 7);
    }
}
