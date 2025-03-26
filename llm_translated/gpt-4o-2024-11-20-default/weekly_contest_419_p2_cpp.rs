```rust
use std::collections::VecDeque;
use std::io;
use std::cmp;

// Definition of the TreeNode structure
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// Solution struct and its method
struct Solution;

impl Solution {
    fn kth_largest_perfect_subtree(root: Option<&Box<TreeNode>>, k: usize) -> i32 {
        // Helper vector to store heights of perfect subtrees
        let mut hs = Vec::new();

        // Recursive DFS function using closures
        fn dfs(node: Option<&Box<TreeNode>>, hs: &mut Vec<i32>) -> i32 {
            if let Some(n) = node {
                let left_h = dfs(n.left.as_deref(), hs);
                let right_h = dfs(n.right.as_deref(), hs);

                // If either subtree is invalid, return -1
                if left_h < 0 || left_h != right_h {
                    return -1;
                }

                let height = left_h + 1;
                hs.push(height);
                height
            } else {
                // If the node is None, return 0
                0
            }
        }

        dfs(root, &mut hs);

        if k > hs.len() {
            return -1;
        }

        // Use nth_element to find the k-th largest height
        let target_idx = hs.len() - k;
        hs.select_nth_unstable(target_idx);
        (1 << hs[target_idx]) - 1
    }
}

// Helper functions to create and free the tree
fn create_tree(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }

    let root = Box::new(TreeNode::new(arr[0]));
    let mut queue = VecDeque::new();
    queue.push_back(root);

    let mut i = 1;
    while i < arr.len() {
        if let Some(current) = queue.pop_front() {
            let mut current_ref = current;

            if arr[i] != 0 {
                let left = Box::new(TreeNode::new());
            }
        }
    }
}
``