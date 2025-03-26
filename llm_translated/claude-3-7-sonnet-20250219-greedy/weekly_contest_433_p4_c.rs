use std::io::{self, BufRead};
use std::cmp::min;

// Stack implementation
struct Stack {
    data: Vec<i32>,
    top: i32,
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

    // Get size of stack
    fn size(&self) -> i32 {
        self.top + 1
    }

    // Push value onto stack
    fn push(&mut self, value: i32) {
        self.top += 1;
        if self.top as usize >= self.data.len() {
            self.data.push(value);
        } else {
            self.data[self.top as usize] = value;
        }
    }

    // Pop value from stack
    fn pop(&mut self) -> i32 {
        if self.is_empty() {
            return -1; // Stack empty
        }
        let value = self.data[self.top as usize];
        self.top -= 1;
        value
    }

    // Get top element without removing
    fn top(&self) -> i32 {
        if self.is_empty() {
            return -1; // Stack empty
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

// Calculate sum of subarray minimums
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
    let nums_size = nums.len();
    
    // Create a temporary array with an extra element
    let mut temp = Vec::with_capacity(nums_size + 1);
    temp.extend_from_slice(nums);
    temp.push(i32::MIN / 2); // Add a very small value as sentinel
    
    // Calculate sum of subarray minimums
    let ans = sum_subarray_mins(&temp, k);
    
    // Negate all elements and calculate again
    for i in 0..nums_size + 1 {
        temp[i] = -temp[i];
    }
    temp[nums_size] = -temp[nums_size]; // Restore sentinel element's sign
    
    // Subtract the negated result from the total
    ans - sum_subarray_mins(&temp, k)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and k
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read the array
    let second_line = lines.next().unwrap()?;
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate and print the result
    let result = min_max_subarray_sum(&nums, k);
    println!("{}", result);
    
    Ok(())
}