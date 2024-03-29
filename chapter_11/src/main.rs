mod exercises;

fn main() {
    println!("\n*** Chapter 11 ***\n");

    let mut v = [1, 2, 3, 4, 5];
    double(&mut v, 0);
    println!("doubled: {:?}\n", v);

    println!("5! = {}", factorial(5));
    println!("5! = {}\n", factorial_params(5, 1, 1));

    let v = vec![1, 2, 3, 4, 5];
    println!("sum of {:?} is {}\n", v, sum(&v));

    let s = "abcde";
    println!("reverse of {} is {}\n", s, reverse(s));

    let s = "axbxcxd";
    println!("count of 'x' in {} is {}\n", s, count_x(s));

    let num = 11;
    println!("number of paths for {} stairs is {}\n", num, stairs(num));

    let s = "abc";
    println!("anagrams of {:?} are {:?}\n", s, anagrams(s));

    //// Exercises
    println!("\n*** Exercises ***\n");

    let a = ["ab", "c", "def", "ghij"];
    println!("chars count in {:?} is {}", a, exercises::count_chars(&a));

    let nums = [100, 99, 98, 97, -6, -3, 0];
    println!(
        "even numbers in {:?} are {:?}",
        nums,
        exercises::find_evens(&nums)
    );

    println!(
        "the {}-th triangular number is {}",
        7,
        exercises::triangular_number(7)
    );

    let s = "abcdefghijklmnopqrstuvwxyz";
    println!("'x' is at {}", exercises::index_x(s));

    println!(
        "number of unique paths in 3x7 grid is {}",
        exercises::unique_paths(3, 7)
    );
}

fn double(data: &mut [i32], idx: usize) {
    if idx >= data.len() {
        return;
    }
    data[idx] *= 2;
    double(data, idx + 1);
}

fn factorial(num: u8) -> u8 {
    let mut product = 1;
    (2..=num).for_each(|n| product *= n);
    product
}

fn factorial_params(num: u8, i: u8, product: u8) -> u8 {
    if i > num {
        product
    } else {
        factorial_params(num, i + 1, product * i)
    }
}

fn sum(data: &[i32]) -> i32 {
    if data.is_empty() {
        0
    } else {
        data[0] + sum(&data[1..data.len()])
    }
}

fn reverse(s: &str) -> String {
    if s.is_empty() {
        String::new()
    } else {
        let (first, rest) = s.split_at(1);
        let mut reversed = reverse(rest);
        reversed.push_str(first);
        reversed
    }
}

fn count_x(s: &str) -> usize {
    if s.is_empty() {
        0
    } else {
        let (first, rest) = s.split_at(1);
        count_x(rest) + first.eq("x") as usize
    }
}

fn stairs(num: u8) -> usize {
    match num {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 4,
        _ => stairs(num - 1) + stairs(num - 2) + stairs(num - 3),
    }
}

fn anagrams(s: &str) -> Vec<String> {
    if s.is_empty() {
        return vec![String::new()];
    }
    let mut collection = Vec::new();
    for a in anagrams(&s[1..]) {
        for i in 0..=a.len() {
            let mut anagram = a.clone();
            anagram.insert_str(i, &s[0..1]);
            collection.push(anagram);
        }
    }
    collection
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

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_factorial_params() {
        assert_eq!(factorial_params(0, 1, 1), 1);
        assert_eq!(factorial_params(1, 1, 1), 1);
        assert_eq!(factorial_params(2, 1, 1), 2);
        assert_eq!(factorial_params(3, 1, 1), 6);
        assert_eq!(factorial_params(4, 1, 1), 24);
        assert_eq!(factorial_params(5, 1, 1), 120);
        assert_eq!(factorial_params(5, 10, 100), 100);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(&[]), 0);
        assert_eq!(sum(&[1]), 1);
        assert_eq!(sum(&[1, 2]), 3);
        assert_eq!(sum(&[1, 2, 3]), 6);
    }

    #[test]
    #[should_panic]
    fn test_sum_panics() {
        sum(&[i32::MAX, 1]);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(""), "".to_string());
        assert_eq!(reverse("x"), "x".to_string());
        assert_eq!(reverse("xy"), "yx".to_string());
        assert_eq!(reverse("xyz"), "zyx".to_string());
    }

    #[test]
    fn test_count_x() {
        assert_eq!(count_x(""), 0);
        assert_eq!(count_x("x"), 1);
        assert_eq!(count_x("xy"), 1);
        assert_eq!(count_x("yx"), 1);
        assert_eq!(count_x("yxy"), 1);
        assert_eq!(count_x("xyx"), 2);
        assert_eq!(count_x("yxx"), 2);
    }

    #[test]
    fn test_stairs() {
        assert_eq!(stairs(0), 0);
        assert_eq!(stairs(1), 1);
        assert_eq!(stairs(2), 2);
        assert_eq!(stairs(3), 4);
        assert_eq!(stairs(4), 7);
        assert_eq!(stairs(5), 13);
    }

    #[test]
    fn test_anagrams() {
        assert_eq!(anagrams(""), vec![""]);
        assert_eq!(anagrams("a"), vec!["a"]);
        assert_eq!(anagrams("ab"), vec!["ab", "ba"]);
        assert_eq!(
            anagrams("abc"),
            vec!["abc", "bac", "bca", "acb", "cab", "cba"]
        );
        assert_eq!(anagrams("abcdef").len(), 6 * 5 * 4 * 3 * 2)
    }
}
