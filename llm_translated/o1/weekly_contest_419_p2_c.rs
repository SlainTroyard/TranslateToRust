// Translation of LeetCode Weekly Contest 419 Problem 2 from C to Rust

// -----------------------
// START OF TRANSLATED CODE
// -----------------------

use std::collections::VecDeque;
use std::io::{self, BufRead};

// A binary tree node.
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

// Equivalent to the C code's tree_judge(root, floor) function.
// It traverses the tree and records the size of each "perfect subtree"
// (if left_floor == right_floor && right_floor >= floor) in a shared vector.
fn tree_judge(node: &Option<Box<TreeNode>>, floor: i32, arr: &mut Vec<i32>) -> i32 {
    if let Some(n) = node {
        let (mut left_floor, mut right_floor) = (0, 0);

        // Check children just as the C code does.
        match (&n.left, &n.right) {
            (Some(_), Some(_)) => {
                left_floor = tree_judge(&n.left, floor + 1, arr);
                right_floor = tree_judge(&n.right, floor + 1, arr);
            }
            (Some(_), None) => {
                left_floor = tree_judge(&n.left, floor + 1, arr);
                right_floor = 0;
            }
            (None, Some(_)) => {
                left_floor = 0;
                right_floor = tree_judge(&n.right, floor + 1, arr);
            }
            (None, None) => {
                // Leaf node
                left_floor = floor;
                right_floor = floor;
            }
        }

        // If the subtree rooted at this node is "perfect," record its size:
        // The size is 2^(right_floor - floor + 1) - 1
        if left_floor == right_floor && right_floor >= floor {
            let height = (right_floor - floor + 1) as u32;
            let perfect_size = 2_i32.pow(height) - 1;
            arr.push(perfect_size);
            return left_floor;
        }

        0
    } else {
        0
    }
}

// A direct translation of the C code's quick_sort, using the same partition logic.
// This is intentionally written in a similar style, including indices and do-while-like loops.
fn quick_sort(arr: &mut [i32], l: i32, r: i32) {
    if l >= r {
        return;
    }
    let mut l_t = l - 1;
    let mut r_t = r + 1;
    let mid_val = arr[((l + r) >> 1) as usize];

    loop {
        loop {
            l_t += 1;
            if arr[l_t as usize] >= mid_val {
                break;
            }
        }
        loop {
            r_t -= 1;
            if arr[r_t as usize] <= mid_val {
                break;
            }
        }
        if l_t >= r_t {
            break;
        }
        arr.swap(l_t as usize, r_t as usize);
    }

    quick_sort(arr, l, r_t);
    quick_sort(arr, r_t + 1, r);
}

// Equivalent to the C code's kthLargestPerfectSubtree(root, k)
fn kthLargestPerfectSubtree(root: &Option<Box<TreeNode>>, k: i32) -> i32 {
    // In C, arr was a static array. Here we use a vector.
    let mut arr = Vec::new();
    tree_judge(root, 1, &mut arr);

    // Sort arr using the same quick_sort logic
    if !arr.is_empty() {
        quick_sort(&mut arr, 0, (arr.len() as i32) - 1);
    }

    // If k is out of range, return -1
    if arr.len() < k as usize {
        -1
    } else {
        // kth largest => arr[arr.len() - k]
        arr[arr.len() - k as usize]
    }
}

// Equivalent to the C code's create_tree(int *arr, int size)
// Builds a binary tree level by level, where 0 indicates no node.
fn create_tree(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }
    // Create the root
    let mut root = Box::new(TreeNode {
        val: arr[0],
        left: None,
        right: None,
    });

    // Use a queue to build the tree level by level
    let mut queue = VecDeque::new();
    queue.push_back(&mut root);

    let mut i = 1;
    while i < arr.len() {
        let current = queue.pop_front().unwrap();

        // If the left child is not 0, create it
        if arr[i] != 0 {
            let left_node = Box::new(TreeNode {
                val: arr[i],
                left: None,
                right: None,
            });
            current.left = Some(left_node);
            queue.push_back(current.left.as_mut().unwrap()); 
        }
        i += 1;

        // If the right child is not 0, create it
        if i < arr.len() && arr[i] != 0 {
            let right_node = Box::new(TreeNode {
                val: arr[i],
                left: None,
                right: None,
            });
            current.right = Some(right_node);
            queue.push_back(current.right.as_mut().unwrap());
        }
        i += 1;
    }

    Some(root)
}

// Equivalent to the C code's free_tree, which in Rust is unnecessary
// because memory is freed automatically. We'll keep it for completeness.
fn free_tree(_root: Option<Box<TreeNode>>) {
    // No action needed in Rust
}

// 用于调试打印二叉树，使用层次格式打印 (for debugging, level-order print).
// The C code does not call this in main, but we include it for completeness.
fn print_tree(root: &Option<Box<TreeNode>>) {
    let mut queue = VecDeque::new();
    queue.push_back(root.as_ref());

    while let Some(node_opt) = queue.pop_front() {
        match node_opt {
            None => {
                print!("null ");
            }
            Some(node) => {
                print!("{} ", node.val);
                queue.push_back(node.left.as_ref());
                queue.push_back(node.right.as_ref());
            }
        }
    }
}

// The main function replicates the same I/O logic as the C code:
// 1) Read k
// 2) Read arrSize
// 3) Read arr array
// 4) Create tree
// 5) Print kthLargestPerfectSubtree
// 6) Free memory (unnecessary in Rust, but we keep the structure)
fn main() {
    let stdin = io::stdin();
    let mut tokens = Vec::new();

    // Read all tokens from stdin (handling any spacing/line breaks as scanf would)
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        for w in line.split_whitespace() {
            tokens.push(w.parse::<i32>().unwrap());
        }
    }

    // Extract k and arrSize
    let mut idx = 0;
    let k = tokens[idx];
    idx += 1;
    let arr_size = tokens[idx];
    idx += 1;

    // Read and store the array
    let mut input_arr = Vec::new();
    for _ in 0..arr_size {
        input_arr.push(tokens[idx]);
        idx += 1;
    }

    // Create the tree
    let root = create_tree(&input_arr);

    // Compute and print the result
    // EXACT SAME stdout (just the number followed by newline)
    println!("{}", kthLargestPerfectSubtree(&root, k));

    // Free the tree (no-op in Rust, but included for structure consistency)
    free_tree(root);
}

// ---------------------
// END OF TRANSLATED CODE
// ---------------------