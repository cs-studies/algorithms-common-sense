fn main() {
    println!("\n*** Chapter 11 ***\n");

    let mut v = [1, 2, 3, 4, 5];
    double(&mut v, 0);
    println!("doubled: {:?}", v);
}

fn double(data: &mut [i32], idx: usize) {
    if idx >= data.len() {
        return;
    }
    data[idx] *= 2;
    double(data, idx + 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double() {
        let mut v = Vec::<i32>::new();
        double(&mut v, 0);
        assert_eq!(v, Vec::<i32>::new());

        let mut v = vec![1, 2, 3];
        double(&mut v, 10);
        assert_eq!(v, vec![1, 2, 3]);

        let mut v = vec![0, 1, 2, 3];
        double(&mut v, 0);
        assert_eq!(v, vec![0, 2, 4, 6]);
    }
}
