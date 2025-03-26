use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::{self, BufRead};

// Definition for a binary tree node.
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
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
    fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut heights: Vec<i32> = Vec::new();
        
        // Define a recursive dfs function to compute heights
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, heights: &mut Vec<i32>) -> i32 {
            match node {
                None => 0,
                Some(node_rc) => {
                    let node = node_rc.borrow();
                    let left_h = dfs(&node.left, heights);
                    let right_h = dfs(&node.right, heights);
                    
                    if left_h < 0 || left_h != right_h {
                        return -1;
                    }
                    
                    heights.push(left_h + 1);
                    left_h + 1
                }
            }
        }
        
        dfs(&root, &mut heights);
        
        if k as usize > heights.len() {
            return -1;
        }
        
        // Equivalent to nth_element in C++
        let idx = heights.len() - k as usize;
        heights.select_nth_unstable(idx);
        
        // Calculate 2^h - 1 (number of nodes in a perfect binary tree of height h)
        (1 << heights[idx]) - 1
    }
}

fn create_tree(arr: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if arr.is_empty() {
        return None;
    }
    
    let root = Rc::new(RefCell::new(TreeNode::new(arr[0])));
    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));
    
    let mut i = 1;
    while i < arr.len() {
        if let Some(current) = queue.pop_front() {
            // Left child
            if arr[i] != 0 {
                let left_child = Rc::new(RefCell::new(TreeNode::new(arr[i])));
                current.borrow_mut().left = Some(Rc::clone(&left_child));
                queue.push_back(left_child);
            }
            
            // Right child
            i += 1;
            if i < arr.len() && arr[i] != 0 {
                let right_child = Rc::new(RefCell::new(TreeNode::new(arr[i])));
                current.borrow_mut().right = Some(Rc::clone(&right_child));
                queue.push_back(right_child);
            }
            
            i += 1;
        }
    }
    
    Some(root)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read k and n
    let mut input = lines.next().unwrap().unwrap();
    let mut parts: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let k = parts[0];
    let n = parts[1];
    
    // Read array elements
    input = lines.next().unwrap().unwrap();
    let arr: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let root = create_tree(&arr);
    let solution = Solution::kth_largest_perfect_subtree(root, k);
    
    println!("{}", solution);
}