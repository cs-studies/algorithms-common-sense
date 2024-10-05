use std::cmp::Ordering;

fn main() {
    println!("\n*** Chapter 15 ***\n");

    let node1 = TreeNode::new(25, None, None);
    let node2 = TreeNode::new(75, None, None);
    let root = TreeNode::new(50, node1.into_child(), node2.into_child());
    dbg!(&root);

    let found = root.search(25);
    dbg!(&found);

    let found_none = root.search(80);
    dbg!(&found_none);
}

type Child<T> = Option<Box<TreeNode<T>>>;

#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Child<T>,
    right: Child<T>,
}

impl<T: Ord> TreeNode<T> {
    fn new(value: T, left: Child<T>, right: Child<T>) -> Self {
        Self { value, left, right }
    }

    fn into_child(self) -> Child<T> {
        Some(Box::new(self))
    }

    fn search(&self, value: T) -> Option<&Self> {
        match value.cmp(&self.value) {
            Ordering::Equal => Some(self),
            Ordering::Less => self.left.as_deref()?.search(value),
            Ordering::Greater => self.right.as_deref()?.search(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let node1 = TreeNode::new(11, None, None);
        assert_eq!(node1.value, 11);
        assert!(node1.left.is_none());
        assert!(node1.right.is_none());

        let node2 = TreeNode::new(12, node1.into_child(), None);
        assert_eq!(node2.value, 12);
        assert_eq!(node2.left.unwrap().value, 11);
        assert!(node2.right.is_none());

        let node3 = TreeNode::new(
            13,
            TreeNode::new(1, None, None).into_child(),
            TreeNode::new(2, None, None).into_child(),
        );
        assert_eq!(node3.value, 13);
        assert_eq!(node3.left.unwrap().value, 1);
        assert_eq!(node3.right.unwrap().value, 2);
    }

    #[test]
    fn test_search() {
        let node1 = TreeNode::new(25, None, None);
        assert_eq!(node1.search(25).unwrap().value, 25);

        let node2 = TreeNode::new(
            13,
            TreeNode::new(1, None, None).into_child(),
            TreeNode::new(22, None, None).into_child(),
        );
        assert!(node2.search(100).is_none());
        assert_eq!(node2.search(22).unwrap().value, 22);
    }
}
