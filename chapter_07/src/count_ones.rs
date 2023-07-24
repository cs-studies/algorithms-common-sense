pub fn count(data: Vec<Vec<u8>>) -> usize {
    let mut count = 0;
    for inner in data.iter() {
        for n in inner.iter() {
            if *n == 1 {
                count += 1;
            }
        }
    }
    count
}

//// Rust Extras
#[allow(dead_code)]
fn count_extra(data: Vec<Vec<u8>>) -> usize {
    data.into_iter().flatten().filter(|x| *x == 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        assert_eq!(count(vec![vec![]]), 0);
        assert_eq!(count(vec![vec![0, 0], vec![0, 0]]), 0);
        assert_eq!(count(vec![vec![0, 1], vec![0, 0]]), 1);
        assert_eq!(count(vec![vec![0, 1], vec![1, 1]]), 3);
    }

    #[test]
    fn test_count_extra() {
        assert_eq!(count_extra(vec![vec![]]), 0);
        assert_eq!(count_extra(vec![vec![0, 0], vec![0, 0]]), 0);
        assert_eq!(count_extra(vec![vec![0, 1], vec![0, 0]]), 1);
        assert_eq!(count_extra(vec![vec![0, 1], vec![1, 1]]), 3);
    }
}
