use std::io::{self, BufRead};
use std::collections::VecDeque;

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
    fn kth_largest_perfect_subtree(root: Option<Box<TreeNode>>, k: usize) -> i32 {
        let mut heights = Vec::new();

        fn dfs(node: &Option<Box<TreeNode>>, heights: &mut Vec<usize>) -> i32 {
            if let Some(n) = node {
                let left_h = dfs(&n.left, heights);
                let right_h = dfs(&n.right, heights);
                if left_h < 0 || left_h != right_h {
                    return -1;
                }
                heights.push(left_h as usize + 1);
                left_h + 1
            } else {
                0
            }
        }

        dfs(&root, &mut heights);

        if k > heights.len() {
            return -1;
        }
        heights.sort_unstable();
        (1 << heights[heights.len() - k]) - 1
    }
}

fn create_tree(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }

    let mut root = Some(Box::new(TreeNode::new(arr[0])));
    let mut q = VecDeque::new();
    q.push_back(root.as_mut().unwrap());

    for i in (1..arr.len()).step_by(2) {
        if let Some(current) = q.pop_front() {
            if arr[i] != 0 {
                current.left = Some(Box::new(TreeNode::new(arr[i])));
                q.push_back(current.left.as_mut().unwrap());
            }

            if i + 1 < arr.len() && arr[i + 1] != 0 {
                current.right = Some(Box::new(TreeNode::new(arr[i + 1])));
                q.push_back(current.right.as_mut().unwrap());
            }
        }
    }

    root
}

fn free_tree(root: &Option<Box<TreeNode>>) {
    if let Some(node) = root {
        free_tree(&node.left);
        free_tree(&node.right);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let arr: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let root = create_tree(&arr);
    let solution = Solution;
    println!("{}", solution.kth_largest_perfect_subtree(root, k));

    free_tree(&root);
}