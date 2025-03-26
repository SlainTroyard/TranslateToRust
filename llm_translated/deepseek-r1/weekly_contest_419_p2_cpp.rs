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
    pub fn kth_largest_perfect_subtree(root: Option<Box<TreeNode>>, k: i32) -> i32 {
        let mut hs = Vec::new();
        Self::dfs(&root, &mut hs);

        let k = k as usize;
        if k > hs.len() {
            return -1;
        }

        let index = hs.len() - k;
        hs.select_nth_unstable(index);
        (1 << hs[index]) - 1
    }

    fn dfs(node: &Option<Box<TreeNode>>, hs: &mut Vec<i32>) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let left_h = Self::dfs(&n.left, hs);
                let right_h = Self::dfs(&n.right, hs);

                if left_h < 0 || left_h != right_h {
                    return -1;
                }

                let current_h = left_h + 1;
                hs.push(current_h);
                current_h
            }
        }
    }
}

fn create_tree(arr: Vec<i32>) -> Option<Box<TreeNode>> {
    if arr.is_empty() || arr[0] == 0 {
        return None;
    }

    let mut root = Box::new(TreeNode::new(arr[0]));
    let mut queue = VecDeque::new();
    queue.push_back(root.as_mut());

    let mut i = 1;
    while let Some(current) = queue.pop_front() {
        if i < arr.len() {
            let left_val = arr[i];
            i += 1;
            if left_val != 0 {
                current.left = Some(Box::new(TreeNode::new(left_val)));
                queue.push_back(current.left.as_mut().unwrap());
            }
        }

        if i < arr.len() {
            let right_val = arr[i];
            i += 1;
            if right_val != 0 {
                current.right = Some(Box::new(TreeNode::new(right_val)));
                queue.push_back(current.right.as_mut().unwrap());
            }
        }
    }

    Some(root)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    let n: usize = parts.next().unwrap().parse().unwrap();

    let arr: Vec<i32> = lines
        .flat_map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .take(n)
        .collect();

    let root = create_tree(arr);
    let result = Solution::kth_largest_perfect_subtree(root, k);
    println!("{}", result);
}