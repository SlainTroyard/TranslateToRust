use std::collections::VecDeque;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

struct Solution;

impl Solution {
    fn kth_largest_perfect_subtree(&self, root: Option<Box<TreeNode>>, k: i32) -> i32 {
        let mut hs = Vec::new();

        fn dfs(node: &Option<Box<TreeNode>>, hs: &mut Vec<i32>) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let left_h = dfs(&n.left, hs);
                    let right_h = dfs(&n.right, hs);
                    if left_h < 0 || left_h != right_h {
                        return -1;
                    }
                    let current_h = left_h + 1;
                    hs.push(current_h);
                    current_h
                }
            }
        }

        dfs(&root, &mut hs);

        if k as usize > hs.len() {
            return -1;
        }
        // Sort in descending order to get k-th largest
        hs.sort_unstable_by(|a, b| b.cmp(a));
        let val = hs[k as usize - 1];
        (1 << val) - 1
    }
}

fn create_tree(arr: Vec<i32>) -> Option<Box<TreeNode>> {
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

    let mut index = 1;

    while let Some(current_node) = queue.pop_front() {
        if index < arr.len() && arr[index] != 0 {
            let left_child = Box::new(TreeNode {
                val: arr[index],
                left: None,
                right: None,
            });
            current_node.left = Some(left_child);
            queue.push_back(current_node.left.as_mut().unwrap());
        }
        index += 1;

        if index < arr.len() && arr[index] != 0 {
            let right_child = Box::new(TreeNode {
                val: arr[index],
                left: None,
                right: None,
            });
            current_node.right = Some(right_child);
            queue.push_back(current_node.right.as_mut().unwrap());
        }
        index += 1;
    }

    Some(root)
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let k = tokens[0];
    let n = tokens[1] as usize;
    let arr = &tokens[2..2 + n];

    let root = create_tree(arr.to_vec());
    let solution = Solution {};
    let result = solution.kth_largest_perfect_subtree(root, k);
    println!("{}", result);
}