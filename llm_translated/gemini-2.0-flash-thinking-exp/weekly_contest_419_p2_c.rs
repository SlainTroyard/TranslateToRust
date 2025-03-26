use std::{io, process};
use std::collections::VecDeque;
use std::cmp::Reverse;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn tree_judge_rs(root: &Option<Box<TreeNode>>, floor: i32, perfect_subtree_sizes: &mut Vec<i32>) -> i32 {
    if let Some(node) = root {
        let mut left_floor = 0;
        let mut right_floor = 0;

        match (&node.left, &node.right) {
            (Some(_), Some(_)) => {
                left_floor = tree_judge_rs(&node.left, floor + 1, perfect_subtree_sizes);
                right_floor = tree_judge_rs(&node.right, floor + 1, perfect_subtree_sizes);
            }
            (Some(_), None) => {
                left_floor = tree_judge_rs(&node.left, floor + 1, perfect_subtree_sizes);
                right_floor = 0;
            }
            (None, Some(_)) => {
                left_floor = 0;
                right_floor = tree_judge_rs(&node.right, floor + 1, perfect_subtree_sizes);
            }
            (None, None) => {
                left_floor = floor;
                right_floor = floor;
            }
        }

        if left_floor == right_floor && right_floor >= floor {
            perfect_subtree_sizes.push(i32::pow(2, (right_floor - floor + 1) as u32) - 1);
            return left_floor;
        }
        0 // Return 0 if not a perfect subtree at this level for the current root in the path.
    } else {
        0 // Base case for null node, should not be reached under normal conditions for subtree check
    }
}

fn kth_largest_perfect_subtree_rs(root: Option<Box<TreeNode>>, k: i32) -> i32 {
    let mut perfect_subtree_sizes = Vec::new();
    tree_judge_rs(&root, 1, &mut perfect_subtree_sizes);
    perfect_subtree_sizes.sort_by_key(|&x| Reverse(x)); // Sort in descending order

    if (k as usize) > 0 && (k as usize) <= perfect_subtree_sizes.len() {
        perfect_subtree_sizes[(k as usize) - 1]
    } else {
        -1
    }
}

fn create_tree_rs(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }

    let root = Some(Box::new(TreeNode {
        val: arr[0],
        left: None,
        right: None,
    }));

    let mut queue: VecDeque<&mut Option<Box<TreeNode>>> = VecDeque::new();
    queue.push_back(&mut root.as_mut().unwrap().left);
    queue.push_back(&mut root.as_mut().unwrap().right);

    let mut i = 1;
    while i < arr.len() && !queue.is_empty() {
        if arr[i] != 0 {
            let new_node = Some(Box::new(TreeNode {
                val: arr[i],
                left: None,
                right: None,
            }));
            if let Some(current_child_ref) = queue.pop_front() {
                *current_child_ref = new_node;
                if current_child_ref.is_some() {
                    queue.push_back(&mut current_child_ref.as_mut().unwrap().left);
                    queue.push_back(&mut current_child_ref.as_mut().unwrap().right);
                }
            }
        } else {
            queue.pop_front(); //consume null node position even if value is 0
            if !queue.is_empty(){
                queue.pop_front(); //consume null node position even if value is 0
            }

        }
        i += 2;
    }

    root
}

fn print_tree_rs(root: &Option<Box<TreeNode>>) {
    let mut queue: VecDeque<&Option<Box<TreeNode>>> = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let node_opt = queue.pop_front();
        if let Some(node_ref) = node_opt {
            if let Some(node) = node_ref {
                print!("{} ", node.val);
                queue.push_back(&node.left);
                queue.push_back(&node.right);
            } else {
                print!("null ");
            }
        } else {
            print!("null "); // Should not reach here unless initial root is None
        }
    }
    println!();
}


fn main() -> io::Result<()> {
    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str)?;
    let k: i32 = k_str.trim().parse().unwrap();

    let mut arr_size_str = String::new();
    io::stdin().read_line(&mut arr_size_str)?;
    let arr_size: usize = arr_size_str.trim().parse().unwrap();

    let mut arr_str = String::new();
    io::stdin().read_line(&mut arr_str)?;
    let arr: Vec<i32> = arr_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let root = create_tree_rs(&arr);
    let result = kth_largest_perfect_subtree_rs(root, k);
    println!("{}", result);

    Ok(())
}