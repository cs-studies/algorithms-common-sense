use std::collections::HashSet;

mod exercises;

fn main() {
    println!("\n*** Chapter 08 ***\n");

    let v1 = vec!["a", "b"];
    let v2 = vec!["c", "b", "f", "a"];
    println!("v1: {:?}", &v1);
    println!("v2: {:?}", &v2);
    println!("is_subset: {}\n", is_subset(&v1, &v2));

    let v1 = vec!["a", "b"];
    let v2 = vec!["c", "b", "f", "a"];
    println!("v1: {:?}", &v1);
    println!("v2: {:?}", &v2);
    println!("is_subset_hash: {}\n", is_subset_hash(&v1, &v2));

    //// Exercises
    println!("\n*** Exercises ***\n");

    let v1 = vec![1, 2, 3, 4, 5];
    println!("v1: {:?}", v1);
    let v2 = vec![0, 2, 4, 6, 8];
    println!("v2: {:?}", v2);
    println!("Intersection: {:?} \n", exercises::intersect(&v1, &v2));

    let v = vec!["a", "b", "c", "d", "c", "e", "f"];
    println!("v: {:?}", v);
    println!("Duplicate: {:?} \n", exercises::find_duplicate(&v));

    let s = "the quick brown box jumps over a lazy dog";
    println!("s: '{s}'");
    println!("Letter: {:?} \n", exercises::find_missing(s));

    let s = "minimum";
    println!("s: '{s}'");
    println!("Letter: {:?} \n", exercises::first_non_dup(s));
}

fn is_subset(a: &[&str], b: &[&str]) -> bool {
    #[rustfmt::skip]
    let (large, small) = if a.len() > b.len() {
        (a, b)
    } else {
        (b, a)
    };
    for s in small {
        let mut found = false;
        for l in large {
            if s.eq(l) {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }
    true
}

fn is_subset_hash(a: &[&str], b: &[&str]) -> bool {
    #[rustfmt::skip]
    let (large, small) = if a.len() > b.len() {
        (a, b)
    } else {
        (b, a)
    };

    let mut large_set = HashSet::new();
    for l in large {
        large_set.insert(l);
    }

    for s in small {
        if !large_set.contains(s) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subset() {
        assert!(is_subset(&[], &[]));
        assert!(is_subset(&["a"], &[]));
        assert!(is_subset(&[], &["a"]));
        assert!(is_subset(&["a"], &["a", "b"]));
        assert!(is_subset(&["b", "a"], &["c", "a", "b"]));
        assert!(!is_subset(&["a", "b"], &["x", "y", "a"]));
        assert!(!is_subset(&["x", "y", "a"], &["a", "b"]));
    }

    #[test]
    fn test_is_subset_hash() {
        assert!(is_subset_hash(&[], &[]));
        assert!(is_subset_hash(&["a"], &[]));
        assert!(is_subset_hash(&[], &["a"]));
        assert!(is_subset_hash(&["a"], &["a", "b"]));
        assert!(is_subset_hash(&["b", "a"], &["c", "a", "b"]));
        assert!(!is_subset_hash(&["a", "b"], &["x", "y", "a"]));
        assert!(!is_subset_hash(&["x", "y", "a"], &["a", "b"]));
    }
}
