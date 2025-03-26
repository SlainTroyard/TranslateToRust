use std::cell::RefCell;
use std::cmp::Ordering;
use std::io;
use std::io::prelude::*;
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

struct Solution {}

impl Solution {
    fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut hs: Vec<i32> = Vec::new();
        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            hs: &mut Vec<i32>,
        ) -> i32 {
            if node.is_none() {
                return 0;
            }
            let node = node.unwrap();
            let left_h = dfs(node.borrow().left.clone(), hs);
            let right_h = dfs(node.borrow().right.clone(), hs);

            if left_h < 0 || left_h != right_h {
                return -1;
            }
            hs.push(left_h + 1);
            return left_h + 1;
        }

        dfs(root, &mut hs);

        if k as usize > hs.len() {
            return -1;
        }
        let k_usize = k as usize;
        if !hs.is_empty() {
            let len = hs.len();
            hs.select_nth_unstable_by(len - k_usize, |a, b| b.cmp(a));
        }

        (1 << hs[hs.len() - k_usize]) - 1
    }
}

fn create_tree(arr: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if arr.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(arr[0])));
    let mut q: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    q.push(root.clone());

    let mut i = 1;
    while i < arr.len() {
        let current = q.remove(0);

        if arr[i] != 0 {
            let left = Rc::new(RefCell::new(TreeNode::new(arr[i])));
            current.borrow_mut().left = Some(left.clone());
            q.push(left);
        }

        if i + 1 < arr.len() && arr[i + 1] != 0 {
            let right = Rc::new(RefCell::new(TreeNode::new(arr[i + 1])));
            current.borrow_mut().right = Some(right.clone());
            q.push(right);
        }

        i += 2;
    }

    Some(root)
}

fn main() {
    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).unwrap();
    let mut iter = k_str.trim().split_whitespace();
    let k = iter.next().unwrap().parse::<i32>().unwrap();
    let n = iter.next().unwrap().parse::<i32>().unwrap();

    let mut arr_str = String::new();
    io::stdin().read_line(&mut arr_str).unwrap();
    let arr: Vec<i32> = arr_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let root = create_tree(&arr);
    let solution = Solution {};
    println!("{}", solution.kth_largest_perfect_subtree(root, k));
}