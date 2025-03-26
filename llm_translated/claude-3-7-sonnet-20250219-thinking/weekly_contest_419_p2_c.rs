use std::io::{self, BufRead};
use std::cmp::Ordering;

// Define the TreeNode structure
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    // Create a new TreeNode with a given value
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// Judge function to find perfect subtrees
fn tree_judge(root: &TreeNode, floor: i32, arr: &mut Vec<i32>) -> i32 {
    let mut left_floor = 0;
    let mut right_floor = 0;

    if root.left.is_some() && root.right.is_some() {
        left_floor = tree_judge(root.left.as_ref().unwrap(), floor + 1, arr);
        right_floor = tree_judge(root.right.as_ref().unwrap(), floor + 1, arr);
    } else if root.left.is_some() {
        left_floor = tree_judge(root.left.as_ref().unwrap(), floor + 1, arr);
        right_floor = 0;
    } else if root.right.is_some() {
        left_floor = 0;
        right_floor = tree_judge(root.right.as_ref().unwrap(), floor + 1, arr);
    } else {
        left_floor = floor;
        right_floor = floor;
    }

    if left_floor == right_floor && right_floor >= floor {
        arr.push((2_i32.pow((right_floor - floor + 1) as u32)) - 1);
        return left_floor;
    }

    0
}

// Quick sort implementation
fn quick_sort(arr: &mut [i32], l: usize, r: usize) {
    if l >= r {
        return;
    }

    let mut l_t = l as i32 - 1;
    let mut r_t = r as i32 + 1;
    let mid_val = arr[(l + r) >> 1];

    while l_t < r_t {
        loop {
            l_t += 1;
            if arr[l_t as usize] >= mid_val {
                break;
            }
        }

        loop {
            r_t -= 1;
            if arr[r_t as usize] <= mid_val {
                break;
            }
        }

        if l_t < r_t {
            arr.swap(l_t as usize, r_t as usize);
        }
    }

    quick_sort(arr, l, r_t as usize);
    if r_t as usize + 1 <= r {
        quick_sort(arr, r_t as usize + 1, r);
    }
}

// Main function to find kth largest perfect subtree
fn kth_largest_perfect_subtree(root: &TreeNode, k: i32) -> i32 {
    let mut arr = Vec::with_capacity(10000);
    tree_judge(root, 1, &mut arr);

    if !arr.is_empty() {
        quick_sort(&mut arr, 0, arr.len() - 1);
        let idx = arr.len() as i32 - k;
        if idx < 0 {
            return -1;
        }
        return arr[idx as usize];
    }
    -1
}

// Create a binary tree from an array
fn create_tree(arr: &[i32], size: usize) -> Option<Box<TreeNode>> {
    if size == 0 {
        return None;
    }

    let root = Box::new(TreeNode::new(arr[0]));
    let mut queue = Vec::with_capacity(size);
    queue.push(root);

    let mut front = 0;
    let mut i = 1;
    while i < size {
        let current = &mut queue[front];
        front += 1;

        // Add left child
        if arr[i] != 0 {
            let left_node = Box::new(TreeNode::new(arr[i]));
            current.left = Some(left_node);
            queue.push(current.left.as_mut().unwrap().clone());
        }

        // Add right child if there's an element available
        i += 1;
        if i < size && arr[i] != 0 {
            let right_node = Box::new(TreeNode::new(arr[i]));
            current.right = Some(right_node);
            queue.push(current.right.as_mut().unwrap().clone());
        }
        i += 1;
    }

    Some(queue[0].clone())
}

// Print tree for debugging
fn print_tree(root: &TreeNode) {
    let mut queue = Vec::with_capacity(10000);
    queue.push(Some(Box::new(TreeNode {
        val: root.val,
        left: root.left.clone(),
        right: root.right.clone(),
    })));

    while !queue.is_empty() {
        let node = queue.remove(0);
        match node {
            None => print!("null "),
            Some(n) => {
                print!("{} ", n.val);
                queue.push(n.left.clone());
                queue.push(n.right.clone());
            }
        }
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k
    let k: i32 = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    // Read array size
    let arr_size: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    // Read array elements
    let arr: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Create the tree and find kth largest perfect subtree
    if let Some(root) = create_tree(&arr, arr_size) {
        println!("{}", kth_largest_perfect_subtree(&root, k));
    } else {
        println!("-1");
    }
}