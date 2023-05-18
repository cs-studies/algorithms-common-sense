fn main() {
    println!("\n*** Chapter 04 ***\n");

    let mut v = vec![4, 2, 7, 1, 3];
    println!("Vector: {:?}", v);
    println!("Sorted: {:?}\n", bubble_sort(&mut v));

    v = vec![65, -55, 45, -35, 0, 15, 10];
    println!("Vector: {:?}", v);
    println!("Sorted: {:?}", bubble_sort(&mut v));
}

/// In each pass-through, the highest unsorted value
/// "bubbles" up to its correct position.
fn bubble_sort(data: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut last_index = data.len() - 1;
    let mut is_finish = false;

    while !is_finish {
        is_finish = true;
        for i in 0..last_index {
            if data[i] > data[i + 1] {
                data.swap(i, i + 1);
                is_finish = false;
            }
            println!("check index {i}: {:?}", data);
        }
        last_index -= 1;
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![65, -55, 45, -35, 0, 15, 10];
        let mut v_sorted = vec![-55, -35, 0, 10, 15, 45, 65];
        assert_eq!(&mut v_sorted, bubble_sort(&mut v));
    }
}
