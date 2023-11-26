use std::collections::HashSet;

pub fn intersect(a: &[i32], b: &[i32]) -> Vec<i32> {
    #[rustfmt::skip]
    let (large, small) = if a.len() > b.len() {
        (a, b)
    } else {
        (b, a)
    };
    let mut hs = HashSet::new();
    for l in large {
        hs.insert(l);
    }
    let mut result = Vec::new();
    for s in small {
        if hs.contains(s) {
            result.push(*s);
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
        assert_eq!(x, vec![3, 4]);
    }
}
