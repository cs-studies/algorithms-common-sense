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
}
