fn main() {
    println!("\n*** Chapter 07 ***\n");

    let v = vec![4, 2, 4, 1, 3];
    println!("Vector: {:?}", v);
    println!("Evens average: {:?}\n", find_evens_average(&v).unwrap());

    let v = vec!['a', 'b', 'c', 'd'];
    println!("Vector: {:?}", v);
    println!("Permutations: {:?}\n", build_words(&v));

    let v = vec![2, 5, 8, 10, 12, 11];
    println!("Vector: {:?}", v);
    println!("Sample: {:?}\n", sample(&v).unwrap());

    let v = vec![100.5, 80.0, 40.2, 33.3];
    println!("Fahrenheit readings: {:?}", v);
    println!("Celsius average: {:?}\n", find_celsius_average(&v).unwrap());

    let v = vec!["Skirt", "Dress"];
    println!("Clothes: {:?}", v);
    println!("Inventory: {:?}\n", mark_inventory(&v, 5));

    let v = vec![vec![0, 1, 1, 1, 0], vec![0, 1, 0, 1, 0, 1], vec![1, 0]];
    println!("Vector: {:?}", v);
    println!("Ones: {:?}\n", count_ones(v));

    let s = "rötör";
    println!("String: {:?}", s);
    println!("Palindrome: {:?}\n", is_palindrome(s));
}

fn find_evens_average(data: &[i32]) -> Option<f32> {
    let mut sum = 0;
    let mut count = 0;

    for v in data {
        if v % 2 == 0 {
            sum += v;
            count += 1;
        }
    }
    match count {
        0 => None,
        _ => Some(sum as f32 / count as f32),
    }
}

//// Rust Extras
#[allow(dead_code)]
fn find_evens_average_extra(data: &[i32]) -> Option<f32> {
    let evens: Vec<_> = data.iter().filter(|&x| x % 2 == 0).copied().collect();
    match evens.is_empty() {
        true => None,
        false => Some(evens.iter().sum::<i32>() as f32 / evens.len() as f32),
    }
}

fn build_words(data: &[char]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut word;
    for c in data {
        for d in data {
            if c != d {
                word = String::from(*c);
                word.push(*d);
                result.push(word);
            }
        }
    }
    result
}

fn sample(data: &[i32]) -> Option<[i32; 3]> {
    let data_len = data.len();
    match data_len {
        0 => None,
        _ => Some([data[0], data[data_len / 2], data[data_len - 1]]),
    }
}

fn find_celsius_average(data_fahrenheit: &[f32]) -> Option<f32> {
    if data_fahrenheit.is_empty() {
        return None;
    }
    let mut data_celsius = Vec::new();
    for f in data_fahrenheit {
        let converted = (f - 32.0) / 1.8;
        data_celsius.push(converted);
    }
    let mut sum = 0.0;
    for c in data_celsius.iter() {
        sum += c;
    }
    Some(sum / data_celsius.len() as f32)
}

//// Rust Extras
#[allow(dead_code)]
fn find_celsius_average_extra(data_fahrenheit: &[f32]) -> Option<f32> {
    let data_len = data_fahrenheit.len();
    match data_len {
        0 => None,
        _ => {
            let sum: f32 = data_fahrenheit
                .iter()
                .copied()
                .map(|f| (f - 32.0) / 1.8)
                .sum();
            Some(sum / data_len as f32)
        }
    }
}

fn mark_inventory(clothes: &[&str], max_size: i8) -> Vec<String> {
    let mut inventory: Vec<String> = Vec::new();
    for cloth in clothes {
        for size in 1..=max_size {
            inventory.push(format!("{cloth} Size: {}", size));
        }
    }
    inventory
}

fn count_ones(data: Vec<Vec<u8>>) -> usize {
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
fn count_ones_extra(data: Vec<Vec<u8>>) -> usize {
    data.into_iter().flatten().filter(|x| *x == 1).count()
}

fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    for i in 0..(len / 2) {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

//// Rust Extras
#[allow(dead_code)]
fn is_palindrome_extra_1(s: &str) -> bool {
    let mid = s.len() / 2;
    let left = s.chars().take(mid);
    let right = s.chars().rev().take(mid);
    for (l, r) in left.zip(right) {
        if l != r {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn is_palindrome_extra_2(s: &str) -> bool {
    let mid = s.len() / 2;
    let left = s.chars().take(mid);
    let right = s.chars().rev().take(mid);
    left.eq(right)
}

#[allow(dead_code)]
fn is_palindrome_extra_3(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_evens_average() {
        assert_eq!(None, find_evens_average(&[]));
        assert_eq!(Some(3.0), find_evens_average(&[4, 2, 7, 1, 3]));
        assert_eq!(Some(2.5), find_evens_average(&[5, 4, 2, 9, 2, 2]));
    }

    #[test]
    fn test_find_evens_average_extra() {
        assert_eq!(None, find_evens_average_extra(&[]));
        assert_eq!(Some(3.0), find_evens_average_extra(&[4, 2, 7, 1, 3]));
        assert_eq!(Some(2.5), find_evens_average_extra(&[5, 4, 2, 9, 2, 2]));
    }

    #[test]
    fn test_build_words() {
        assert_eq!(Vec::<String>::new(), build_words(&[]));
        assert_eq!(
            vec!["ab", "ac", "ba", "bc", "ca", "cb"],
            build_words(&['a', 'b', 'c'])
        );
    }

    #[test]
    fn test_sample() {
        assert_eq!(None, sample(&[]));
        assert_eq!(Some([2, 2, 2]), sample(&[2]));
        assert_eq!(Some([2, 4, 10]), sample(&[2, 4, 10]));
        assert_eq!(Some([2, 8, 10]), sample(&[2, 4, 8, 10]));
        assert_eq!(Some([2, 9, 11]), sample(&[2, 4, 8, 9, 10, 11]));
    }

    #[test]
    fn test_find_celsius_average() {
        assert_eq!(None, find_celsius_average(&[]));
        assert_eq!(Some(0.0), find_celsius_average(&[32.0]));
        assert_eq!(Some(30.0), find_celsius_average(&[68.0, 104.0]));
    }

    #[test]
    fn test_find_celsius_average_extra() {
        assert_eq!(None, find_celsius_average_extra(&[]));
        assert_eq!(Some(0.0), find_celsius_average_extra(&[32.0]));
        assert_eq!(Some(30.0), find_celsius_average_extra(&[68.0, 104.0]));
    }

    #[test]
    fn test_mark_inventory() {
        assert_eq!(Vec::<String>::new(), mark_inventory(&[], 5));
        let inventory = &["Purple Shirt", "Green Shirt"];
        let marked_inventory_1 = vec!["Purple Shirt Size: 1", "Green Shirt Size: 1"];
        assert_eq!(marked_inventory_1, mark_inventory(inventory, 1));
        let marked_inventory_3 = vec![
            "Purple Shirt Size: 1",
            "Purple Shirt Size: 2",
            "Purple Shirt Size: 3",
            "Green Shirt Size: 1",
            "Green Shirt Size: 2",
            "Green Shirt Size: 3",
        ];
        assert_eq!(marked_inventory_3, mark_inventory(inventory, 3));
    }

    #[test]
    fn test_count_ones() {
        assert_eq!(0, count_ones(vec![vec![]]));
        assert_eq!(0, count_ones(vec![vec![0, 0], vec![0, 0]]));
        assert_eq!(1, count_ones(vec![vec![0, 1], vec![0, 0]]));
        assert_eq!(3, count_ones(vec![vec![0, 1], vec![1, 1]]));
    }

    #[test]
    fn test_count_ones_extra() {
        assert_eq!(0, count_ones_extra(vec![vec![]]));
        assert_eq!(0, count_ones_extra(vec![vec![0, 0], vec![0, 0]]));
        assert_eq!(1, count_ones_extra(vec![vec![0, 1], vec![0, 0]]));
        assert_eq!(3, count_ones_extra(vec![vec![0, 1], vec![1, 1]]));
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(""));
        assert!(is_palindrome("a"));
        assert!(is_palindrome("abba"));
        assert!(!is_palindrome("abja"));
        assert!(!is_palindrome("rotör"));
        assert!(is_palindrome("rötör"));
        assert!(is_palindrome("racecar"));
    }

    #[test]
    fn test_is_palindrome_extra_1() {
        assert!(is_palindrome_extra_1(""));
        assert!(is_palindrome_extra_1("a"));
        assert!(is_palindrome_extra_1("abba"));
        assert!(!is_palindrome_extra_1("abja"));
        assert!(!is_palindrome_extra_1("rotör"));
        assert!(is_palindrome_extra_1("rötör"));
        assert!(is_palindrome_extra_1("racecar"));
    }

    #[test]
    fn test_is_palindrome_extra_2() {
        assert!(is_palindrome_extra_2(""));
        assert!(is_palindrome_extra_2("a"));
        assert!(is_palindrome_extra_2("abba"));
        assert!(!is_palindrome_extra_2("abja"));
        assert!(!is_palindrome_extra_2("rotör"));
        assert!(is_palindrome_extra_2("rötör"));
        assert!(is_palindrome_extra_2("racecar"));
    }

    #[test]
    fn test_is_palindrome_extra_3() {
        assert!(is_palindrome_extra_3(""));
        assert!(is_palindrome_extra_3("a"));
        assert!(is_palindrome_extra_3("abba"));
        assert!(!is_palindrome_extra_3("abja"));
        assert!(!is_palindrome_extra_3("rotör"));
        assert!(is_palindrome_extra_3("rötör"));
        assert!(is_palindrome_extra_3("racecar"));
    }
}
