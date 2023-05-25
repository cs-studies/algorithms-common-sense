fn main() {
    println!("\n*** Chapter 07 ***\n");

    let v = vec![4, 2, 4, 1, 3];
    println!("Vector: {:?}", v);
    println!("Evens average: {:?}\n", find_evens_average(&v));
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
}
