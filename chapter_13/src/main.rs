use std::cmp::Ordering;

fn main() {
    println!("\n*** Chapter 13 ***\n");

    let v = vec![0, -5, 2, 1, -6, 3];
    println!("Before: {:?}", v);
    let right_idx = v.len().saturating_sub(1);
    let mut sa = SortableArray::new(v);
    sa.quicksort(0, right_idx);
    println!("After: {:?}", sa.data);

    let v = vec![0, -50, 20, 10, -60, 30];
    println!("\n{:?}", v);
    let right_idx = v.len().saturating_sub(1);
    let mut sa = SortableArray::new(v);
    println!(
        "The 3-rd smallest value is {:?}",
        sa.quickselect(3, 0, right_idx)
    );

    let mut v = vec![0, -50, 20, 10, -60, 10];
    println!("\nVector {:?}", v);
    println!("Has duplicates: {}", has_duplicates(&mut v));
}

#[derive(Debug)]
struct SortableArray {
    data: Vec<i32>,
}

impl SortableArray {
    pub fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    pub fn quicksort(&mut self, left: usize, right: usize) {
        if left >= right {
            return;
        }
        let pivot = self.partition(left, right);
        if pivot > 0 {
            self.quicksort(left, pivot - 1);
        }
        self.quicksort(pivot + 1, right);
    }

    pub fn quickselect(&mut self, k: usize, left: usize, right: usize) -> i32 {
        if left >= right {
            return self.data[left];
        }
        let pivot = self.partition(left, right);
        match k.cmp(&pivot) {
            Ordering::Less => self.quickselect(k, left, pivot - 1),
            Ordering::Greater => self.quickselect(k, pivot + 1, right),
            Ordering::Equal => self.data[pivot],
        }
    }

    fn partition(&mut self, mut left: usize, mut right: usize) -> usize {
        let pivot = right;
        let pivot_val = self.data[pivot];
        right -= 1;

        loop {
            while self.data[left] < pivot_val {
                left += 1;
            }
            while self.data[right] > pivot_val {
                if let Some(val) = right.checked_sub(1) {
                    right = val;
                } else {
                    break;
                }
            }
            if left >= right {
                break;
            } else {
                self.data.swap(left, right);
            }
        }

        self.data.swap(left, pivot);

        left
    }
}

fn has_duplicates(data: &mut [i32]) -> bool {
    let data_len = data.len();
    if data_len < 2 {
        return false;
    }
    data.sort();
    for i in 0..(data_len - 1) {
        if data[i] == data[i+1] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        let mut sa = SortableArray::new(vec![]);
        sa.quicksort(0, 0);
        assert_eq!(sa.data, []);

        let mut sa = SortableArray::new(vec![2]);
        sa.quicksort(0, 0);
        assert_eq!(sa.data, [2]);

        let mut sa = SortableArray::new(vec![2, 1]);
        sa.quicksort(0, 1);
        assert_eq!(sa.data, [1, 2]);

        let mut sa = SortableArray::new(vec![0, -5, 2, 6, 3]);
        sa.quicksort(0, 4);
        assert_eq!(sa.data, [-5, 0, 2, 3, 6]);
    }

    #[test]
    fn test_quickselect() {
        let mut sa = SortableArray::new(vec![3]);
        assert_eq!(sa.quickselect(0, 0, 0), 3);

        let mut sa = SortableArray::new(vec![3, 4]);
        assert_eq!(sa.quickselect(0, 0, 1), 3);

        let mut sa = SortableArray::new(vec![3, 4]);
        assert_eq!(sa.quickselect(1, 0, 1), 4);

        let mut sa = SortableArray::new(vec![0, -50, 10, -60, 40]);
        assert_eq!(sa.quickselect(3, 0, 4), 10);
    }

    #[test]
    fn test_has_duplicates() {
        assert!(!has_duplicates(&mut []));
        assert!(!has_duplicates(&mut [3, 9, 4]));
        assert!(has_duplicates(&mut [5, 3, 1, 9, 1, 4]));
    }
}
