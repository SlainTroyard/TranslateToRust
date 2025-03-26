use std::io::{self, BufRead};
use std::cmp;
use std::mem;

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

// Global variables to store the sizes of perfect subtrees
static mut ARR: Vec<i32> = Vec::new();

fn tree_judge(root: &Option<Box<TreeNode>>, floor: i32) -> i32 {
    let mut left_floor = 0;
    let mut right_floor = 0;

    match root {
        Some(node) => {
            if let Some(left) = &node.left {
                left_floor = tree_judge(&Some(left.clone()), floor + 1);
            }
            if let Some(right) = &node.right {
                right_floor = tree_judge(&Some(right.clone()), floor + 1);
            }

            if node.left.is_none() && node.right.is_none() {
                left_floor = floor;
                right_floor = floor;
            } else if node.left.is_none() {
                left_floor = 0;
            } else if node.right.is_none() {
                right_floor = 0;
            }

            if left_floor == right_floor && right_floor >= floor {
                unsafe {
                    ARR.push((2i32.pow((right_floor - floor + 1) as u32)) - 1);
                }
                left_floor
            } else {
                0
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
        loop {
            l_t += 1;
            if l_t >= unsafe { ARR.len() } || unsafe { ARR[l_t] } >= mid_val {
                break;
            }
        }

        loop {
            r_t -= 1;
            if r_t == 0 || unsafe { ARR[r_t] } <= mid_val {
                break;
            }
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

    let mut root = Box::new(TreeNode::new(arr[0]));
    let mut queue = vec![&mut root];

    for chunk in arr[1..].chunks(2) {
        let current = queue.remove(0);

        if chunk[0] != 0 {
            current.left = Some(Box::new(TreeNode::new(chunk[0])));
            queue.push(current.left.as_mut().unwrap());
        }

        if chunk.len() > 1 && chunk[1] != 0 {
            current.right = Some(Box::new(TreeNode::new(chunk[1])));
            queue.push(current.right.as_mut().unwrap());
        }
    }

    Some(root)
}

fn free_tree(root: Option<Box<TreeNode>>) {
    if let Some(node) = root {
        free_tree(node.left);
        free_tree(node.right);
    }
}

fn print_tree(root: &Option<Box<TreeNode>>) {
    let mut queue = vec![root];
    while !queue.is_empty() {
        let node = queue.remove(0);
        match node {
            Some(n) => {
                print!("{} ", n.val);
                queue.push(&n.left);
                queue.push(&n.right);
            }
            None => print!("null "),
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
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let root = create_tree(&arr);

    println!("{}", kth_largest_perfect_subtree(root, k));

    Ok(())
}