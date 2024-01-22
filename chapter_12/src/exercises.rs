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
}
