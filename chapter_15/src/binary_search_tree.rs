use std::{cmp::Ordering, fmt::Debug};

type Tree<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub(crate) value: T,
    left: Tree<T>,
    pub(crate) right: Tree<T>,
}

#[derive(Debug)]
pub struct BinarySearchTree<T> {
    pub(crate) root: Tree<T>,
}

impl<T: Ord + Debug> BinarySearchTree<T> {
    pub fn new(root: Tree<T>) -> Self {
        Self { root }
    }

    pub fn search(&self, value: T) -> Option<&Node<T>> {
        match &self.root {
            None => None,
            Some(node) => node.search(value),
        }
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            None => {
                self.root = Node::new(value, None, None).into_tree();
            }
            Some(ref mut node) => node.insert(value),
        }
    }

    pub fn delete(&mut self, value: T) {
        Node::delete(&mut self.root, value);
    }

    pub fn traverse_inorder(&self) {
        match &self.root {
            None => {},
            Some(node) => node.traverse_inorder(),
        }
    }
}

impl<T: Ord + Debug> Node<T> {
    pub fn new(value: T, left: Tree<T>, right: Tree<T>) -> Self {
        Self { value, left, right }
    }

    pub fn into_tree(self) -> Tree<T> {
        Some(Box::new(self))
    }

    fn search(&self, value: T) -> Option<&Self> {
        match value.cmp(&self.value) {
            Ordering::Equal => Some(self),
            Ordering::Less => self.left.as_deref()?.search(value),
            Ordering::Greater => self.right.as_deref()?.search(value),
        }
    }

    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Equal => {}
            Ordering::Less => match self.left {
                Some(ref mut node) => node.insert(value),
                None => {
                    self.left = Node::new(value, None, None).into_tree();
                }
            },
            Ordering::Greater => match self.right {
                Some(ref mut node) => node.insert(value),
                None => {
                    self.right = Node::new(value, None, None).into_tree();
                }
            },
        }
    }

    fn delete(link: &mut Tree<T>, value: T) {
        if let Some(ref mut node) = link {
            match value.cmp(&node.value) {
                Ordering::Less => Node::delete(&mut node.left, value),
                Ordering::Greater => Node::delete(&mut node.right, value),
                Ordering::Equal => match (&node.left, &node.right) {
                    (None, None) => *link = None,
                    (None, Some(_)) => *link = node.right.take(),
                    (Some(_), None) => *link = node.left.take(),
                    (Some(_), Some(_)) => {
                        let mut successor = &mut node.right;
                        while successor.as_ref().unwrap().left.is_some() {
                            successor = &mut successor.as_mut().unwrap().left;
                        }
                        let successor_node = successor.take().unwrap();
                        *successor = successor_node.right;
                        node.value = successor_node.value;

                        // Or use the lift function instead.
                        /*
                        node.value = lift(&mut node.right).unwrap();
                        fn lift<T>(tree: &mut Tree<T>) -> Option<T> {
                            if tree.as_ref().unwrap().left.is_some() {
                                lift(&mut tree.as_mut().unwrap().left)
                            } else {
                                let node = tree.take().unwrap();
                                *tree = node.right;
                                Some(node.value)
                            }
                        }*/
                    }
                },
            }
        }
    }

    fn traverse_inorder(&self) {
        if let Some(left) = &self.left {
            left.traverse_inorder();
        }
        println!("{:?}", &self.value);
        if let Some(right) = &self.right {
            right.traverse_inorder();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_new() {
        let bst = BinarySearchTree::<i32>::new(None);
        assert!(bst.root.is_none());

        let node = Node::new(1, None, None);
        let bst = BinarySearchTree::new(node.into_tree());
        assert!(bst.root.is_some());
        assert_eq!(bst.root.unwrap().value, 1);

        let node = Node::new(
            2,
            Node::new(1, None, None).into_tree(),
            Node::new(3, None, None).into_tree(),
        );
        let bst = BinarySearchTree::new(node.into_tree());
        let head = bst.root.as_ref().unwrap();
        assert_eq!(head.value, 2);
        assert_eq!(head.left.as_ref().unwrap().value, 1);
        assert_eq!(head.right.as_ref().unwrap().value, 3);
    }

    #[test]
    fn test_node_new() {
        let node1 = Node::new(25, None, None);
        assert_eq!(node1.value, 25);
        assert!(node1.left.is_none());
        assert!(node1.right.is_none());

        let node2 = Node::new(30, node1.into_tree(), None);
        assert_eq!(node2.value, 30);
        assert_eq!(node2.left.unwrap().value, 25);
        assert!(node2.right.is_none());

        let node3 = Node::new(
            2,
            Node::new(1, None, None).into_tree(),
            Node::new(3, None, None).into_tree(),
        );
        assert_eq!(node3.value, 2);
        assert_eq!(node3.left.unwrap().value, 1);
        assert_eq!(node3.right.unwrap().value, 3);
    }

    #[test]
    fn test_search() {
        let node = Node::new(25, None, None);
        let bst = BinarySearchTree::new(node.into_tree());
        assert!(bst.search(20).is_none());
        assert_eq!(bst.search(25).unwrap().value, 25);

        let node = Node::new(
            2,
            Node::new(1, None, None).into_tree(),
            Node::new(3, None, None).into_tree(),
        );
        let bst = BinarySearchTree::new(node.into_tree());
        assert!(bst.search(20).is_none());
        assert_eq!(bst.search(1).unwrap().value, 1);
        assert_eq!(bst.search(2).unwrap().value, 2);
        assert_eq!(bst.search(3).unwrap().value, 3);
    }

    #[test]
    fn test_insert() {
        let mut bst = BinarySearchTree::new(None);
        bst.insert(1);
        assert_eq!(bst.root.unwrap().value, 1);

        let node = Node::new(2, None, None);
        let mut bst = BinarySearchTree::new(node.into_tree());

        bst.insert(2);
        let head = bst.root.as_ref().unwrap();
        assert_eq!(head.value, 2);
        assert!(head.left.is_none());
        assert!(head.right.is_none());

        bst.insert(3);
        let head = bst.root.as_ref().unwrap();
        assert!(head.left.is_none());
        assert_eq!(head.right.as_ref().unwrap().value, 3);

        bst.insert(1);
        let head = bst.root.as_ref().unwrap();
        assert_eq!(head.left.as_ref().unwrap().value, 1);
        assert_eq!(head.right.as_ref().unwrap().value, 3);
    }

    #[test]
    fn test_delete() {
        let mut bst = BinarySearchTree::new(None);
        bst.delete(1);
        assert!(bst.root.is_none());

        let node = Node::new(2, None, None);
        let mut bst = BinarySearchTree::new(node.into_tree());
        bst.delete(1);
        assert!(bst.root.is_some());
        bst.delete(2);
        assert!(bst.root.is_none());

        bst.insert(10);
        bst.insert(5);
        bst.insert(20);
        bst.insert(15);
        bst.insert(25);

        bst.delete(10);
        let head = bst.root.as_ref().unwrap();
        assert_eq!(head.value, 15);
        assert!(head.left.is_some());
        assert!(head.right.is_some());
    }
}
