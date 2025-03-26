use std::io;
use std::cmp;
use std::collections::VecDeque;

// Stack implementation in Rust
struct Stack {
    data: Vec<i32>,
}

impl Stack {
    fn new(capacity: usize) -> Self {
        Stack {
            data: Vec::with_capacity(capacity),
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    fn top(&self) -> Option<i32> {
        self.data.last().copied()
    }
}

// Helper function for calculating combinations
fn count(m: i32, k: i32) -> i64 {
    if m > k {
        ((m * 2 - k + 1) as i64 * k as i64) / 2
    } else {
        ((m + 1) as i64 * m as i64) / 2
    }
}

// Calculate subarray minimums sum
fn sum_subarray_mins(nums: &[i32], k: i32) -> i64 {
    let mut res: i64 = 0;
    let mut stack = Stack::new(nums.len() + 1);

    stack.push(-1); // Sentinel element

    for r in 0..nums.len() {
        while stack.size() > 1 && nums[stack.top().unwrap() as usize] >= nums[r] {
            let i = stack.pop().unwrap();
            let l = stack.top().unwrap();
            let cnt = count((r as i32 - l - 1), k)
                - count((i - l - 1), k)
                - count((r as i32 - i - 1), k);
            res += nums[i as usize] as i64 * cnt;
        }
        stack.push(r as i32);
    }

    res
}

// Compute the result for the problem
fn min_max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    // Create a temporary array with original values and a sentinel element
    let mut temp: Vec<i32> = nums.to_vec();
    temp.push(cmp::min(i32::MIN / 2, -100_000)); // Add very small sentinel value

    // Calculate the sum of subarray minimums
    let mut ans = sum_subarray_mins(&temp, k);

    // Flip all elements and recalculate
    for elem in temp.iter_mut() {
        *elem = -*elem;
    }

    ans -= sum_subarray_mins(&temp, k);
    ans
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read n, k");
    let mut values = input.trim().split_whitespace();
    let n: usize = values.next().unwrap().parse().expect("Invalid n");
    let k: i32 = values.next().unwrap().parse().expect("Invalid k");

    let mut nums = Vec::with_capacity(n);
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read nums array");
    nums.extend(
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("Invalid element in nums")),
    );

    assert_eq!(nums.len(), n, "Nums array size mismatch");

    // Compute result
    let result = min_max_subarray_sum(&nums, k);

    // Output result
    println!("{}", result);
}