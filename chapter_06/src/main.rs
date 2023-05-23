fn main() {
    println!("\n*** Chapter 06 ***\n");

    let mut v = vec![4, 2, 7, 1, 3];
    println!("Vector: {:?}", v);
    insertion_sort(&mut v);
    println!("Sorted: {:?}\n", v);

    let m = [3, 1, 4, 2];
    let n = [4, 5, 3, 6];
    println!("m: {:?}", m);
    println!("n: {:?}", n);
    println!("Intersection: {:?}", intersection(&m, &n));
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

fn intersection(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = vec![];
    for i in a {
        for j in b {
            if i == j {
                result.push(*i);
                break;
            }
        }
    }
    result
}

//// Rust Extras
#[allow(dead_code)]
fn intersection_extra(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter().filter(|x| b.contains(x)).copied().collect()
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

    #[test]
    fn test_intersection() {
        let v_empty = Vec::<i32>::new();
        assert_eq!(v_empty, intersection(&[], &[]));
        assert_eq!(v_empty, intersection(&[1], &[]));
        assert_eq!(v_empty, intersection(&[], &[1]));
        let x = intersection(&[3, 1, 4, 2], &[4, 5, 3, 6]);
        assert_eq!(vec![3, 4], x);
    }

    #[test]
    fn test_intersection_extra() {
        let v_empty = Vec::<i32>::new();
        assert_eq!(v_empty, intersection_extra(&[], &[]));
        assert_eq!(v_empty, intersection_extra(&[1], &[]));
        assert_eq!(v_empty, intersection_extra(&[], &[1]));
        let x = intersection_extra(&[3, 1, 4, 2], &[4, 5, 3, 6]);
        assert_eq!(vec![3, 4], x);
    }
}
