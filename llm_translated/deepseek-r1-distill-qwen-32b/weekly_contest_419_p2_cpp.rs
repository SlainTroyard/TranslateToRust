use std::io;
use std::collections::VecDeque;

struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn create_tree(arr: Vec<i32>) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }

    let root = Box::new(TreeNode {
        val: arr[0],
        left: None,
        right: None,
    });

    let mut q = VecDeque::new();
    q.push_back(root);

    for i in (1..arr.len()).step_by(2) {
        let current = q.pop_front().unwrap();

        // Left child
        if i < arr.len() && arr[i] != 0 {
            let left = Box::new(TreeNode {
                val: arr[i],
                left: None,
                right: None,
            });
            current.left = Some(left);
            q.push_back(left);
        }

        // Right child
        if i + 1 < arr.len() && arr[i + 1] != 0 {
            let right = Box::new(TreeNode {
                val: arr[i + 1],
                left: None,
                right: None,
            });
            current.right = Some(right);
            q.push_back(right);
        }
    }

    Some(root)
}

struct Solution;

impl Solution {
    fn kth_largest_perfect_subtree(root: Option<Box<TreeNode>>, k: i32) -> i32 {
        let mut heights = Vec::new();

        fn dfs(node: &Option<Box<TreeNode>>, heights: &mut Vec<i32>) -> i32 {
            if let Some(node) = node {
                let left = dfs(&node.left, heights);
                let right = dfs(&node.right, heights);

                if left < 0 || right < 0 || left != right {
                    return -1;
                }

                let current_height = left + 1;
                heights.push(current_height);
                current_height
            } else {
                0
            }
        }

        let _ = dfs(&root, &mut heights);

        if k > heights.len() as i32 || k <= 0 {
            return -1;
        }

        let mut sorted_heights = heights.clone();
        sorted_heights.sort_unstable_by(|a, b| b.cmp(a));

        let h = sorted_heights[(k - 1) as usize];
        (1 << h) - 1
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let k = parts.next().unwrap().parse::<i32>().unwrap();
    let n = parts.next().unwrap().parse::<usize>().unwrap();

    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        let num = num.trim().parse::<i32>().unwrap();
        arr.push(num);
    }

    let root = create_tree(arr);
    let solution = Solution;
    let result = solution.kth_largest_perfect_subtree(root, k);
    println!("{}", result);
}