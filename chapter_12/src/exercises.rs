use std::collections::HashMap;

// The corresponding  exercise isn't very well defined
// and the solution given in the book is arguable.
// The following solution shows an alternate approach,
// summing the numbers from left to right.
// This solution is recursive only because
// it's requested so by the exercise.
pub fn add_until_100(data: &[u8]) -> u8 {
    let last = match data.last() {
        Some(&n) => n,
        None => return 0,
    };
    let sum_first = add_until_100(&data[0..data.len() - 1]);
    let sum = sum_first.checked_add(last).unwrap_or(sum_first);
    if sum > 100 {
        sum_first
    } else {
        sum
    }
}

pub fn golomb(n: u16, memo: &mut HashMap<u16, u16>) -> u16 {
    if let Some(&val) = memo.get(&n) {
        return val;
    }
    let val = match n {
        0 => panic!("Golomb sequence starts with 1"),
        1 => 1,
        _ => 1 + golomb(n - golomb(golomb(n - 1, memo), memo), memo),
    };
    memo.insert(n, val);
    val
}

pub fn unique_paths(r: u8, c: u8, memo: &mut HashMap<(u8, u8), u16>) -> u16 {
    if let Some(&val) = memo.get(&(r, c)) {
        return val;
    }
    let val = match (r, c) {
        (0, _) | (_, 0) => panic!("rows and columns must be positive numbers"),
        (1, _) | (_, 1) => 1,
        _ => unique_paths(r - 1, c, memo)
            .checked_add(unique_paths(r, c - 1, memo))
            .expect("simple examples should not overflow memory"),
    };
    memo.insert((r, c), val);
    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_until_100() {
        assert_eq!(add_until_100(&Vec::new()), 0);
        assert_eq!(add_until_100(&vec![10, 20, 30, 40]), 100);
        assert_eq!(add_until_100(&vec![10, 20, 30, 41]), 60);
        assert_eq!(add_until_100(&vec![90, 10, 20, 30]), 100);
        assert_eq!(add_until_100(&vec![91, 10, 20, 30]), 91);
        assert_eq!(add_until_100(&vec![1, u8::MAX]), 1);
        assert_eq!(add_until_100(&vec![100, u8::MAX]), 100);
        assert_eq!(add_until_100(&vec![101, u8::MAX]), 0);
        assert_eq!(add_until_100(&vec![u8::MAX, 1]), 1);
        assert_eq!(add_until_100(&vec![u8::MAX, 100]), 100);
        assert_eq!(add_until_100(&vec![u8::MAX, 101]), 0);
    }

    #[test]
    fn test_golomb() {
        let mut hm = HashMap::new();
        assert_eq!(golomb(1, &mut hm), 1);
        assert_eq!(golomb(2, &mut hm), 2);
        assert_eq!(golomb(3, &mut hm), 2);
        assert_eq!(golomb(4, &mut hm), 3);
        assert_eq!(golomb(5, &mut hm), 3);
        assert_eq!(golomb(6, &mut hm), 4);
        assert_eq!(golomb(7, &mut hm), 4);
        assert_eq!(golomb(8, &mut hm), 4);
        assert_eq!(golomb(9, &mut hm), 5);
        assert_eq!(golomb(10, &mut hm), 5);
        assert_eq!(golomb(11, &mut hm), 5);
        assert_eq!(golomb(12, &mut hm), 6);
    }

    #[test]
    #[should_panic(expected = "Golomb sequence starts with 1")]
    fn test_golomb_panics() {
        golomb(0, &mut HashMap::new());
    }

    #[test]
    fn test_unique_paths() {
        let mut hm = HashMap::new();
        assert_eq!(unique_paths(1, 1, &mut hm), 1);
        assert_eq!(unique_paths(2, 1, &mut hm), 1);
        assert_eq!(unique_paths(1, 2, &mut hm), 1);
        assert_eq!(unique_paths(2, 2, &mut hm), 2);
        assert_eq!(unique_paths(3, 1, &mut hm), 1);
        assert_eq!(unique_paths(1, 3, &mut hm), 1);
        assert_eq!(unique_paths(3, 2, &mut hm), 3);
        assert_eq!(unique_paths(2, 3, &mut hm), 3);
    }

    #[test]
    #[should_panic]
    fn test_unique_paths_00() {
        unique_paths(0, 0, &mut HashMap::new());
    }

    #[test]
    #[should_panic]
    fn test_unique_paths_01() {
        unique_paths(0, 1, &mut HashMap::new());
    }

    #[test]
    #[should_panic]
    fn test_unique_paths_10() {
        unique_paths(1, 0, &mut HashMap::new());
    }
}
