use std::io::{self, BufRead, Write};

// Stack implementation
struct Stack {
    data: Vec<i32>,
}

impl Stack {
    // Create a new stack with a given capacity
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
        if self.size() < self.data.capacity() {
            self.data.push(value);
        }
    }

    // Pop a value from the stack
    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    // Peek at the top value of the stack
    fn top(&self) -> Option<&i32> {
        self.data.last()
    }
}

// Helper function to calculate the count
fn count(m: i32, k: i32) -> i64 {
    if m > k {
        (m * 2 - k + 1) as i64 * k as i64 / 2
    } else {
        (m + 1) as i64 * m as i64 / 2
    }
}

// Calculate the sum of subarray minimums
fn sum_subarray_mins(nums: &[i32], k: i32) -> i64 {
    let mut res = 0;
    let mut stack = Stack::new(nums.len() + 1);
    stack.push(-1); // Add sentinel element

    for r in 0..nums.len() {
        while let Some(&i) = stack.top() {
            if i == -1 || nums[i as usize] < nums[r] {
                break;
            }
            let l = *stack.pop().unwrap();
            let cnt = count(r as i32 - l as i32 - 1, k)
                - count(i as i32 - l as i32 - 1, k)
                - count(r as i32 - i as i32 - 1, k);
            res += nums[i as usize] as i64 * cnt;
        }
        stack.push(r as i32);
    }

    res
}

// Main function to calculate the min-max subarray sum
fn min_max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    let mut temp = nums.to_vec();
    temp.push(i32::MIN / 2); // Add a very small value as sentinel

    let ans = sum_subarray_mins(&temp, k);

    // Invert all elements
    for num in &mut temp {
        *num = -*num;
    }

    // Subtract the result of the inverted array
    ans - sum_subarray_mins(&temp, k)
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read input
    stdin.lock().read_line(&mut buffer).unwrap();
    let mut iter = buffer.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    buffer.clear();
    stdin.lock().read_line(&mut buffer).unwrap();
    let nums: Vec<i32> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Call the function and print the result
    let result = min_max_subarray_sum(&nums, k);
    writeln!(stdout, "{}", result).unwrap();
}