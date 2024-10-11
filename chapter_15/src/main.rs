mod binary_search_tree;

use binary_search_tree::{BinarySearchTree, Node};

fn main() {
    println!("\n*** Chapter 15 ***\n");

    let node1 = Node::new(25, None, None);
    let node2 = Node::new(75, None, None);
    let root =
        Node::new(50, node1.into_tree(), node2.into_tree()).into_tree();

    let mut bst = BinarySearchTree::new(root);
    dbg!(&bst);

    let found = bst.search(25);
    dbg!(&found);

    let found_none = bst.search(80);
    dbg!(&found_none);

    bst.insert(50);
    bst.insert(11);
    bst.insert(33);
    bst.insert(30);
    bst.insert(40);
    bst.insert(75);
    bst.insert(56);
    bst.insert(52);
    bst.insert(61);
    bst.insert(89);
    bst.insert(82);
    bst.insert(95);
    println!("After insert()");
    dbg!(&bst);

    bst.delete(56);
    println!("After delete()");
    dbg!(&bst);
}
