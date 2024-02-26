fn main() {
    println!("\n*** Chapter 13 ***\n");

    let mut v = vec![0, 5, 2, 1, 6, 3];
    v.sort();
    println!("sorted: {:?}", v);

    // let sa = SortableArray::new(v);
}

struct SortableArray {
    data: Vec<i32>,
}

impl SortableArray {
    pub fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    fn partition(&mut self, mut left_idx: usize, mut right_idx: usize) -> usize {
        let pivot_idx = right_idx;
        let pivot = self.data[pivot_idx];
        right_idx -= 1;

        loop {
            while self.data[left_idx] < pivot {
                left_idx += 1;
            }
            while self.data[right_idx] > pivot {
                right_idx -= 1;
            }
            if left_idx >= right_idx {
                break;
            } else {
                self.data.swap(left_idx, right_idx);
                // left_idx += 1;
            }
        }

        self.data.swap(left_idx, pivot_idx);

        left_idx
    }
}
