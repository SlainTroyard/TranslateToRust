use std::io::{self, Write};

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
        (m * 2 - k + 1) * k / 2
    } else {
        (m + 1) * m / 2
    }
}

// Function to calculate the sum of subarray minimums
fn sum_subarray_mins(nums: &[i32], k: i32) -> i64 {
    let mut res = 0;
    let mut stack = Stack::new(nums.len() + 1);
    stack.push(-1); // Add sentinel element

    for r in 0..nums.len() {
        while let Some(&top) = stack.top() {
            if top == -1 || nums[top as usize] < nums[r] {
                break;
            }
            let i = stack.pop().unwrap();
            let l = *stack.top().unwrap();
            let cnt = count((r - l - 1) as i32, k) - count((i - l - 1) as i32, k) - count((r - i - 1) as i32, k);
            res += (nums[i as usize] as i64) * cnt;
        }
        stack.push(r as i32);
    }

    res
}

// Main function to calculate the min-max subarray sum
fn min_max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    let mut temp = nums.to_vec();
    temp.push(i32::MIN / 2); // Add a very small value as a sentinel

    let ans = sum_subarray_mins(&temp, k);

    for num in &mut temp {
        *num = -*num;
    }

    ans - sum_subarray_mins(&temp, k)
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().and_then(|x| x.parse().ok()).expect("Invalid input for n");
    let k: i32 = iter.next().and_then(|x| x.parse().ok()).expect("Invalid input for k");

    let mut nums = vec![0; n];
    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        nums[i] = input.trim().parse().expect("Invalid input for nums");
    }

    // Call the function to calculate the result
    let result = min_max_subarray_sum(&nums, k);

    // Output the result
    println!("{}", result);
}