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
    println!("Sorted: {:?}", bubble_sort(&v));
}

fn bubble_sort(data: &Vec<i32>) -> Vec<i32> {
    let mut result = data.clone();
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
            println!("check index {i}: {:?}", result);
        }
        last_index -= 1;
    }
    result
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
}
