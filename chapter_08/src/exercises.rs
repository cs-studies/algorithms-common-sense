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
}
