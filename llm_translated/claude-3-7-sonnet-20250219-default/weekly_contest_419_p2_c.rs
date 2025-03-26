use std::cell::RefCell;
use std::io::{self, BufRead};
use std::rc::Rc;
use std::collections::VecDeque;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
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

fn tree_judge(root: &Option<Rc<RefCell<TreeNode>>>, floor: i32, arr: &mut Vec<i32>) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let left_floor;
        let right_floor;

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
            arr.push(2_i32.pow((right_floor - floor + 1) as u32) - 1);
            return left_floor;
        }

        return 0;
    }
    0
}

fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut arr = Vec::with_capacity(10000);
    tree_judge(&root, 1, &mut arr);

    arr.sort_unstable(); // Using Rust's built-in sort which is typically faster than manual quicksort
    
    if arr.len() as i32 - k < 0 {
        return -1;
    }
    
    arr[arr.len() - k as usize]
}

fn create_tree(values: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0])));
    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));

    let mut i = 1;
    while i < values.len() {
        if let Some(current) = queue.pop_front() {
            // Left child
            if i < values.len() && values[i] != 0 {
                let left = Rc::new(RefCell::new(TreeNode::new(values[i])));
                current.borrow_mut().left = Some(Rc::clone(&left));
                queue.push_back(left);
            }
            i += 1;

            // Right child
            if i < values.len() && values[i] != 0 {
                let right = Rc::new(RefCell::new(TreeNode::new(values[i])));
                current.borrow_mut().right = Some(Rc::clone(&right));
                queue.push_back(right);
            }
            i += 1;
        }
    }

    Some(root)
}

// Function to print the tree level by level (for debugging)
fn print_tree(root: &Option<Rc<RefCell<TreeNode>>>) {
    if root.is_none() {
        return;
    }

    let mut queue = VecDeque::new();
    queue.push_back(root.as_ref().unwrap().clone());

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        let node_ref = node.borrow();
        
        print!("{} ", node_ref.val);
        
        if let Some(left) = &node_ref.left {
            queue.push_back(left.clone());
        } else {
            print!("null ");
        }
        
        if let Some(right) = &node_ref.right {
            queue.push_back(right.clone());
        } else {
            print!("null ");
        }
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    
    // Read k
    let k = lines.next().unwrap().trim().parse::<i32>().unwrap();
    
    // Read array size
    let arr_size = lines.next().unwrap().trim().parse::<usize>().unwrap();
    
    // Read array elements
    let arr: Vec<i32> = lines.next().unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    
    // Create the tree
    let root = create_tree(&arr);
    
    // Get the result
    let result = kth_largest_perfect_subtree(root, k);
    
    // Print the result
    println!("{}", result);
}