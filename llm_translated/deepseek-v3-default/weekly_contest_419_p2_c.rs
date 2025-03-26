use std::collections::VecDeque;
use std::io;

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
static mut ARR_SIZE: usize = 0;

fn tree_judge(root: &TreeNode, floor: i32) -> i32 {
    let left_floor = if root.left.is_some() && root.right.is_some() {
        tree_judge(root.left.as_ref().unwrap(), floor + 1)
    } else if root.left.is_some() {
        tree_judge(root.left.as_ref().unwrap(), floor + 1)
    } else if root.right.is_some() {
        tree_judge(root.right.as_ref().unwrap(), floor + 1)
    } else {
        floor
    };

    let right_floor = if root.left.is_some() && root.right.is_some() {
        tree_judge(root.right.as_ref().unwrap(), floor + 1)
    } else if root.left.is_some() {
        0
    } else if root.right.is_some() {
        tree_judge(root.right.as_ref().unwrap(), floor + 1)
    } else {
        floor
    };

    if left_floor == right_floor && right_floor >= floor {
        unsafe {
            ARR.push(2i32.pow((right_floor - floor + 1) as u32) - 1);
            ARR_SIZE += 1;
        }
        return left_floor;
    }

    0
}

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
    quick_sort(arr, r_t as usize + 1, r);
}

fn kth_largest_perfect_subtree(root: &TreeNode, k: i32) -> i32 {
    unsafe {
        ARR = Vec::with_capacity(10000);
        ARR_SIZE = 0;
        tree_judge(root, 1);

        quick_sort(&mut ARR, 0, ARR_SIZE - 1);
        if ARR_SIZE as i32 - k < 0 {
            return -1;
        }

        ARR[ARR_SIZE - k as usize]
    }
}

fn create_tree(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }

    let root = Box::new(TreeNode::new(arr[0]));
    let mut queue = VecDeque::new();
    queue.push_back(&root);

    let mut i = 1;
    while !queue.is_empty() && i < arr.len() {
        let current = queue.pop_front().unwrap();

        if arr[i] != 0 {
            let left_node = Box::new(TreeNode::new(arr[i]));
            current.left = Some(left_node);
            queue.push_back(current.left.as_ref().unwrap());
        }
        i += 1;

        if i < arr.len() && arr[i] != 0 {
            let right_node = Box::new(TreeNode::new(arr[i]));
            current.right = Some(right_node);
            queue.push_back(current.right.as_ref().unwrap());
        }
        i += 1;
    }

    Some(root)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let k: i32 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr_size: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let root = create_tree(&arr).unwrap();
    println!("{}", kth_largest_perfect_subtree(&root, k));
}