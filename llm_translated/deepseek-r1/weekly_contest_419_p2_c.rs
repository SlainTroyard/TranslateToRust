use std::collections::VecDeque;
use std::io::{self, Read};

// TreeNode struct definition
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

// Function to create a tree from a level-order array
fn create_tree(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }

    let mut root = Box::new(TreeNode {
        val: arr[0],
        left: None,
        right: None,
    });

    let mut queue = VecDeque::new();
    queue.push_back(&mut root);

    let mut i = 1;
    while let Some(current) = queue.pop_front() {
        // Process left child
        if i < arr.len() {
            let left_val = arr[i];
            if left_val != 0 {
                current.left = Some(Box::new(TreeNode {
                    val: left_val,
                    left: None,
                    right: None,
                }));
                if let Some(left) = current.left.as_mut() {
                    queue.push_back(left);
                }
            }
            i += 1;
        }

        // Process right child
        if i < arr.len() {
            let right_val = arr[i];
            if right_val != 0 {
                current.right = Some(Box::new(TreeNode {
                    val: right_val,
                    left: None,
                    right: None,
                }));
                if let Some(right) = current.right.as_mut() {
                    queue.push_back(right);
                }
            }
            i += 1;
        }
    }

    Some(root)
}

// Recursive function to find perfect subtree sizes
fn tree_judge(node: &TreeNode, floor: i32, result: &mut Vec<i32>) -> i32 {
    let (left_floor, right_floor) = match (&node.left, &node.right) {
        (Some(left), Some(right)) => (
            tree_judge(left, floor + 1, result),
            tree_judge(right, floor + 1, result),
        ),
        (Some(left), None) => (tree_judge(left, floor + 1, result), 0),
        (None, Some(right)) => (0, tree_judge(right, floor + 1, result)),
        (None, None) => (floor, floor),
    };

    if left_floor == right_floor && right_floor >= floor {
        let depth_diff = right_floor - floor + 1;
        let size = (2i32).pow(depth_diff as u32) - 1;
        result.push(size);
        left_floor
    } else {
        0
    }
}

// Main logic to find k-th largest perfect subtree
fn kth_largest_perfect_subtree(root: Option<&TreeNode>, k: i32) -> i32 {
    let mut sizes = Vec::new();

    if let Some(node) = root {
        tree_judge(node, 1, &mut sizes);
    }

    let k = k as usize;
    if k == 0 || k > sizes.len() {
        return -1;
    }

    sizes.sort_unstable();
    sizes[sizes.len() - k]
}

// Main function with input handling
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let k = tokens.next().expect("Missing k");
    let arr_size = tokens.next().expect("Missing array size") as usize;
    let arr: Vec<i32> = tokens.take(arr_size).collect();

    let root = create_tree(&arr);
    let result = kth_largest_perfect_subtree(root.as_deref(), k);
    println!("{}", result);
}