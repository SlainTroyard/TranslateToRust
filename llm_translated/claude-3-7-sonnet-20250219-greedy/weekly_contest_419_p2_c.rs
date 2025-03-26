use std::cell::RefCell;
use std::io::{self, BufRead};
use std::rc::Rc;

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
        let (mut left_floor, mut right_floor) = (0, 0);
        
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
            arr.push((2_i32.pow((right_floor - floor + 1) as u32)) - 1);
            return left_floor;
        }

        return 0;
    }
    0
}

fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut arr = Vec::with_capacity(10000);
    tree_judge(&root, 1, &mut arr);

    arr.sort_unstable();
    
    if arr.len() < k as usize {
        return -1;
    }

    arr[arr.len() - k as usize]
}

fn create_tree(arr: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if arr.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(arr[0])));
    let mut queue = Vec::with_capacity(arr.len());
    queue.push(Rc::clone(&root));

    let mut i = 1;
    while i < arr.len() {
        if let Some(current) = queue.remove(0) {
            // Left child
            if i < arr.len() && arr[i] != 0 {
                let left_node = Rc::new(RefCell::new(TreeNode::new(arr[i])));
                current.borrow_mut().left = Some(Rc::clone(&left_node));
                queue.push(left_node);
            }
            i += 1;

            // Right child
            if i < arr.len() && arr[i] != 0 {
                let right_node = Rc::new(RefCell::new(TreeNode::new(arr[i])));
                current.borrow_mut().right = Some(Rc::clone(&right_node));
                queue.push(right_node);
            }
            i += 1;
        }
    }

    Some(root)
}

// Function to print the tree in level order (for debugging)
fn print_tree(root: &Option<Rc<RefCell<TreeNode>>>) {
    let mut queue = Vec::new();
    if let Some(node) = root {
        queue.push(Rc::clone(node));
    }
    
    while !queue.is_empty() {
        let node = queue.remove(0);
        let node_ref = node.borrow();
        
        print!("{} ", node_ref.val);
        
        if let Some(left) = &node_ref.left {
            queue.push(Rc::clone(left));
        } else {
            print!("null ");
        }
        
        if let Some(right) = &node_ref.right {
            queue.push(Rc::clone(right));
        } else {
            print!("null ");
        }
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read k
    let k: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read array size
    let arr_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read array elements
    let arr: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let root = create_tree(&arr);
    
    println!("{}", kth_largest_perfect_subtree(root, k));
}