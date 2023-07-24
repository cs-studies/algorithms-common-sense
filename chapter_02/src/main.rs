use std::cmp::Ordering;
use std::collections::BTreeSet;

fn main() {
    println!("\n*** Chapter 02 ***\n");

    let mut a = [17, 3, 80, 202, 42];
    println!("Array: {:?}\n", a);

    for search_for in [42, 22] {
        if let Some(index) = linear_search(&mut a, search_for) {
            println!("linear_search: found {search_for} at position {index}\n");
        } else {
            println!("linear_search: did not find {search_for}\n");
        }
    }

    for search_for in [42, 22] {
        if let Some(index) = binary_search(&mut a, search_for) {
            println!("binary_search: found {search_for} at position {index}\n");
        } else {
            println!("binary_search: did not find {search_for}\n");
        }
    }

    //// Rust Extras

    for search_for in [42, 22] {
        if let Ok(index) = a.binary_search(&search_for) {
            println!("slice::binary_search found {search_for} at {index}");
        } else {
            println!("slice::binary_search did not find {search_for}");
        }
    }

    // Looking into "ordered arrays" topic,
    // consider BTreeSet: an ordered set based on a B-Tree.
    let s = BTreeSet::from([150, 150, 17, 56, 12, 8, 33, 47, 42, 1, 5]);
    println!("\nBTreeSet: {:?}\n", s);
}

pub fn linear_search(data: &mut [u8], search_for: u8) -> Option<usize> {
    data.sort();
    println!("sorted: {:?}", data);
    println!("search_for: {:?}", search_for);
    for (i, &value) in data.iter().enumerate() {
        println!("value: {:?}", &value);
        match search_for.cmp(&value) {
            Ordering::Equal => return Some(i),
            Ordering::Less => break,
            _ => {}
        }
    }
    None
}

pub fn binary_search(data: &mut [u8], search_for: u8) -> Option<usize> {
    data.sort();
    println!("sorted: {:?}", data);
    println!("search_for: {:?}", search_for);
    let mut lower_bound = 0;
    let mut upper_bound = data.len().checked_sub(1)?;
    while lower_bound <= upper_bound {
        println!("lower_bound: {}", lower_bound);
        println!("upper_bound: {}", upper_bound);
        let midpoint = (lower_bound + upper_bound) / 2;
        println!("midpoint: {}", midpoint);
        let value_at_midpoint = data[midpoint];
        println!("value_at_midpoint: {:?}", value_at_midpoint);
        match search_for.cmp(&value_at_midpoint) {
            Ordering::Equal => return Some(midpoint),
            Ordering::Less => upper_bound = midpoint.checked_sub(1)?,
            Ordering::Greater => lower_bound = midpoint.checked_add(1)?,
        };
    }
    None
}

pub fn linear_search_s(data: &BTreeSet<u8>, search_for: u8) -> Option<usize> {
    println!("search_for: {:?}", search_for);
    for (i, &value) in data.iter().enumerate() {
        println!("value: {:?}", &value);
        match search_for.cmp(&value) {
            Ordering::Equal => return Some(i),
            Ordering::Less => break,
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        assert_eq!(linear_search(&mut [], 2), None);

        let mut a = [10, 30, 3, 42, 3];
        assert_eq!(linear_search(&mut a, 2), None);
        assert_eq!(linear_search(&mut a, 10), Some(2));
    }

    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search(&mut [], 2), None);

        let mut a = [10, 30, 3, 42, 3];
        assert_eq!(binary_search(&mut a, 2), None);
        assert_eq!(binary_search(&mut a, 10), Some(2));
    }
}
