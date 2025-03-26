use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::{self, Read};
use std::rc::Rc;

// Define a type alias for a reference counted, mutable TreeNode.
type NodeRef = Rc<RefCell<TreeNode>>;

// Definition of a binary tree node.
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
}

// Helper constructor methods for TreeNode.
impl TreeNode {
    fn new(val: i32) -> NodeRef {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }
}

// The Solution struct that contains our algorithm.
struct Solution;

impl Solution {
    // This function computes the kth largest perfect subtree. A perfect subtree is defined
    // such that the left and right subtree heights are equal (and non-negative), and its own height
    // is one plus its common child height.
    //
    // The function returns the number of nodes in the kth largest perfect subtree found.
    // If there are fewer than k perfect subtrees then it returns -1.
    //
    // The number of nodes is computed as (1 << height) - 1.
    fn kth_largest_perfect_subtree(root: &Option<NodeRef>, k: usize) -> i32 {
        // This vector collects the heights of all perfect subtrees.
        let mut hs: Vec<i32> = Vec::new();

        // Define a recursive dfs helper function.
        // It returns the height of the perfect subtree if the subtree rooted at `node` is perfect,
        // otherwise it returns -1.
        fn dfs(node: Option<&NodeRef>, hs: &mut Vec<i32>) -> i32 {
            match node {
                None => 0, // An empty tree is perfect with height 0.
                Some(n) => {
                    // To avoid multiple mutable borrows, we clone the Option references for left and right.
                    let node_ref = n.borrow();
                    // Get the left and right children as Option<&NodeRef>
                    let left = node_ref.left.as_ref();
                    let right = node_ref.right.as_ref();
                    
                    // Recurse on the left and right children.
                    let left_h = dfs(left, hs);
                    let right_h = dfs(right, hs);

                    // If either subtree is not perfect, or their heights differ, then this subtree is not perfect.
                    if left_h < 0 || left_h != right_h {
                        return -1;
                    }
                    let current_h = left_h + 1;
                    hs.push(current_h);
                    current_h
                }
            }
        }

        // Run dfs starting from the root.
        dfs(root.as_ref(), &mut hs);

        // If there are fewer perfect subtrees than k, return -1.
        if k > hs.len() {
            return -1;
        }
        // Use Rust's select_nth_unstable to partition the vector such that the (len - k)th element
        // is the kth largest element.
        let pos = hs.len() - k;
        hs.select_nth_unstable(pos);
        let height = hs[pos];
        // Compute and return the number of nodes in a perfect binary tree of this height.
        (1 << height) - 1
    }
}

// Create a binary tree from the given array of integers.
// The array represents nodes in level-order, where a value of 0 indicates that the node is absent.
fn create_tree(arr: &[i32]) -> Option<NodeRef> {
    if arr.is_empty() {
        return None;
    }

    // The first number in the array is always the root.
    let root = TreeNode::new(arr[0]);
    let mut q = VecDeque::new();
    q.push_back(root.clone());

    // Iterate over the array in steps of 2 for the left and right children.
    let mut i = 1;
    while i < arr.len() {
        // Get the next node in the queue.
        if let Some(current) = q.pop_front() {
            // For the left child.
            if arr[i] != 0 {
                let left_node = TreeNode::new(arr[i]);
                current.borrow_mut().left = Some(left_node.clone());
                q.push_back(left_node);
            }
            // For the right child.
            if i + 1 < arr.len() && arr[i + 1] != 0 {
                let right_node = TreeNode::new(arr[i + 1]);
                current.borrow_mut().right = Some(right_node.clone());
                q.push_back(right_node);
            }
        }
        i += 2;
    }

    Some(root)
}

fn main() -> io::Result<()> {
    // Read the entire input from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input by whitespace and parse the values.
    let mut iter = input.split_whitespace();
    let k: usize = iter
        .next()
        .expect("Expected k as the first integer")
        .parse()
        .expect("k should be a valid integer");
    let n: usize = iter
        .next()
        .expect("Expected n as the second integer")
        .parse()
        .expect("n should be a valid integer");

    // Read n integers into a vector.
    let arr: Vec<i32> = iter
        .take(n)
        .map(|s| s.parse().expect("Expected a valid integer"))
        .collect();

    // Create the binary tree from arr.
    let root = create_tree(&arr);

    // Compute the kth largest perfect subtree.
    let result = Solution::kth_largest_perfect_subtree(&root, k);

    // Output the result to stdout.
    println!("{}", result);

    Ok(())
}