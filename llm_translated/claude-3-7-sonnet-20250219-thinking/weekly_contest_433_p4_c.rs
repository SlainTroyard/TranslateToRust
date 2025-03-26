use std::io::{self, BufRead};

// Stack implementation
struct Stack {
    data: Vec<i32>,
}

impl Stack {
    // Create a new stack with given capacity
    fn new(capacity: usize) -> Self {
        Stack {
            data: Vec::with_capacity(capacity),
        }
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
    fn pop(&mut self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data.pop().unwrap()
        }
    }

    // Get the top element without removing it
    fn top(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            *self.data.last().unwrap()
        }
    }
}

// Calculate combination count helper function
fn count(m: i32, k: i32) -> i64 {
    if m > k {
        ((m * 2 - k + 1) as i64 * k as i64) / 2
    } else {
        ((m + 1) as i64 * m as i64) / 2
    }
}

// Calculate sum of minimum values in sub-arrays
fn sum_subarray_mins(nums: &[i32], k: i32) -> i64 {
    let mut res = 0;
    let mut stack = Stack::new(nums.len() + 1);
    
    stack.push(-1); // Add sentinel element
    
    for r in 0..nums.len() {
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
    // Create a temporary array with one extra element
    let mut temp = Vec::with_capacity(nums.len() + 1);
    temp.extend_from_slice(nums);
    temp.push(i32::MIN / 2); // Add a sentinel value
    
    // Calculate sum of minimums
    let ans = sum_subarray_mins(&temp, k);
    
    // Negate all elements and recalculate
    for i in 0..temp.len() {
        temp[i] = -temp[i];
    }
    temp[temp.len() - 1] = -temp[temp.len() - 1]; // Restore the sign of sentinel
    
    // Subtract the negated result
    ans - sum_subarray_mins(&temp, k)
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    
    // Allocate memory and read array
    let mut nums = Vec::with_capacity(n);
    for i in 0..n {
        match lines.next() {
            Some(Ok(line)) => {
                match line.trim().parse::<i32>() {
                    Ok(num) => nums.push(num),
                    Err(_) => {
                        eprintln!("Error reading input for nums[{}]", i);
                        return;
                    }
                }
            },
            _ => {
                eprintln!("Error reading input for nums[{}]", i);
                return;
            }
        }
    }
    
    // Call function to calculate result
    let result = min_max_subarray_sum(&nums, k);
    
    // Output result
    println!("{}", result);
}