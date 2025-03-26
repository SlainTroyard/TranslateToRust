use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::cmp::Ordering;

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
        let mut left_floor = 0;
        let mut right_floor = 0;

        if node.left.is_some() && node.right.is_some() {
            left_floor = tree_judge(&node.left, floor + 1, arr);
            right_floor = tree_judge(&node.right, floor + 1, arr);
        } else if node.left.is_some() {
            left_floor = tree_judge(&node.left, floor + 1, arr);
            right_floor = 0;
        } else if node.right.is_some() {
            left_floor = 0;
            right_floor = tree_judge(&node.right, floor + 1, arr);
        } else {
            left_floor = floor;
            right_floor = floor;
        }

        if left_floor == right_floor && right_floor >= floor {
            arr.push((2_i32.pow((right_floor - floor + 1) as u32) - 1) as i32);
            return left_floor;
        }
    }
    0
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let mid_val = arr[arr.len() / 2];
    let (mut left, mut right) = (0, arr.len() - 1);

    while left <= right {
        while arr[left] < mid_val {
            left += 1;
        }
        while arr[right] > mid_val {
            right = right.saturating_sub(1);
        }
        if left <= right {
            arr.swap(left, right);
            left += 1;
            right = right.saturating_sub(1);
        }
    }

    quick_sort(&mut arr[..right + 1]);
    quick_sort(&mut arr[left..]);
}

fn kth_largest_perfect_subtree(root: &Option<Box<TreeNode>>, k: usize) -> i32 {
    let mut arr = Vec::new();
    tree_judge(root, 1, &mut arr);

    quick_sort(&mut arr);
    if arr.len() < k {
        -1
    } else {
        arr[arr.len() - k]
    }
}

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
            if arr[i] != 0 {
                let left_node = Box::new(TreeNode::new(arr[i]));
                current.left = Some(left_node.clone());
                queue.push_back(left_node);
            }
            i += 1;

            if i < arr.len() && arr[i] != 0 {
                let right_node = Box::new(TreeNode::new(arr[i]));
                current.right = Some(right_node.clone());
                queue.push_back(right_node);
            }
            i += 1;
        }
    }

    Some(queue.pop_front().unwrap())
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let k: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let arr_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let arr: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let root = create_tree(&arr);
    let result = kth_largest_perfect_subtree(&root, k);
    println!("{}", result);
}