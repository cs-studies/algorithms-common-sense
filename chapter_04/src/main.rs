use std::collections::HashSet;
mod exercises;

fn main() {
    println!("\n*** Chapter 04 ***\n");

    let v = vec![];
    println!("Vector: {:?}", v);
    println!("Sorted: {:?}\n", bubble_sort(&v));

    let v = vec![4, 2, 7, 1, 3];
    println!("Vector: {:?}", v);
    println!("Sorted: {:?}\n", bubble_sort(&v));

    let v = vec![65, -55, 45, -35, 0, 15, 10];
    println!("Vector: {:?}", v);
    println!("Sorted: {:?}\n", bubble_sort(&v));

    let a = [1, 4, 5, 2, 9];
    println!("Array: {:?}", a);
    println!("Duplicates: {}\n", has_duplicates(&a));

    let a = [65, -55, 45, 45, 10];
    println!("Array: {:?}", a);
    println!("Duplicates: {}\n", has_duplicates(&a));

    let a = [1, 4, 5, 2, 9];
    println!("Array: {:?}", a);
    println!("Duplicates: {}\n", has_duplicates_linear(&a));

    let a = [65, -55, 45, 45, 10];
    println!("Array: {:?}", a);
    println!("Duplicates: {}\n", has_duplicates_linear(&a));

    println!("Greatest Product: {:?}",
             exercises::greatest_product(&[5, 3, 7, 1]));
    println!("Greatest Number: {:?}",
             exercises::greatest_number(&[5, 3, 7, 1]));
}

fn bubble_sort(data: &[i32]) -> Vec<i32> {
    let mut result = data.to_owned();
    let mut last_index = match result.len().checked_sub(1) {
        Some(val) => val,
        None => return result,
    };
    let mut is_sorted = false;

    while !is_sorted {
        is_sorted = true;
        for i in 0..last_index {
            if result[i] > result[i + 1] {
                is_sorted = false;
                result.swap(i, i + 1);
            }
            println!("Check index {i}: {:?}", result);
        }
        last_index -= 1;
    }
    result
}

fn has_duplicates(data: &[i32]) -> bool {
    let mut steps = 0;
    for (i, m) in data.iter().enumerate() {
        for (j, n) in data.iter().enumerate() {
            steps += 1;
            if i != j && m == n {
                return true;
            }
        }
    }
    println!("Steps: {steps}");
    false
}

fn has_duplicates_linear(data: &[i32]) -> bool {
    let mut steps = 0;
    let mut existing_numbers = HashSet::new();
    for v in data {
        steps += 1;
        if !existing_numbers.insert(v) {
            return true;
        }
        println!("Checked numbers: {:?}", existing_numbers);
    }
    println!("Steps: {:?}", steps);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        assert_eq!(Vec::<i32>::new(), bubble_sort(&vec![]));

        let v = vec![65, -55, 45, -35, 0, 15, 10];
        assert_eq!(vec![-55, -35, 0, 10, 15, 45, 65], bubble_sort(&v));
    }

    #[test]
    fn test_has_duplicates() {
        assert_eq!(false, has_duplicates(&[]));
        assert!(has_duplicates(&[1, 5, 3, 9, 1, 4]));
    }

    #[test]
    fn test_has_duplicates_linear() {
        assert_eq!(false, has_duplicates_linear(&[]));
        assert!(has_duplicates_linear(&[1, 5, 3, 9, 1, 4]));
    }
}
