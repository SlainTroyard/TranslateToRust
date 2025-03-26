use std::io::{self, BufRead, Write};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn tree_judge(root: &Option<Rc<RefCell<TreeNode>>>, floor: i32, arr: &mut Vec<i32>) -> i32 {
    let mut left_floor = 0;
    let mut right_floor = 0;

    if let Some(node) = root {
        let node_borrowed = node.borrow();
        if node_borrowed.left.is_some() && node_borrowed.right.is_some() {
            left_floor = tree_judge(&node_borrowed.left, floor + 1, arr);
            right_floor = tree_judge(&node_borrowed.right, floor + 1, arr);
        } else if node_borrowed.left.is_some() {
            left_floor = tree_judge(&node_borrowed.left, floor + 1, arr);
            right_floor = 0;
        } else if node_borrowed.right.is_some() {
            left_floor = 0;
            right_floor = tree_judge(&node_borrowed.right, floor + 1, arr);
        } else {
            left_floor = floor;
            right_floor = floor;
        }

        if left_floor == right_floor && right_floor >= floor {
            arr.push(2_i32.pow((right_floor - floor + 1) as u32) - 1);
            return left_floor;
        }
    }

    0
}

fn quick_sort(arr: &mut [i32], l: usize, r: usize) {
    if l >= r {
        return;
    }

    let mid_val = arr[(l + r) / 2];
    let (mut l_t, mut r_t) = (l - 1, r + 1);

    while l_t < r_t {
        loop {
            l_t += 1;
            if arr[l_t] >= mid_val { break; }
        }

        loop {
            r_t -= 1;
            if arr[r_t] <= mid_val { break; }
        }

        if l_t < r_t {
            arr.swap(l_t, r_t);
        }
    }

    quick_sort(arr, l, r_t);
    quick_sort(arr, r_t + 1, r);
}

fn kth_largest_perfect_subtree(root: &Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut arr = Vec::new();
    tree_judge(root, 1, &mut arr);

    quick_sort(&mut arr, 0, arr.len() - 1);
    if arr.len() as i32 - k < 0 {
        return -1;
    }

    arr[arr.len() - k as usize]
}

fn create_tree(arr: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if arr.is_empty() {
        return None;
    }

    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(arr[0]))));
    let mut queue = vec![root.clone()];

    for i in (1..arr.len()).step_by(2) {
        if let Some(node) = queue.pop() {
            if arr[i] != 0 {
                let left_node = Some(Rc::new(RefCell::new(TreeNode::new(arr[i]))));
                node.as_ref().unwrap().borrow_mut().left = left_node.clone();
                queue.push(left_node);
            }

            if i + 1 < arr.len() && arr[i + 1] != 0 {
                let right_node = Some(Rc::new(RefCell::new(TreeNode::new(arr[i + 1]))));
                node.as_ref().unwrap().borrow_mut().right = right_node.clone();
                queue.push(right_node);
            }
        }
    }

    root
}

fn free_tree(root: &Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        free_tree(&node.borrow().left);
        free_tree(&node.borrow().right);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read k
    stdin.lock().read_line(&mut buffer).unwrap();
    let k: i32 = buffer.trim().parse().unwrap();
    buffer.clear();

    // Read array size
    stdin.lock().read_line(&mut buffer).unwrap();
    let arr_size: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Read array elements
    let mut arr = vec![0; arr_size];
    for i in 0..arr_size {
        stdin.lock().read_line(&mut buffer).unwrap();
        arr[i] = buffer.trim().parse().unwrap();
        buffer.clear();
    }

    // Create the tree
    let root = create_tree(&arr);

    // Get the k-th largest perfect subtree
    let result = kth_largest_perfect_subtree(&root, k);

    // Print the result
    writeln!(stdout, "{}", result).unwrap();

    // Free the tree
    free_tree(&root);
}