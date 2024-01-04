fn main() {
    println!("\n*** Chapter 12 ***\n");

    let v = vec![1, 2, 3, 4];
    println!("max in {:?} is {}", v, max(&v));
}

fn max(data: &[i32]) -> i32 {
    if data.is_empty() {
        panic!("pass non-empty data")
    }
    if data.len() == 1 {
        return data[0];
    }
    let rest_max = max(&data[1..]);
    if data[0] > rest_max {
        return data[0];
    }
    rest_max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        assert_eq!(max(&[1]), 1);
        assert_eq!(max(&[1, 2]), 2);
        assert_eq!(max(&[2, 1]), 2);
        assert_eq!(max(&[2, 1, 3]), 3);
        assert_eq!(max(&[2, 3, 1]), 3);
        assert_eq!(max(&[3, 2, 1]), 3);
    }

    #[test]
    #[should_panic]
    fn test_max_panics() {
        max(&[]);
    }
}
