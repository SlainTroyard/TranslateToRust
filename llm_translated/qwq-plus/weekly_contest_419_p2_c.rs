use std::cell::RefCell;
use std::collections::VecDeque;
use std::io;
use std::rc::Rc;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

// Creates a binary tree from an array of integers.
fn create_tree(arr: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if arr.is_empty() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode {
        val: arr[0],
        left: None,
        right: None,
    }));
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());
    for i in (1..arr.len()).step_by(2) {
        let current = queue.pop_front().unwrap();
        let mut current_borrow = current.borrow_mut();
        if i < arr.len() && arr[i] != 0 {
            let left_node = Rc::new(RefCell::new(TreeNode {
                val: arr[i],
                left: None,
                right: None,
            }));
            current_borrow.left = Some(left_node.clone());
            queue.push_back(left_node);
        }
        if i + 1 < arr.len() && arr[i + 1] != 0 {
            let right_node = Rc::new(RefCell::new(TreeNode {
                val: arr[i + 1],
                left: None,
                right: None,
            }));
            current_borrow.right = Some(right_node.clone());
            queue.push_back(right_node);
        }
    }
    Some(root)
}

// Recursively checks and collects sizes of perfect subtrees.
fn tree_judge(
    node: &Option<Rc<RefCell<TreeNode>>>,
    floor: i32,
    collector: &mut Vec<i32>,
) -> i32 {
    if let Some(n) = node {
        let n_borrow = n.borrow();
        let left = &n_borrow.left;
        let right = &n_borrow.right;

        let (left_floor, right_floor) = if left.is_some() && right.is_some() {
            let lf = tree_judge(left, floor + 1, collector);
            let rf = tree_judge(right, floor + 1, collector);
            (lf, rf)
        } else if left.is_some() {
            let lf = tree_judge(left, floor + 1, collector);
            (lf, 0)
        } else if right.is_some() {
            let rf = tree_judge(right, floor + 1, collector);
            (0, rf)
        } else {
            (floor, floor)
        };

        if left_floor == right_floor && left_floor >= floor {
            let size = (2_i32.pow((left_floor - floor + 1) as u32) - 1);
            collector.push(size);
            left_floor
        } else {
            0
        }
    } else {
        0
    }
}

// Custom quicksort implementation to sort the collected sizes.
fn quick_sort(arr: &mut [i32]) {
    fn sort(arr: &mut [i32], l: usize, r: usize) {
        if l >= r {
            return;
        }
        let mut l_t = l - 1;
        let mut r_t = r + 1;
        let mid_val = arr[(l + r) >> 1];
        while l_t < r_t {
            loop {
                l_t += 1;
                if arr[l_t] >= mid_val {
                    break;
                }
            }
            loop {
                r_t -= 1;
                if arr[r_t] <= mid_val {
                    break;
                }
            }
            if l_t < r_t {
                arr.swap(l_t, r_t);
            }
        }
        sort(arr, l, r_t);
        sort(arr, r_t + 1, r);
    }
    sort(arr, 0, arr.len() - 1);
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();

    // Read k from input.
    stdin.read_line(&mut input)
        .expect("Failed to read k");
    let k: i32 = input.trim().parse().expect("Invalid k");
    input.clear();

    // Read array size.
    stdin.read_line(&mut input)
        .expect("Failed to read array size");
    let arr_size: usize = input.trim().parse().expect("Invalid array size");
    input.clear();

    // Read array elements.
    let mut elements = Vec::new();
    loop {
        let mut s = String::new();
        if stdin.read_line(&mut s).is_err() {
            break;
        }
        for token in s.split_whitespace() {
            elements.push(token.parse().unwrap());
            if elements.len() == arr_size {
                break;
            }
        }
        if elements.len() == arr_size {
            break;
        }
    }
    elements.truncate(arr_size);

    // Create the tree.
    let root = create_tree(&elements);

    // Collect perfect subtree sizes.
    let mut collector = Vec::new();
    if let Some(root_node) = root {
        tree_judge(&Some(root_node), 1, &mut collector);
    }

    // Sort the collected sizes.
    quick_sort(&mut collector);

    // Determine the k-th largest element.
    let result = if collector.len() >= k as usize {
        collector[collector.len() - k as usize]
    } else {
        -1
    };

    println!("{}", result);
}