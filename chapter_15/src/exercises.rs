use crate::{BinarySearchTree, Node};
use std::fmt::Debug;

impl<T> BinarySearchTree<T> {
    pub fn find_greatest(&self) -> Option<&T> {
        match &self.root {
            None => None,
            Some(node) => node.find_greatest(),
        }
    }
}

impl<T: Debug> BinarySearchTree<T> {
    pub fn traverse_preorder(&self) {
        match &self.root {
            None => {}
            Some(node) => node.traverse_preorder(),
        }
    }

    pub fn traverse_postorder(&self) {
        match &self.root {
            None => {}
            Some(node) => node.traverse_postorder(),
        }
    }
}

impl<T> Node<T> {
    fn find_greatest(&self) -> Option<&T> {
        match &self.right {
            None => Some(&self.value),
            Some(right) => right.find_greatest(),
        }
    }
}

impl<T: Debug> Node<T> {
    fn traverse_preorder(&self) {
        println!("{:?}", &self.value);
        if let Some(left) = &self.left {
            left.traverse_preorder();
        }
        if let Some(right) = &self.right {
            right.traverse_preorder();
        }
    }

    fn traverse_postorder(&self) {
        if let Some(left) = &self.left {
            left.traverse_postorder();
        }
        if let Some(right) = &self.right {
            right.traverse_postorder();
        }
        println!("{:?}", &self.value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_greatest() {
        let bst = BinarySearchTree::<i32>::new(None);
        assert!(bst.find_greatest().is_none());

        let node = Node::new(1, None, None);
        let bst = BinarySearchTree::new(node.into_tree());
        assert_eq!(bst.find_greatest().unwrap(), &1);

        let bst = BinarySearchTree::new(
            Node::new(
                "B",
                Node::new("A", None, None).into_tree(),
                Node::new("C", None, None).into_tree(),
            )
            .into_tree(),
        );
        assert_eq!(bst.find_greatest().unwrap(), &"C");
    }
}
