pub fn get(data: &[i32]) -> Vec<i32> {
    let data_len = data.len();
    if data_len < 2 {
        return vec![];
    }
    let mut products: Vec<i32> = vec![];
    for i in 0..data_len {
        for j in (i + 1)..data_len {
            let product = data[i].checked_mul(data[j]).unwrap();
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
        assert_eq!(Vec::<i32>::new(), get(&[]));
        assert_eq!(Vec::<i32>::new(), get(&[2]));
        assert_eq!(vec![2], get(&[1, 2]));
        assert_eq!(vec![2, 3, 4, 6, 8, 12], get(&[1, 2, 3, 4]));
    }
}
