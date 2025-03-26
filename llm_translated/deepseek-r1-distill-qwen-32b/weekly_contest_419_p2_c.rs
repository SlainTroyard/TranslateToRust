use std::collections::VecDeque;
use std::io::{self, BufRead};

struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn tree_judge(root: &TreeNode, floor: i32, arr: &mut Vec<i32>) -> i32 {
    let mut left_floor = 0;
    let mut right_floor = 0;

    if let Some(left) = &root.left {
        left_floor = tree_judge(left, floor + 1, arr);
    }
    if let Some(right) = &root.right {
        right_floor = tree_judge(right, floor + 1, arr);
    }

    if root.left.is_none() && root.right.is_none() {
        left_floor = floor;
        right_floor = floor;
    } else if root.left.is_none() {
        left_floor = 0;
    } else if root.right.is_none() {
        right_floor = 0;
    }

    if left_floor == right_floor && right_floor >= floor {
        let size = (2_i32.pow((right_floor - floor + 1) as u32)) - 1;
        arr.push(size);
        return left_floor;
    }

    0
}

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
                if l_t >= arr.len() || arr[l_t] >= mid_val {
                    break;
                }
            }

            loop {
                r_t -= 1;
                if r_t < l_t || arr[r_t] <= mid_val {
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

fn create_tree(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }

    let root = Box::new(TreeNode {
        val: arr[0],
        left: None,
        right: None,
    });

    let mut queue = VecDeque::new();
    queue.push_back(&mut *root);

    let mut i = 1;
    while i < arr.len() && !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        if arr[i] != 0 {
            let left = Box::new(TreeNode {
                val: arr[i],
                left: None,
                right: None,
            });
            current.left = Some(left);
            queue.push_back(current.left.as_mut().unwrap());
        }
        i += 1;
        if i >= arr.len() {
            break;
        }

        if arr[i] != 0 {
            let right = Box::new(TreeNode {
                val: arr[i],
                left: None,
                right: None,
            });
            current.right = Some(right);
            queue.push_back(current.right.as_mut().unwrap());
        }
        i += 1;
    }

    Some(root)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k
    let k_line = lines.next().ok_or("No input")??;
    let k: i32 = k_line.trim().parse()?;

    // Read array size
    let size_line = lines.next().ok_or("No size input")??;
    let size: usize = size_line.trim().parse()?;

    // Read array elements
    let mut arr = Vec::new();
    for line in lines {
        let line = line?;
        let nums: Vec<i32> = line.trim().split_whitespace()
            .map(|s| s.parse().map_err(|e| e.into()))
            .collect::<Result<_, _>>()?;
        arr.extend(nums);
    }

    // Ensure we have enough elements
    if arr.len() < size {
        while arr.len() < size {
            let line = lines.next().ok_or("Not enough elements")??;
            let nums: Vec<i32> = line.trim().split_whitespace()
                .map(|s| s.parse().map_err(|e| e.into()))
                .collect::<Result<_, _>>()?;
            arr.extend(nums);
        }
    }

    // Create tree
    let root = create_tree(&arr[..size]);

    // Collect perfect subtree sizes
    let mut perfect_sizes = Vec::new();
    if let Some(r) = root.as_ref() {
        tree_judge(r, 1, &mut perfect_sizes);
    }

    // Sort the array
    quick_sort(&mut perfect_sizes);

    // Find k-th largest
    if perfect_sizes.len() >= k as usize {
        println!("{}", perfect_sizes[perfect_sizes.len() - k as usize]);
    } else {
        println!("-1");
    }

    Ok(())
}