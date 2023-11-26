use std::collections::HashSet;

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
}
