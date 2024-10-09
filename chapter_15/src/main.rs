mod binary_search_tree;

use binary_search_tree::TreeNode;

fn main() {
    println!("\n*** Chapter 15 ***\n");

    let node1 = TreeNode::new(25, None, None);
    let node2 = TreeNode::new(75, None, None);
    let mut root = TreeNode::new(50, node1.into_child(), node2.into_child());
    dbg!(&root);

    let found = root.search(25);
    dbg!(&found);

    let found_none = root.search(80);
    dbg!(&found_none);

    root.insert(50);
    root.insert(11);
    root.insert(33);
    root.insert(30);
    root.insert(40);
    root.insert(75);
    root.insert(56);
    root.insert(52);
    root.insert(61);
    root.insert(89);
    root.insert(82);
    root.insert(95);
    println!("After insert()");
    dbg!(&root);

    root.delete(56);
    println!("After delete()");
    dbg!(&root);
}
