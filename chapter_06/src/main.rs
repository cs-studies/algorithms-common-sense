fn main() {
    println!("\n*** Chapter 06 ***\n");

    let mut v = vec![4, 2, 7, 1, 3];
    println!("Vector: {:?}", v);
    insertion_sort(&mut v);
    println!("Sorted: {:?}\n", v);
}

fn insertion_sort(data: &mut [i32]) {
    for i in 1..data.len() {
        let mut p = i;
        while p > 0 && data[p - 1] > data[p] {
            data.swap(p, p - 1);
            p -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut v: Vec<i32> = vec![];
        insertion_sort(&mut v);
        assert_eq!(Vec::<i32>::new(), v);

        let mut a = [65, -55, 45, -35, 0, 15, 10];
        insertion_sort(&mut a);
        assert_eq!([-55, -35, 0, 10, 15, 45, 65], a);
    }
}
