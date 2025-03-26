use std::io::{self, BufRead};
use std::collections::VecDeque;

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
    fn kth_largest_perfect_subtree(root: &Option<Box<TreeNode>>, k: i32) -> i32 {
        let mut hs = Vec::new();
        
        // Define a recursive DFS function equivalent to the C++ lambda
        fn dfs(node: &Option<Box<TreeNode>>, hs: &mut Vec<i32>) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let left_h = dfs(&n.left, hs);
                    let right_h = dfs(&n.right, hs);
                    
                    if left_h < 0 || left_h != right_h {
                        return -1;
                    }
                    
                    hs.push(left_h + 1);
                    left_h + 1
                }
            }
        }
        
        dfs(root, &mut hs);
        
        if (k as usize) > hs.len() {
            return -1;
        }
        
        // Sort to find the kth largest height
        hs.sort_unstable();
        let height = hs[hs.len() - (k as usize)];
        
        // Calculate the size of a perfect tree with the given height
        (1 << height) - 1
    }
}

fn create_tree(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }
    
    let root = Box::new(TreeNode::new(arr[0]));
    let mut q = VecDeque::new();
    q.push_back(root);
    
    let mut i = 1;
    while i < arr.len() {
        if let Some(mut current) = q.pop_front() {
            // Process left child
            if i < arr.len() {
                if arr[i] != 0 {
                    let left_node = Box::new(TreeNode::new(arr[i]));
                    current.left = Some(left_node);
                    q.push_back(current.left.as_mut().unwrap());
                }
                i += 1;
            }
            
            // Process right child
            if i < arr.len() {
                if arr[i] != 0 {
                    let right_node = Box::new(TreeNode::new(arr[i]));
                    current.right = Some(right_node);
                    q.push_back(current.right.as_mut().unwrap());
                }
                i += 1;
            }
            
            if i >= arr.len() {
                return Some(current);
            } else {
                q.push_back(current);
            }
        }
    }
    
    q.pop_front().map(|node| node)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read k and n values from the first line
    let line = lines.next().unwrap()?;
    let mut parts = line.split_whitespace();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    let n: i32 = parts.next().unwrap().parse().unwrap();
    
    // Read array values from the second line
    let line = lines.next().unwrap()?;
    let arr: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .take(n as usize)  // Ensure we only take n elements
        .collect();
    
    let root = create_tree(&arr);
    let result = Solution::kth_largest_perfect_subtree(&root, k);
    
    println!("{}", result);
    
    Ok(())
}