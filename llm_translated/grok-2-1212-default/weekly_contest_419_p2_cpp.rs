// Problem: Weekly Contest 419 Problem 2

use std::io::{self, BufRead};
use std::cmp::Ordering;

// Definition for a binary tree node
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
    pub fn kth_largest_perfect_subtree(root: Option<Box<TreeNode>>, k: i32) -> i32 {
        let mut hs = Vec::new();
        fn dfs(node: &Option<Box<TreeNode>>, hs: &mut Vec<i32>) -> i32 {
            match node {
                None => 0,
                Some(node) => {
                    let left_h = dfs(&node.left, hs);
                    let right_h = dfs(&node.right, hs);
                    if left_h < 0 || left_h != right_h {
                        -1
                    } else {
                        hs.push(left_h + 1);
                        left_h + 1
                    }
                }
            }
        }
        dfs(&root, &mut hs);

        if k as usize > hs.len() {
            return -1;
        }
        hs.select_nth_unstable_by(hs.len() - k as usize, |a, b| b.cmp(a));
        (1 << hs[hs.len() - k as usize]) - 1
    }
}

fn create_tree(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }
    
    let mut root = Some(Box::new(TreeNode::new(arr[0])));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(&mut root);

    for chunk in arr[1..].chunks(2) {
        if let Some(Some(node)) = queue.pop_front() {
            if chunk[0] != 0 {
                node.left = Some(Box::new(TreeNode::new(chunk[0])));
                queue.push_back(&mut node.left);
            }
            if chunk.len() > 1 && chunk[1] != 0 {
                node.right = Some(Box::new(TreeNode::new(chunk[1])));
                queue.push_back(&mut node.right);
            }
        }
    }

    root
}

fn free_tree(root: &mut Option<Box<TreeNode>>) {
    if let Some(node) = root.take() {
        free_tree(&mut node.left);
        free_tree(&mut node.right);
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap()?;
    let arr: Vec<i32> = second_line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut root = create_tree(&arr);
    let solution = Solution;
    println!("{}", solution.kth_largest_perfect_subtree(root, k));

    free_tree(&mut root);
    Ok(())
}