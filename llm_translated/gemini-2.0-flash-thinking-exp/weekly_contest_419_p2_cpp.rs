use std::cell::RefCell;
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut hs: Vec<i32> = Vec::new();
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, hs: &mut Vec<i32>) -> i32 {
            if node.is_none() {
                return 0;
            }
            let node_ref = node.as_ref().unwrap().borrow();
            let left_h = dfs(node_ref.left.clone(), hs);
            let right_h = dfs(node_ref.right.clone(), hs);
            if left_h < 0 || left_h != right_h {
                return -1;
            }
            hs.push(left_h + 1);
            return left_h + 1;
        }
        dfs(root, &mut hs);

        if k as usize > hs.len() {
            return -1;
        }
        if hs.is_empty() {
            return -1; // Handle empty hs to avoid panic in nth_element
        }
        let n = hs.len();
        let kth_index = n - k as usize;

        // Partial sort to find the k-th largest element
        let mut partition_point = kth_index;
        let mut l = 0;
        let mut r = n;

        while l < r {
            let pivot_index = l + (r - l) / 2;
            hs.swap(pivot_index, r - 1);
            let mut store_index = l;
            for i in l..r - 1 {
                if hs[i] <= hs[r - 1] {
                    hs.swap(store_index, i);
                    store_index += 1;
                }
            }
            hs.swap(store_index, r - 1);

            if store_index == kth_index {
                partition_point = store_index;
                break;
            } else if store_index < kth_index {
                l = store_index + 1;
            } else {
                r = store_index;
            }
        }

        if partition_point < hs.len() {
            return (1 << hs[partition_point]) - 1;
        } else {
            return -1; // Should not reach here if k is within hs.size()
        }
    }
}

fn create_tree(arr: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if arr.is_empty() {
        return None;
    }

    let root = Some(Rc::new(RefCell::new(TreeNode::new(arr[0]))));
    let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    q.push_back(root.clone().unwrap());

    let mut i = 1;
    while i < arr.len() {
        if let Some(current) = q.pop_front() {
            if arr[i] != 0 {
                let left_node = Some(Rc::new(RefCell::new(TreeNode::new(arr[i]))));
                current.borrow_mut().left = left_node.clone();
                q.push_back(left_node.unwrap());
            }
            i += 1;
            if i < arr.len() && arr[i] != 0 {
                let right_node = Some(Rc::new(RefCell::new(TreeNode::new(arr[i]))));
                current.borrow_mut().right = right_node.clone();
                q.push_back(right_node.unwrap());
            }
            i += 1;
        } else {
            break; // Should not happen unless input is malformed
        }
    }
    root
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    let n: i32 = parts.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap();
    let arr: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let root = create_tree(&arr);
    let solution = Solution {};
    println!("{}", solution.kth_largest_perfect_subtree(root, k));
}