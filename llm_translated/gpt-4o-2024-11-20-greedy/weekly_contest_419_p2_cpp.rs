use std::collections::VecDeque;
use std::io::{self, Write};

// Definition for a binary tree node.
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

struct Solution;

impl Solution {
    pub fn kth_largest_perfect_subtree(root: Option<Box<TreeNode>>, k: usize) -> i32 {
        let mut hs = Vec::new();

        // Helper function for DFS
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

        // Find the k-th largest element
        hs.select_nth_unstable(hs.len() - k);
        let h = hs[hs.len() - k];
        (1 << h) - 1
    }
}

// Function to create a binary tree from a vector
fn create_tree(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }

    let root = Box::new(TreeNode::new(arr[0]));
    let mut queue = VecDeque::new();
    queue.push_back(root);

    let mut i = 1;
    while i < arr.len() {
        if let Some(mut current) = queue.pop_front() {
            if arr[i] != 0 {
                current.left = Some(Box::new(TreeNode::new(arr[i])));
                queue.push_back(current.left.as_ref().unwrap().clone());
            }
            if i + 1 < arr.len() && arr[i + 1] != 0 {
                current.right = Some(Box::new(TreeNode::new(arr[i + 1])));
                queue.push_back(current.right.as_ref().unwrap().clone());
            }
            i += 2;
        }
    }

    Some(root)
}

// Function to free the tree (not strictly necessary in Rust due to ownership and drop semantics)
fn free_tree(_root: Option<Box<TreeNode>>) {
    // Rust automatically handles memory cleanup, so this function is a no-op.
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let k: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let root = create_tree(&arr);
    let solution = Solution;
    let result = solution.kth_largest_perfect_subtree(root, k);

    println!("{}", result);

    // Free the tree (not strictly necessary in Rust)
    free_tree(root);
}