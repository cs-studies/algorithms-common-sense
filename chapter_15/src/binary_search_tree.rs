use std::cmp::Ordering;

type Child<T> = Option<Box<TreeNode<T>>>;

#[derive(Debug)]
pub struct TreeNode<T: Ord + Clone> {
    value: T,
    left: Child<T>,
    right: Child<T>,
}

impl<T: Ord + Clone> TreeNode<T> {
    pub fn new(value: T, left: Child<T>, right: Child<T>) -> Self {
        Self { value, left, right }
    }

    pub fn into_child(self) -> Child<T> {
        Some(Box::new(self))
    }

    pub fn search(&self, value: T) -> Option<&Self> {
        match value.cmp(&self.value) {
            Ordering::Equal => Some(self),
            Ordering::Less => self.left.as_deref()?.search(value),
            Ordering::Greater => self.right.as_deref()?.search(value),
        }
    }

    pub fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Equal => {}
            Ordering::Less => match self.left {
                Some(ref mut node) => node.insert(value),
                None => {
                    self.left = TreeNode::new(value, None, None).into_child();
                }
            },
            Ordering::Greater => match self.right {
                Some(ref mut node) => node.insert(value),
                None => {
                    self.right = TreeNode::new(value, None, None).into_child();
                }
            },
        }
    }

    // This implementation is not ideal,
    // but close enough to the book.
    // Maybe, it'll be improved with time.
    pub fn delete(&mut self, value: T) {
        self.left = Self::delete_do(value.clone(), self.left.take());
        self.right = Self::delete_do(value, self.right.take());
    }

    fn delete_do(value: T, root: Child<T>) -> Child<T> {
        match root {
            None => None,
            Some(mut node) => match value.cmp(&node.value) {
                Ordering::Less => {
                    node.left = Self::delete_do(value, node.left);
                    Some(node)
                }
                Ordering::Greater => {
                    node.right = Self::delete_do(value, node.right);
                    Some(node)
                }
                Ordering::Equal => {
                    if node.left.is_none() {
                        node.right
                    } else if node.right.is_none() {
                        node.left
                    } else {
                        let mut successor = node.right.as_ref();
                        while let Some(ref s) = successor.unwrap().left {
                            successor = Some(s);
                        }
                        let successor_val = &successor.unwrap().value;
                        node.value = successor_val.clone();
                        node.right = Self::delete_do(successor_val.clone(), node.right);
                        Some(node)
                    }
                }
            },
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

    #[test]
    fn test_insert() {
        let mut root = TreeNode::new(20, None, None);
        root.insert(20);
        assert!(root.left.is_none());
        assert!(root.right.is_none());

        root.insert(30);
        assert!(root.left.is_none());
        assert_eq!(root.right.as_ref().unwrap().value, 30);

        root.insert(10);
        assert_eq!(root.left.unwrap().value, 10);
        assert_eq!(root.right.unwrap().value, 30);
    }

    #[test]
    fn test_delete() {
        let mut root = TreeNode::new(20, None, None);
        root.insert(10);
        root.delete(10);
        assert!(root.left.is_none());
        assert!(root.right.is_none());

        root.insert(10);
        root.insert(30);
        root.delete(30);
        assert_eq!(root.left.unwrap().value, 10);
        assert!(root.right.is_none());
    }
}
