use std::io::{self, BufRead, Write};

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn tree_judge(root: &Option<Box<TreeNode>>, floor: i32, arr: &mut Vec<i32>) -> i32 {
    let mut left_floor = 0;
    let mut right_floor = 0;

    if let Some(node) = root {
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
        l_t += 1;
        while arr[l_t] < mid_val {
            l_t += 1;
        }
        r_t -= 1;
        while arr[r_t] > mid_val {
            r_t -= 1;
        }
        if l_t < r_t {
            arr.swap(l_t, r_t);
        }
    }

    quick_sort(arr, l, r_t);
    quick_sort(arr, r_t + 1, r);
}

fn kth_largest_perfect_subtree(root: &Option<Box<TreeNode>>, k: i32) -> i32 {
    let mut arr = Vec::new();
    tree_judge(root, 1, &mut arr);

    quick_sort(&mut arr, 0, arr.len() - 1);
    if arr.len() as i32 - k < 0 {
        return -1;
    }

    arr[arr.len() - k as usize]
}

fn create_tree(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }

    let mut root = Box::new(TreeNode {
        val: arr[0],
        left: None,
        right: None,
    });

    let mut queue = vec![root.as_mut()];
    let mut i = 1;

    while i < arr.len() {
        if let Some(current) = queue.pop() {
            if i < arr.len() && arr[i] != 0 {
                current.left = Some(Box::new(TreeNode {
                    val: arr[i],
                    left: None,
                    right: None,
                }));
                queue.push(current.left.as_mut().unwrap());
            }
            i += 1;

            if i < arr.len() && arr[i] != 0 {
                current.right = Some(Box::new(TreeNode {
                    val: arr[i],
                    left: None,
                    right: None,
                }));
                queue.push(current.right.as_mut().unwrap());
            }
            i += 1;
        }
    }

    Some(root)
}

fn free_tree(root: &Option<Box<TreeNode>>) {
    if let Some(node) = root {
        free_tree(&node.left);
        free_tree(&node.right);
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

    let root = create_tree(&arr);

    // Compute and print the result
    writeln!(stdout, "{}", kth_largest_perfect_subtree(&root, k)).unwrap();

    // Free the tree
    free_tree(&root);
}