use std::io::{self, Read};
use std::collections::VecDeque;

// Definition for a binary tree node.
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

/// Recursively checks whether the subtree rooted at `node` is a perfect binary tree.
///
/// `floor` indicates the current depth level. If the subtree is perfect,
/// its total node count is computed as 2^(height difference + 1) - 1 and
/// appended to `perfect_sizes`. The function returns the floor (depth) of the leaves
/// of the perfect subtree, or 0 if the subtree is not perfect.
///
/// This function preserves the same logic as the C function `tree_judge()`.
fn tree_judge(node: &TreeNode, floor: i32, perfect_sizes: &mut Vec<i32>) -> i32 {
    let (mut left_floor, mut right_floor) = (0, 0);

    // Check children existence to call recursive function accordingly
    match (&node.left, &node.right) {
        (Some(left), Some(right)) => {
            left_floor = tree_judge(left, floor + 1, perfect_sizes);
            right_floor = tree_judge(right, floor + 1, perfect_sizes);
        }
        (Some(left), None) => {
            left_floor = tree_judge(left, floor + 1, perfect_sizes);
            right_floor = 0;
        }
        (None, Some(right)) => {
            left_floor = 0;
            right_floor = tree_judge(right, floor + 1, perfect_sizes);
        }
        (None, None) => {
            left_floor = floor;
            right_floor = floor;
        }
    }

    // If the subtree is perfect (both subtrees have same depth and are not smaller than current level)
    if left_floor == right_floor && right_floor >= floor {
        // Compute number of nodes in a perfect binary subtree:
        // 2^(height difference + 1) - 1, using shifting for exponentiation.
        let exponent = (right_floor - floor + 1) as u32;
        let count = 2_i32.pow(exponent) - 1;
        perfect_sizes.push(count);
        return left_floor;
    }

    // Return 0 to indicate that this subtree is not perfect.
    0
}


/// A recursive implementation of quick sort on a mutable slice of integers in the range [l, r].
///
/// This function replicates the C quick_sort() routine.
fn quick_sort(arr: &mut [i32], l: isize, r: isize) {
    if l >= r {
        return;
    }

    let mut l_t = l - 1;
    let mut r_t = r + 1;
    // Use integer division to get the middle index; same as (l + r) >> 1 in C.
    let mid_val = arr[((l + r) >> 1) as usize];

    // Partitioning the array
    while l_t < r_t {
        // Move l_t forward until an element that is not less than mid_val is found.
        loop {
            l_t += 1;
            if arr[l_t as usize] >= mid_val {
                break;
            }
        }
        // Move r_t backward until an element that is not greater than mid_val is found.
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
    quick_sort(arr, l, r_t);
    quick_sort(arr, r_t + 1, r);
}

/// Computes the k-th largest perfect subtree.
///
/// It builds a vector of perfect subtree sizes using `tree_judge`, then sorts it using `quick_sort`.
/// If there are not enough perfect subtrees, returns -1.
fn kth_largest_perfect_subtree(root: &TreeNode, k: usize) -> i32 {
    // Pre-allocate a vector with capacity 10000 to mimic the C code's static allocation.
    let mut perfect_sizes: Vec<i32> = Vec::with_capacity(10000);
    tree_judge(root, 1, &mut perfect_sizes);

    // Sort the array using quick_sort.
    if !perfect_sizes.is_empty() {
        quick_sort(&mut perfect_sizes[..], 0, (perfect_sizes.len() as isize) - 1);
    }

    if perfect_sizes.len() < k {
        return -1;
    }
    perfect_sizes[perfect_sizes.len() - k]
}


/// Creates a binary tree from a level order representation.
///
/// The input slice `arr` is assumed to be in level-order. A value of 0 indicates a null node.
/// This function mimics the C function `create_tree()`.
fn create_tree(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }

    // Create the root node.
    let mut root = Box::new(TreeNode {
        val: arr[0],
        left: None,
        right: None,
    });

    // Use a queue to assign children in level-order.
    let mut queue = VecDeque::new();
    queue.push_back(&mut root);

    let mut i = 1;
    while i < arr.len() {
        if let Some(current) = queue.pop_front() {
            // Assign left child if it exists in the input (non-zero).
            if arr[i] != 0 {
                current.left = Some(Box::new(TreeNode {
                    val: arr[i],
                    left: None,
                    right: None,
                }));
                // Safe to unwrap because we just assigned it.
                queue.push_back(current.left.as_mut().unwrap());
            }
            i += 1;
            // Assign right child if it exists.
            if i < arr.len() {
                if arr[i] != 0 {
                    current.right = Some(Box::new(TreeNode {
                        val: arr[i],
                        left: None,
                        right: None,
                    }));
                    queue.push_back(current.right.as_mut().unwrap());
                }
                i += 1;
            }
        }
    }
    Some(root)
}


/// Prints the tree in level order.
///
/// This utility function mimics the C function `print_tree()`. It prints "null" for absent nodes.
#[allow(dead_code)]
fn print_tree(root: &Option<Box<TreeNode>>) {
    let mut queue = VecDeque::new();
    queue.push_back(root.as_ref());
    while let Some(node_opt) = queue.pop_front() {
        if let Some(node) = node_opt {
            print!("{} ", node.val);
            queue.push_back(node.left.as_ref());
            queue.push_back(node.right.as_ref());
        } else {
            print!("null ");
        }
    }
    println!();
}


/// The main entry point of the program.
/// 
/// It reads input from stdin in the following order:
/// 1. An integer k.
/// 2. An integer arrSize (the number of elements in the level order representation).
/// 3. arrSize integers representing the tree in level-order, where 0 indicates a null node.
/// 
/// It then builds the binary tree, computes the k-th largest perfect subtree size, and prints the result.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the entire input as a single string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    // Split the input into tokens (whitespace separated).
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut iter = tokens.into_iter();

    // Parse k and arr_size from the first two tokens.
    let k: usize = iter.next().ok_or("Missing k")?.parse()?;
    let arr_size: usize = iter.next().ok_or("Missing arr size")?.parse()?;

    // Parse the rest of the tokens into a vector of i32.
    let mut arr: Vec<i32> = Vec::with_capacity(arr_size);
    for _ in 0..arr_size {
        let num: i32 = iter.next().ok_or("Missing an array element")?.parse()?;
        arr.push(num);
    }

    // Create the tree from the input array.
    let root = create_tree(&arr);
    // If the tree is empty, output -1 and exit.
    if root.is_none() {
        println!("-1");
        return Ok(());
    }

    // Compute and print the k-th largest perfect subtree size.
    let result = kth_largest_perfect_subtree(root.as_ref().unwrap(), k);
    println!("{}", result);

    Ok(())
}