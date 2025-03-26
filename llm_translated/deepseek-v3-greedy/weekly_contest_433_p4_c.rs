use std::io::{self, Write};
use std::cmp;
use std::mem;

// Stack implementation
struct Stack {
    data: Vec<i32>,
    top: i32,
    capacity: i32,
}

impl Stack {
    // Create a new stack with given capacity
    fn new(capacity: i32) -> Self {
        Stack {
            data: vec![0; capacity as usize],
            top: -1,
            capacity,
        }
    }

    // Check if the stack is empty
    fn is_empty(&self) -> bool {
        self.top == -1
    }

    // Get the size of the stack
    fn size(&self) -> i32 {
        self.top + 1
    }

    // Push a value onto the stack
    fn push(&mut self, value: i32) {
        if self.top == self.capacity - 1 {
            return; // Stack is full
        }
        self.top += 1;
        self.data[self.top as usize] = value;
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

    // Get the top value of the stack
    fn top(&self) -> i32 {
        if self.is_empty() {
            return -1; // Stack is empty
        }
        self.data[self.top as usize]
    }
}

// Helper function to calculate the number of combinations
fn count(m: i32, k: i32) -> i64 {
    if m > k {
        (m * 2 - k + 1) as i64 * k as i64 / 2
    } else {
        (m + 1) as i64 * m as i64 / 2
    }
}

// Function to calculate the sum of subarray minimums
fn sum_subarray_mins(nums: &[i32], k: i32) -> i64 {
    let mut res: i64 = 0;
    let mut stack = Stack::new(nums.len() as i32 + 1);

    stack.push(-1); // Add sentinel element

    for r in 0..nums.len() {
        while stack.size() > 1 && nums[stack.top() as usize] >= nums[r] {
            let i = stack.pop();
            let l = stack.top();
            let cnt = count((r as i32) - l - 1, k) - count(i - l - 1, k) - count((r as i32) - i - 1, k);
            res += nums[i as usize] as i64 * cnt;
        }
        stack.push(r as i32);
    }

    res
}

// Main function to calculate the min-max subarray sum
fn min_max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    // Create a temporary array with an additional element
    let mut temp = vec![0; nums.len() + 1];
    temp[..nums.len()].copy_from_slice(nums);
    temp[nums.len()] = i32::MIN / 2; // Add a very small value as sentinel

    // Calculate the sum of subarray minimums
    let mut ans = sum_subarray_mins(&temp, k);

    // Negate all elements and calculate again
    for i in 0..temp.len() {
        temp[i] = -temp[i];
    }
    temp[nums.len()] = -temp[nums.len()]; // Restore the sentinel element's sign

    // Subtract the negated result from the total sum
    ans -= sum_subarray_mins(&temp, k);

    ans
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input for n");
    let k: i32 = iter.next().unwrap().parse().expect("Invalid input for k");

    // Read the array
    let mut nums = vec![0; n];
    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        nums[i] = input.trim().parse().expect("Invalid input for nums[i]");
    }

    // Calculate the result
    let result = min_max_subarray_sum(&nums, k);

    // Output the result
    println!("{}", result);
}