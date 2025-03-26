use std::io::{self, BufRead};
use std::cmp;
use std::collections::VecDeque;

// Definition for a binary tree node.
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

static mut ARR: Vec<i32> = Vec::new();

fn tree_judge(root: &Option<Box<TreeNode>>, floor: i32) -> i32 {
    match root {
        Some(node) => {
            let left_floor = if let Some(left) = &node.left {
                tree_judge(&Some(left.clone()), floor + 1)
            } else {
                0
            };

            let right_floor = if let Some(right) = &node.right {
                tree_judge(&Some(right.clone()), floor + 1)
            } else {
                0
            };

            let current_floor = if node.left.is_none() && node.right.is_none() {
                floor
            } else {
                0
            };

            if left_floor == right_floor && right_floor >= floor {
                unsafe {
                    ARR.push((2i32.pow((right_floor - floor + 1) as u32)) - 1);
                }
                left_floor
            } else {
                current_floor
            }
        }
        None => 0,
    }
}

fn quick_sort(l: usize, r: usize) {
    if l >= r {
        return;
    }

    let mut l_t = l - 1;
    let mut r_t = r + 1;
    let mid_val = unsafe { ARR[(l + r) / 2] };

    while l_t < r_t {
        l_t += 1;
        while unsafe { ARR[l_t] } < mid_val {
            l_t += 1;
        }

        r_t -= 1;
        while unsafe { ARR[r_t] } > mid_val {
            r_t -= 1;
        }

        if l_t < r_t {
            unsafe {
                ARR.swap(l_t, r_t);
            }
        }
    }

    quick_sort(l, r_t);
    quick_sort(r_t + 1, r);
}

fn kth_largest_perfect_subtree(root: Option<Box<TreeNode>>, k: i32) -> i32 {
    unsafe {
        ARR.clear();
    }
    tree_judge(&root, 1);

    unsafe {
        quick_sort(0, ARR.len() - 1);
        if (ARR.len() as i32) - k < 0 {
            -1
        } else {
            ARR[ARR.len() - k as usize]
        }
    }
}

fn create_tree(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }

    let mut root = Some(Box::new(TreeNode::new(arr[0])));
    let mut queue = VecDeque::new();
    queue.push_back(root.as_mut());

    for i in (1..arr.len()).step_by(2) {
        if let Some(current) = queue.pop_front() {
            if arr[i] != 0 {
                current.left = Some(Box::new(TreeNode::new(arr[i])));
                queue.push_back(current.left.as_mut());
            }

            if i + 1 < arr.len() && arr[i + 1] != 0 {
                current.right = Some(Box::new(TreeNode::new(arr[i + 1])));
                queue.push_back(current.right.as_mut());
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

fn print_tree(root: &Option<Box<TreeNode>>) {
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        if let Some(node) = queue.pop_front() {
            match node {
                Some(n) => {
                    print!("{} ", n.val);
                    queue.push_back(&n.left);
                    queue.push_back(&n.right);
                }
                None => print!("null "),
            }
        }
    }
    println!();
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let k: i32 = lines.next().unwrap()?.trim().parse().unwrap();
    let arr_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let arr: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let root = create_tree(&arr);
    let result = kth_largest_perfect_subtree(root.clone(), k);
    println!("{}", result);

    // Free the tree
    let mut root = root;
    free_tree(&mut root);

    Ok(())
}