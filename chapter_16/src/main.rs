use std::collections::VecDeque;

fn main() {
    println!("\n*** Chapter 16 ***\n");

    let mut h: Heap<i32> = Heap::new();
    h.insert(10);
    h.insert(15);
    h.insert(25);
    h.insert(8);
    h.insert(20);
    dbg!(&h);
    println!("root_node: {:?}", h.root_node());
    println!("last_node: {:?}\n", h.last_node());

    h.delete();
    dbg!(&h);
    println!("root_node: {:?}", h.root_node());
    println!("last_node: {:?}", h.last_node());
}

#[derive(Debug)]
struct Heap<T> {
    data: VecDeque<T>,
}

impl<T: PartialOrd> Heap<T> {
    fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    fn root_node(&self) -> Option<&T> {
        self.data.front()
    }

    fn last_node(&self) -> Option<&T> {
        self.data.back()
    }

    fn insert(&mut self, value: T) {
        self.data.push_back(value);

        let mut node_idx = self.data.len() - 1;

        // When node_idx is 0, parent_index returns None.
        while let Some(parent_idx) = Self::parent_index(node_idx) {
            if self.data[node_idx] > self.data[parent_idx] {
                self.data.swap(node_idx, parent_idx);
                node_idx = parent_idx;
            } else {
                break;
            }
        }
    }

    fn delete(&mut self) {
        if let Some(last) = self.data.pop_back() {
            if !self.data.is_empty() {
                self.data[0] = last;
            } else {
                return;
            }
        } else {
            return;
        }

        let mut trickle_idx = 0;

        while let Some(greater_idx) = self.greater_child_index(trickle_idx) {
            self.data.swap(trickle_idx, greater_idx);
            trickle_idx = greater_idx;
        }
    }

    fn greater_child_index(&self, i: usize) -> Option<usize> {
        let val = &self.data.get(i)?;

        let left_idx = Self::left_child_index(i)?;
        let right_idx = Self::right_child_index(i)?;

        let left_val = &self.data.get(left_idx);
        let right_val = &self.data.get(right_idx);

        match (left_val, right_val) {
            (Some(l), Some(r)) if r > l && r > val => Some(right_idx),
            (Some(l), _) if l > val => Some(left_idx),
            _ => None,
        }
    }

    fn left_child_index(i: usize) -> Option<usize> {
        i.checked_mul(2)?.checked_add(1)
    }

    fn right_child_index(i: usize) -> Option<usize> {
        i.checked_mul(2)?.checked_add(2)
    }

    fn parent_index(i: usize) -> Option<usize> {
        i.checked_sub(1)?.checked_div(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap() {
        let mut heap = Heap::new();
        heap.insert("X");
        heap.insert("Y");
        heap.insert("Z");
        // heap now contains Z X Y
        assert_eq!(heap.root_node().unwrap(), &"Z");
        assert_eq!(heap.last_node().unwrap(), &"Y");

        let mut heap = Heap::new();
        heap.insert(1);
        heap.insert(2);
        heap.insert(3);
        heap.insert(4);
        heap.insert(5);
        // heap now contains 5 4 2 1 3
        assert_eq!(heap.root_node().unwrap(), &5);
        assert_eq!(heap.last_node().unwrap(), &3);
    }

    #[test]
    fn test_delete() {
        let mut heap = Heap::<i32>::new();
        heap.delete();
        assert!(heap.data.is_empty());

        heap.insert(1);
        heap.delete();
        assert!(heap.data.is_empty());

        heap.insert(1);
        heap.insert(2);
        heap.delete();
        assert_eq!(heap.root_node().unwrap(), &1);
        assert_eq!(heap.last_node().unwrap(), &1);

        heap.insert(2);
        heap.insert(3);
        // heap now contains 3 1 2
        heap.delete();
        assert_eq!(heap.root_node().unwrap(), &2);
        assert_eq!(heap.last_node().unwrap(), &1);
    }

    #[test]
    fn test_left_child_index() {
        assert_eq!(Heap::<i8>::left_child_index(0).unwrap(), 1);
        assert_eq!(Heap::<i8>::left_child_index(4).unwrap(), 9);
        assert_eq!(Heap::<i8>::left_child_index(5).unwrap(), 11);
    }

    #[test]
    fn test_right_child_index() {
        assert_eq!(Heap::<i8>::right_child_index(0).unwrap(), 2);
        assert_eq!(Heap::<i8>::right_child_index(4).unwrap(), 10);
        assert_eq!(Heap::<i8>::right_child_index(5).unwrap(), 12);
    }

    #[test]
    fn test_parent_index() {
        assert_eq!(Heap::<i8>::parent_index(1).unwrap(), 0);
        assert_eq!(Heap::<i8>::parent_index(2).unwrap(), 0);
        assert_eq!(Heap::<i8>::parent_index(4).unwrap(), 1);
        assert_eq!(Heap::<i8>::parent_index(9).unwrap(), 4);
        assert_eq!(Heap::<i8>::parent_index(10).unwrap(), 4);
    }
}
