fn main() {
    println!("\n*** Chapter 13 ***\n");

    let v = vec![0, -5, 2, 1, -6, 3];
    println!("Before: {:?}", v);

    let right_idx = v.len().saturating_sub(1);
    let mut sa = SortableArray::new(v);
    sa.quicksort(0, right_idx);
    println!("After: {:?}", sa.data);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        let data = vec![0, -5, 2, 6, 3];
        let right_idx = data.len() - 1;
        let mut sa = SortableArray::new(data);
        sa.quicksort(0, right_idx);
        assert_eq!(sa.data, [-5, 0, 2, 3, 6]);
    }
}
