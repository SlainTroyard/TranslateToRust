use std::collections::VecDeque;
use std::io::{self, BufRead};

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
    let left_floor = if let Some(ref left) = root.left {
        tree_judge(left, floor + 1)
    } else {
        floor
    };

    let right_floor = if let Some(ref right) = root.right {
        tree_judge(right, floor + 1)
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

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = arr[arr.len() / 2];
    let (mut left, mut right) = (0, arr.len() - 1);

    while left <= right {
        while arr[left] < pivot {
            left += 1;
        }
        while arr[right] > pivot {
            right -= 1;
        }
        if left <= right {
            arr.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    if right > 0 {
        quick_sort(&mut arr[..=right]);
    }
    if left < arr.len() {
        quick_sort(&mut arr[left..]);
    }
}

fn kth_largest_perfect_subtree(root: &TreeNode, k: usize) -> i32 {
    unsafe {
        ARR.clear();
        ARR_SIZE = 0;
        tree_judge(root, 1);

        quick_sort(&mut ARR);
        if ARR_SIZE < k {
            return -1;
        }

        ARR[ARR_SIZE - k]
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
            current.left = Some(Box::new(TreeNode::new(arr[i])));
            queue.push_back(current.left.as_ref().unwrap());
        }
        i += 1;

        if i < arr.len() && arr[i] != 0 {
            current.right = Some(Box::new(TreeNode::new(arr[i])));
            queue.push_back(current.right.as_ref().unwrap());
        }
        i += 1;
    }

    Some(root)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let k: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let arr_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut arr = Vec::with_capacity(arr_size);
    for _ in 0..arr_size {
        let val: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        arr.push(val);
    }

    let root = create_tree(&arr).unwrap();
    println!("{}", kth_largest_perfect_subtree(&root, k));
}