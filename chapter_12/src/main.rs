use std::collections::HashMap;

mod exercises;

fn main() {
    println!("\n*** Chapter 12 ***\n");

    let v = vec![1, 2, 3, 4];
    println!("max in {:?} is {}", v, max(&v));

    println!(
        "fib: the 10th Fibonacci number is {}",
        fib(10, &mut HashMap::new())
    );

    println!("fib_iter: the 10th Fibonacci number is {}", fib_iter(10));

    //// Exercises
    println!("\n*** Exercises ***\n");

    let nums = [100, u8::MAX];
    println!(
        "sum of {:?} is {:?}\n",
        nums,
        exercises::add_until_100(&nums)
    );

    println!(
        "12 appears in the Golomb sequence {} times",
        exercises::golomb(12, &mut HashMap::new())
    );
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

fn fib(n: u8, memo: &mut HashMap<u8, u128>) -> u128 {
    if let Some(&val) = memo.get(&n) {
        return val;
    }

    let val = match n {
        0 | 1 => n as u128,
        _ => fib(n - 2, memo)
            .checked_add(fib(n - 1, memo))
            .expect("simple examples should not overflow memory"),
    };

    memo.insert(n, val);
    val
}

fn fib_iter(n: u8) -> u128 {
    if n == 0 {
        return 0;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 1..n {
        let tmp: u128 = a;
        a = b;
        b = tmp
            .checked_add(a)
            .expect("simple examples should not overflow memory");
    }

    b
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

    #[test]
    fn test_fib() {
        let mut hm = HashMap::new();
        assert_eq!(fib(0, &mut hm), 0);
        assert_eq!(fib(1, &mut hm), 1);
        assert_eq!(fib(2, &mut hm), 1);
        assert_eq!(fib(3, &mut hm), 2);
        assert_eq!(fib(4, &mut hm), 3);
        assert_eq!(fib(5, &mut hm), 5);
        assert_eq!(fib(6, &mut hm), 8);
        assert_eq!(fib(7, &mut hm), 13);
        assert_eq!(fib(8, &mut hm), 21);
        assert_eq!(fib(9, &mut hm), 34);
        assert_eq!(fib(10, &mut hm), 55);
    }

    #[test]
    #[should_panic]
    fn test_fib_panics() {
        fib(u8::MAX, &mut HashMap::new());
    }

    #[test]
    fn test_fib_iter() {
        assert_eq!(fib_iter(0), 0);
        assert_eq!(fib_iter(1), 1);
        assert_eq!(fib_iter(2), 1);
        assert_eq!(fib_iter(3), 2);
        assert_eq!(fib_iter(4), 3);
        assert_eq!(fib_iter(5), 5);
        assert_eq!(fib_iter(6), 8);
        assert_eq!(fib_iter(7), 13);
        assert_eq!(fib_iter(8), 21);
        assert_eq!(fib_iter(9), 34);
        assert_eq!(fib_iter(10), 55);
    }

    #[test]
    #[should_panic]
    fn test_fib_iter_panics() {
        fib_iter(u8::MAX);
    }
}
