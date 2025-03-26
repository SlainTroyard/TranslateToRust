use std::cell::RefCell;
use std::cmp::Reverse;
use std::io;
use std::io::Read;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut hs: Vec<i32> = Vec::new();
        let mut dfs = |dfs: &mut dyn FnMut(Option<Rc<RefCell<TreeNode>>>) -> i32,
                       node: Option<Rc<RefCell<TreeNode>>>|
         -> i32 {
            if node.is_none() {
                return 0;
            }
            let node = node.unwrap();
            let left_h = dfs(node.borrow().left.clone());
            let right_h = dfs(node.borrow().right.clone());
            if left_h < 0 || left_h != right_h {
                return -1;
            }
            hs.push(left_h + 1);
            return left_h + 1;
        };
        dfs(&mut dfs, root);

        if k as usize > hs.len() {
            return -1;
        }
        hs.sort_by_key(|&x| Reverse(x)); // Sort in descending order
        hs.dedup();
        if k as usize > hs.len() {
            return -1;
        }

        (1 << hs[k as usize - 1]) - 1
    }
}

fn create_tree(arr: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
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

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut nums = first_line.split_whitespace();
    let k: i32 = nums.next().unwrap().parse().unwrap();
    let n: i32 = nums.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap();
    let arr: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let root = create_tree(&arr);
    let solution = Solution {};
    println!("{}", solution.kth_largest_perfect_subtree(root, k));

    Ok(())
}