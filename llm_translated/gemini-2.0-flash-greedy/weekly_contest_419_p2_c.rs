use std::cell::RefCell;
use std::cmp::Ordering;
use std::io;
use std::io::Read;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
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
        let mut left_floor = 0;
        let mut right_floor = 0;

        let node = node.borrow();

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
            arr.push((2i32.pow((right_floor - floor + 1) as u32) - 1) as i32);
            return left_floor;
        }

        0
    } else {
        0 // Handle the case where root is None
    }
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    fn partition(arr: &mut [i32]) -> usize {
        let pivot = arr[arr.len() / 2];
        let mut i = 0;
        let mut j = arr.len() - 1;

        while i <= j {
            while arr[i] < pivot {
                i += 1;
            }
            while arr[j] > pivot {
                j -= 1;
            }
            if i <= j {
                arr.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        i
    }

    let p = partition(arr);
    quick_sort(&mut arr[0..p - 1]);
    quick_sort(&mut arr[p..]);
}

fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut arr: Vec<i32> = Vec::new();
    tree_judge(&root, 1, &mut arr);

    quick_sort(&mut arr);

    let arr_size = arr.len();
    if arr_size as i32 - k < 0 {
        return -1;
    }

    arr[arr_size - k as usize]
}

fn create_tree(arr: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if arr.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(arr[0])));
    let mut queue: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    queue.push(root.clone());

    let mut i = 1;
    let mut front = 0;

    while i < arr.len() {
        let current = queue[front].clone();
        front += 1;

        if arr[i] != 0 {
            let left_node = Rc::new(RefCell::new(TreeNode::new(arr[i])));
            current.borrow_mut().left = Some(left_node.clone());
            queue.push(left_node);
        }

        if i + 1 < arr.len() && arr[i + 1] != 0 {
            let right_node = Rc::new(RefCell::new(TreeNode::new(arr[i + 1])));
            current.borrow_mut().right = Some(right_node.clone());
            queue.push(right_node);
        }

        i += 2;
    }

    Some(root)
}

// Function to print the tree in level order for debugging
fn print_tree(root: &Option<Rc<RefCell<TreeNode>>>) {
    let mut queue: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    queue.push(root.clone());
    let mut i = 0;

    while i < queue.len() {
        if let Some(node) = &queue[i] {
            print!("{} ", node.borrow().val);
            queue.push(node.borrow().left.clone());
            queue.push(node.borrow().right.clone());
        } else {
            print!("null ");
        }
        i += 1;
    }
    println!();
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let k: i32 = lines.next().unwrap().parse().unwrap();
    let arr_size: usize = lines.next().unwrap().parse().unwrap();

    let arr: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let root = create_tree(&arr);

    let result = kth_largest_perfect_subtree(root, k);
    println!("{}", result);

    Ok(())
}