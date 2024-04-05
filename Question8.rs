/*
# Q. Given a binary tree, implement a function that returns the maximum depth of the tree.
Code:
*/
struct TreeNode {
        val: i32,
        left: Option<Box<TreeNode>>,
        right: Option<Box<TreeNode>>,
    }
    
    fn max_depth(root: Option<&Box<TreeNode>>) -> i32 {
        match root {
            Some(node) => {
                let left_depth = max_depth(node.left.as_ref());
                let right_depth = max_depth(node.right.as_ref());
                1 + left_depth.max(right_depth)
            }
            None => 0,
        }
    }
    
    fn main() {
        let root = Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 20,
                left: Some(Box::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                })),
            })),
        }));
        println!("Maximum depth of binary tree: {}", max_depth(root.as_ref()));
    }
    