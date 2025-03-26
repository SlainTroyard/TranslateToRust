```rust
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

fn tree_judge(root: &Option<Box<TreeNode>>, floor: i32, arr: &mut Vec<i32>) -> i32 {
    if let Some(node) = root {
        let left_floor = if node.left.is_some() {
            tree_judge(&node.left, floor + 1, arr)
        } else {
            floor
        };
        let right_floor = if node.right.is_some() {
            tree_judge(&node.right, floor + 1, arr)
        } else {
            floor
        };
        if left_floor == right_floor && right_floor >= floor {
            arr.push((2_i32.pow((right_floor - floor + 1) as u32)) - 1);
            return left_floor;
        }
    }
    0
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let (mut l, mut r) = (0, arr.len() - 1);
    let mid_val = arr[arr.len() / 2];
    while l <= r {
        while arr[l] < mid_val {
            l += 1;
        }
        while arr[r] > mid_val {
            if r == 0 {
                break;
            }
            r -= 1;
        }
        if l <= r {
            arr.swap(l, r);
            if l < arr.len() {
                l += 1;
            }
            if r > 0 {
                r -= 1;
            }
        }
    }
    quick_sort(&mut arr[0..=r]);
    quick_sort(&mut arr[l..]);
}

fn kth_largest_perfect_subtree(root: &Option<Box<TreeNode>>, k: usize) -> i32 {
    let mut arr = Vec::new();
    tree_judge(root, 1, &mut arr);
    if arr.len() < k {
        return -1;
    }
    quick_sort(&mut arr);
    arr[arr.len() - k]
}

fn create_tree(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }
    let root = Box::new(TreeNode::new(arr[0]));
    let mut queue = VecDeque::new();
    queue.push_back(root);
    let mut idx = 1;

    let mut final_root = None;

    while let Some(mut current_node) = queue.pop_front() {
        if idx < arr.len() && arr[idx] != 0 {
            current_node.left = Box<....
