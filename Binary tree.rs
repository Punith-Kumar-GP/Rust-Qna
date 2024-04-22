use std::cmp;


#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            cmp::max(left_depth, right_depth) + 1
        }
    }
}

fn main() {
    let mut root = TreeNode::new(3);
    let left = Some(Box::new(TreeNode::new(9)));
    let right = Some(Box::new(TreeNode::new(20)));
    let mut right_node = TreeNode::new(15);
    let right_right = Some(Box::new(TreeNode::new(7)));

    right_node.right = right_right;
    root.left = left;
    root.right = Some(Box::new(right_node));

    let depth = max_depth(Some(Box::new(root)));
    println!("Max depth of the binary tree: {}", depth);
}
