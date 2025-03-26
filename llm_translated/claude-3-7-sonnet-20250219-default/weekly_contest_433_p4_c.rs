use std::io::{self, BufRead};
use std::cmp::min;
use std::i32;

// Stack implementation
struct Stack {
    data: Vec<i32>,
    top: isize,
}

impl Stack {
    // Create a new stack with given capacity
    fn new(capacity: usize) -> Self {
        Stack {
            data: Vec::with_capacity(capacity),
            top: -1,
        }
    }

    // Check if stack is empty
    fn is_empty(&self) -> bool {
        self.top == -1
    }

    // Get the size of the stack
    fn size(&self) -> usize {
        (self.top + 1) as usize
    }

    // Push a value onto the stack
    fn push(&mut self, value: i32) {
        self.top += 1;
        if self.top as usize == self.data.len() {
            self.data.push(value);
        } else {
            self.data[self.top as usize] = value;
        }
    }

    // Pop a value from the stack
    fn pop(&mut self) -> i32 {
        if self.is_empty() {
            return -1; // Stack is empty
        }
        let value = self.data[self.top as usize];
        self.top -= 1;
        value
    }

    // Get the top element without removing it
    fn top(&self) -> i32 {
        if self.is_empty() {
            return -1; // Stack is empty
        }
        self.data[self.top as usize]
    }
}

// Helper function to calculate combination count
fn count(m: i32, k: i32) -> i64 {
    if m > k {
        ((m * 2 - k + 1) as i64 * k as i64) / 2
    } else {
        ((m + 1) as i64 * m as i64) / 2
    }
}

// Calculate the sum of subarray minimums
fn sum_subarray_mins(nums: &[i32], k: i32) -> i64 {
    let mut res: i64 = 0;
    let nums_size = nums.len();
    let mut stack = Stack::new(nums_size + 1);
    
    stack.push(-1); // Add sentinel element
    
    for r in 0..nums_size {
        while stack.size() > 1 && nums[stack.top() as usize] >= nums[r] {
            let i = stack.pop();
            let l = stack.top();
            let cnt = count(r as i32 - l - 1, k) - count(i - l - 1, k) - count(r as i32 - i - 1, k);
            res += nums[i as usize] as i64 * cnt;
        }
        stack.push(r as i32);
    }
    
    res
}

// Main function to calculate min-max subarray sum
fn min_max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    // Create a temporary array with an extra element
    let nums_size = nums.len();
    let mut temp = Vec::with_capacity(nums_size + 1);
    
    // Copy the original array
    temp.extend_from_slice(nums);
    temp.push(i32::MIN / 2); // Add a very small value as sentinel
    
    // Calculate the sum of subarray minimums
    let ans = sum_subarray_mins(&temp, k);
    
    // Negate all elements and calculate again
    for i in 0..nums_size + 1 {
        temp[i] = -temp[i];
    }
    temp[nums_size] = -temp[nums_size]; // Restore the sentinel element's sign
    
    // Subtract the negated result from the first calculation
    ans - sum_subarray_mins(&temp, k)
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and k
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<i32> = first_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = parts[0];
    let k = parts[1];
    
    // Read the array
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate the result
    let result = min_max_subarray_sum(&nums, k);
    
    // Output the result
    println!("{}", result);
}