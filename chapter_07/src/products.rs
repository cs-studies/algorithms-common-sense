pub fn get(data: &[i32]) -> Vec<i32> {
    let data_len = data.len();
    if data_len < 2 {
        return vec![];
    }
    let mut products: Vec<i32> = vec![];
    for i in 0..data_len {
        for j in (i + 1)..data_len {
            let product = data[i]
                .checked_mul(data[j])
                .expect("simple examples should not overflow memory");
            products.push(product);
        }
    }
    products
}

//// Rust Extras
#[allow(dead_code)]
fn get_extra(data: &[i32]) -> Vec<i32> {
    data.iter()
        .enumerate()
        // .inspect(|(_, n)| println!("{n}"))
        .flat_map(|(i, n)| {
            data.iter()
                .skip(i + 1)
                // .inspect(|m| print!("  * {m}"))
                .map(|&m| n
                     .checked_mul(m)
                     .expect("simple examples should not overflow memory"))
            // .inspect(|m| println!(" = {m}"))
        })
        .collect()
}

pub fn get_multi(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut products: Vec<i32> = vec![];
    for i in a {
        for j in b {
            let product = i
                .checked_mul(*j)
                .expect("simple examples should not overflow memory");
            products.push(product);
        }
    }
    products
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        assert_eq!(get(&[]), Vec::<i32>::new());
        assert_eq!(get(&[2]), Vec::<i32>::new());
        assert_eq!(get(&[1, 2]), vec![2]);
        assert_eq!(get(&[1, 2, 3, 4]), vec![2, 3, 4, 6, 8, 12]);
    }

    #[test]
    #[should_panic]
    fn test_get_panics() {
        get(&[1, 2, 3, i32::MAX]);
    }

    #[test]
    fn test_get_extra() {
        assert_eq!(get_extra(&[]), Vec::<i32>::new());
        assert_eq!(get_extra(&[2]), Vec::<i32>::new());
        assert_eq!(get_extra(&[1, 2]), vec![2]);
        assert_eq!(get_extra(&[1, 2, 3, 4]), vec![2, 3, 4, 6, 8, 12]);
    }

    #[test]
    #[should_panic]
    fn test_get_extra_panics() {
        get_extra(&[1, 2, 3, i32::MAX]);
    }

    #[test]
    fn test_get_multi() {
        assert_eq!(get_multi(&[], &[]), Vec::<i32>::new());
        assert_eq!(get_multi(&[2], &[]), Vec::<i32>::new());
        assert_eq!(get_multi(&[], &[2]), Vec::<i32>::new());
        assert_eq!(get_multi(&[1, 2], &[1]), vec![1, 2]);
        assert_eq!(get_multi(&[1, 2], &[3, 4]), vec![3, 4, 6, 8]);
        assert_eq!(
            get_multi(&[1, 2], &[10, 100, 1000]),
            vec![10, 100, 1000, 20, 200, 2000]
        );
    }

    #[test]
    #[should_panic]
    fn test_get_multi_panics() {
        get_multi(&[1, 2, 3, i32::MAX], &[2]);
    }
}
