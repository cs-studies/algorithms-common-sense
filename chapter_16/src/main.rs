fn main() {
    println!("\n*** Chapter 16 ***\n");

    let h: Heap<i32> = Heap::new();
    dbg!(&h);
}

#[derive(Debug)]
struct Heap<T> {
    data: Vec<T>,
}

impl<T> Heap<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn root_node(&self) -> Option<&T> {
        self.data.first()
    }

    fn last_node(&self) -> Option<&T> {
        self.data.last()
    }

    fn insert(&mut self, value: T) {
        self.data.push(value);
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
        heap.insert("A");
        heap.insert("B");
        heap.insert("C");
        assert_eq!(heap.root_node(), Some(&"A"));
        assert_eq!(heap.last_node(), Some(&"C"));
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
