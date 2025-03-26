use std::io::{self, BufRead};
use std::cmp::min;

// Stack implementation
struct Stack {
    data: Vec<i32>,
}

impl Stack {
    // Create a new stack
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    // Check if the stack is empty
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // Get the size of the stack
    fn size(&self) -> usize {
        self.data.len()
    }

    // Push a value onto the stack
    fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    // Pop a value from the stack
    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    // Peek at the top value of the stack
    fn top(&self) -> Option<i32> {
        self.data.last().copied()
    }
}

// Helper function to calculate combinations
fn count(m: i32, k: i32) -> i64 {
    if m > k {
        ((m * 2 - k + 1) as i64 * k as i64) / 2
    } else {
        ((m + 1) as i64 * m as i64) / 2
    }
}

// Calculate the sum of subarray minimums
fn sum_subarray_mins(nums: &[i32], k: i32) -> i64 {
    let mut res = 0;
    let mut stack = Stack::new();

    stack.push(-1); // Add sentinel element

    for r in 0..nums.len() {
        while stack.size() > 1 && nums[stack.top().unwrap() as usize] >= nums[r] {
            let i = stack.pop().unwrap();
            let l = stack.top().unwrap();
            let cnt = count((r as i32) - l - 1, k)
                - count(i - l - 1, k)
                - count((r as i32) - i - 1, k);
            res += nums[i as usize] as i64 * cnt;
        }
        stack.push(r as i32);
    }

    res
}

// Calculate the min-max subarray sum
fn min_max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    let mut temp = nums.to_vec();
    temp.push(i32::MIN / 2); // Add a very small value as sentinel

    // Calculate the sum of subarray minimums
    let mut ans = sum_subarray_mins(&temp, k);

    // Negate all elements and calculate again
    for i in 0..temp.len() {
        temp[i] = -temp[i];
    }

    ans -= sum_subarray_mins(&temp, k);

    ans
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let n: usize = first_line_iter.next().unwrap().parse().unwrap();
    let k: i32 = first_line_iter.next().unwrap().parse().unwrap();

    // Read the array
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Calculate the result
    let result = min_max_subarray_sum(&nums, k);

    // Print the result
    println!("{}", result);
}