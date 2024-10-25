use crate::{BinarySearchTree, Node};

impl<T> BinarySearchTree<T> {
    pub fn find_greatest(&self) -> Option<&T> {
        match &self.root {
            None => None,
            Some(node) => node.find_greatest(),
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
