// Problem: Weekly Contest 419 Problem 2
// Translated from the provided C++ code to Rust, preserving the exact logic and I/O format.

use std::collections::VecDeque;
use std::io::{self, BufRead};

// A simple tree node in Rust, using Box to own children.
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

// Creates a binary tree from the given array, where 0 represents a missing child.
// This replicates the C++ BFS approach exactly.
fn create_tree(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }

    let mut root = Box::new(TreeNode::new(arr[0]));
    let mut queue = VecDeque::new();
    // We push a mutable reference of the root into the queue so we can modify it.
    queue.push_back(&mut root);

    // In the C++ code, we iterate i in steps of 2, pairing left and right children.
    let mut i = 1;
    while i < arr.len() {
        // Pop the front node from the queue.
        let current = match queue.pop_front() {
            Some(node) => node,
            None => break,
        };

        // If this element is non-zero, create a left child.
        if arr[i] != 0 {
            current.left = Some(Box::new(TreeNode::new(arr[i])));
            // Push the newly created left child into the queue.
            queue.push_back(current.left.as_mut().unwrap());
        }

        // If there's a next element for the right child and it is non-zero, create it.
        if i + 1 < arr.len() && arr[i + 1] != 0 {
            current.right = Some(Box::new(TreeNode::new(arr[i + 1])));
            // Push the newly created right child into the queue.
            queue.push_back(current.right.as_mut().unwrap());
        }

        i += 2;
    }

    Some(root)
}

// DFS helper that computes the height of a "perfect" subtree and records it if valid.
// Returns -1 if the subtree at node is not perfect, otherwise returns the height.
fn dfs(node: Option<&Box<TreeNode>>, heights: &mut Vec<i32>) -> i32 {
    match node {
        None => 0, // An empty subtree has height 0 and is considered perfect here
        Some(n) => {
            let left_h = dfs(n.left.as_ref(), heights);
            let right_h = dfs(n.right.as_ref(), heights);

            // If left subtree was invalid (-1) or heights don't match, return -1
            if left_h < 0 || left_h != right_h {
                return -1;
            }

            // This subtree is perfect, record its height
            heights.push(left_h + 1);
            left_h + 1
        }
    }
}

// Finds the k-th largest perfect subtree in the tree based on the subtree's height.
// Then uses the height h to compute the size of that perfect subtree: (1 << h) - 1.
fn kth_largest_perfect_subtree(root: Option<&Box<TreeNode>>, k: usize) -> i32 {
    let mut heights = Vec::new();
    dfs(root, &mut heights);

    // If k is larger than the number of perfect subtrees found, return -1
    if k > heights.len() {
        return -1;
    }

    // Select the (len - k)th element in sorted order to get the k-th largest
    // In C++, std::nth_element was used; in Rust, select_nth_unstable does the same.
    let len = heights.len();
    heights.select_nth_unstable(len - k);
    let h = heights[len - k];

    // The size of a complete binary tree of height h is (1 << h) - 1
    (1 << h) - 1
}

// In Rust, memory is automatically reclaimed when values go out of scope,
// so an explicit freeTree function is not required. We'll define a no-op
// function to mirror the C++ code structure if desired.
fn free_tree(_root: Option<Box<TreeNode>>) {
    // No action needed; Rust automatically frees memory.
}

fn main() -> io::Result<()> {
    // We will read input from stdin exactly as the C++ code does.
    // The format is: 
    // 1) Two integers k and n.
    // 2) Followed by n integers describing the tree.
    // 3) Output a single integer, the result of kthLargestPerfectSubtree.

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut tokens = Vec::new();

    // Read enough tokens until we at least get k and n.
    while tokens.len() < 2 {
        if let Some(line) = lines.next() {
            let line = line?;
            for t in line.split_whitespace() {
                tokens.push(t.to_string());
            }
        } else {
            // No more input, break to avoid potential loop
            break;
        }
    }

    // Parse k and n
    let k = tokens[0].parse::<usize>().unwrap();
    let n = tokens[1].parse::<usize>().unwrap();

    // Now read n more integers for the tree
    while tokens.len() < (2 + n) {
        if let Some(line) = lines.next() {
            let line = line?;
            for t in line.split_whitespace() {
                tokens.push(t.to_string());
            }
        } else {
            break;
        }
    }

    let arr: Vec<i32> = tokens[2..2 + n].iter().map(|x| x.parse().unwrap()).collect();

    // Build the tree
    let root = create_tree(&arr);

    // Solve the problem with the same logic
    let answer = kth_largest_perfect_subtree(root.as_ref(), k);

    // Print the result
    println!("{}", answer);

    // Free the tree (does nothing in Rust but reflects the same structure)
    free_tree(root);

    Ok(())
}