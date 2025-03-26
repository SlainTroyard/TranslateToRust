use std::collections::VecDeque;
use std::io::{self, BufRead};

// Define the TreeNode structure
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

    fn with_children(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Self {
        TreeNode { val, left, right }
    }
}

// Define the Solution structure
struct Solution;

impl Solution {
    fn kth_largest_perfect_subtree(root: Option<Box<TreeNode>>, k: usize) -> i32 {
        let mut hs = Vec::new();

        // Recursive DFS function to calculate the height of perfect subtrees
        fn dfs(node: &Option<Box<TreeNode>>, hs: &mut Vec<i32>) -> i32 {
            if let Some(node) = node {
                let left_h = dfs(&node.left, hs);
                let right_h = dfs(&node.right, hs);
                if left_h < 0 || left_h != right_h {
                    return -1;
                }
                let height = left_h + 1;
                hs.push(height);
                height
            } else {
                0
            }
        }

        dfs(&root, &mut hs);

        if k > hs.len() {
            return -1;
        }

        // Find the k-th largest height
        hs.sort_unstable();
        let kth_height = hs[hs.len() - k];

        // Calculate the number of nodes in a perfect binary tree of height kth_height
        (1 << kth_height) - 1
    }
}

// Function to create a binary tree from a vector of integers
fn create_tree(arr: Vec<i32>) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }

    let root = Some(Box::new(TreeNode::new(arr[0])));
    let mut q = VecDeque::new();
    q.push_back(root.as_ref().unwrap());

    let mut i = 1;
    while i < arr.len() {
        if let Some(current) = q.pop_front() {
            if arr[i] != 0 {
                current.left = Some(Box::new(TreeNode::new(arr[i])));
                q.push_back(current.left.as_ref().unwrap());
            }
            i += 1;

            if i < arr.len() && arr[i] != 0 {
                current.right = Some(Box::new(TreeNode::new(arr[i])));
                q.push_back(current.right.as_ref().unwrap());
            }
            i += 1;
        }
    }

    root
}

// Main function to handle input/output
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and n
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();

    // Read the array of integers
    let second_line = lines.next().unwrap().unwrap();
    let arr: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Create the binary tree
    let root = create_tree(arr);

    // Compute the k-th largest perfect subtree
    let result = Solution::kth_largest_perfect_subtree(root, k);

    // Print the result
    println!("{}", result);
}