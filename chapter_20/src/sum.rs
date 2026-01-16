use std::collections::HashSet;

pub fn two_sum_1(data: &[i32]) -> bool {
    let len = data.len();
    for i in 0..len {
        for j in 0..len {
            if i != j && data[i] + data[j] == 10 {
                return true;
            }
        }
    }
    false
}

pub fn two_sum_2(data: &[i32]) -> bool {
    let mut seen = HashSet::new();

    for &val in data {
        if seen.contains(&(10 - val)) {
            return true;
        }
        seen.insert(val);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_1() {
        assert!(!two_sum_1(&[]));
        assert!(!two_sum_1(&[1]));
        assert!(!two_sum_1(&[1, 5]));
        assert!(two_sum_1(&[1, 5, 9]));
        assert!(two_sum_1(&[1, 6, 8, 7, 4]));
    }

    #[test]
    fn test_two_sum_2() {
        assert!(!two_sum_2(&[]));
        assert!(!two_sum_2(&[1]));
        assert!(!two_sum_2(&[1, 5]));
        assert!(two_sum_2(&[1, 5, 9]));
        assert!(two_sum_2(&[1, 6, 8, 7, 4]));
    }
}
