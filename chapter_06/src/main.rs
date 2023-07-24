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
        assert_eq!(v, Vec::<i32>::new());

        let mut a = [65, -55, 45, -35, 0, 15, 10];
        insertion_sort(&mut a);
        assert_eq!(a, [-55, -35, 0, 10, 15, 45, 65]);
    }

    #[test]
    fn test_intersection() {
        let v_empty = Vec::<i32>::new();
        assert_eq!(intersection(&[], &[]), v_empty);
        assert_eq!(intersection(&[1], &[]), v_empty);
        assert_eq!(intersection(&[], &[1]), v_empty);
        let x = intersection(&[3, 1, 4, 2], &[4, 5, 3, 6]);
        assert_eq!(x, vec![3, 4]);
    }

    #[test]
    fn test_intersection_extra() {
        let v_empty = Vec::<i32>::new();
        assert_eq!(intersection_extra(&[], &[]), v_empty);
        assert_eq!(intersection_extra(&[1], &[]), v_empty);
        assert_eq!(intersection_extra(&[], &[1]), v_empty);
        let x = intersection_extra(&[3, 1, 4, 2], &[4, 5, 3, 6]);
        assert_eq!(x, vec![3, 4]);
    }
}
