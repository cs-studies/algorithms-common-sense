mod binary_search_tree;

use binary_search_tree::{BinarySearchTree, Node};

fn main() {
    println!("\n*** Chapter 15 ***\n");

    let node1 = Node::new(25, None, None);
    let node2 = Node::new(75, None, None);

    let mut bst = BinarySearchTree::new(
        Node::new(50, node1.into_tree(), node2.into_tree()).into_tree(),
    );
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
    println!("\nAfter insert()");
    dbg!(&bst);

    bst.delete(56);
    println!("\nAfter delete()");
    dbg!(&bst);

    let bst = BinarySearchTree::new(
        Node::new(
            "Moby Dick",
            Node::new(
                "Great Expectations",
                Node::new("Alice in Wonderland", None, None).into_tree(),
                Node::new("Lord of the Files", None, None).into_tree(),
            )
            .into_tree(),
            Node::new(
                "Robinson Crusoe",
                Node::new("Pride and Prejudice", None, None).into_tree(),
                Node::new("The Odyssey", None, None).into_tree(),
            )
            .into_tree(),
        )
        .into_tree(),
    );

    println!("\nTraverse inorder:");
    bst.traverse_inorder();
}
